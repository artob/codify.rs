// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Box, Cow, Named, Vec},
    rust,
};

/// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/value-types
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/void
    Void,

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/reference-types#the-object-type
    Object,

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/bool
    Bool,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.single
    Float,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.double
    Double,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.decimal
    Decimal,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.sbyte
    SByte,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.byte
    Byte,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.int16
    Short,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.uint16
    UShort,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.int32
    Int,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.uint32
    UInt,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.int64
    Long,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.uint64
    ULong,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.int128
    Int128,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.uint128
    UInt128,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.intptr
    NInt,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.uintptr
    NUInt,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.range
    Range,

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/char
    Char,

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/reference-types#the-string-type
    String,

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.array
    /// NOTE: We don't support multi-dimensional arrays.
    Array(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.list-1
    List(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.dictionary-2
    Dictionary(Box<Type>, Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/ref
    RefMut(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/ref
    Ptr(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/ref
    PtrMut(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/out
    Out(Box<Type>),

    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/nullable-value-types
    Nullable(Box<Type>),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;

        let input_len = input.len();
        if input_len == 0 {
            return Err(());
        }

        let last_char = input.chars().last().unwrap();
        match last_char {
            '?' if input_len > 1 => {
                return Ok(Nullable(Box::new(input[0..input_len - 1].trim().parse()?)))
            }
            '*' if input_len > 8 && input.starts_with("const ") => {
                return Ok(Ptr(Box::new(input[6..input_len - 1].trim().parse()?)))
            }
            '*' if input_len > 1 => {
                return Ok(PtrMut(Box::new(input[0..input_len - 1].trim().parse()?)))
            }
            ']' if input_len > 2 => {
                return Ok(Array(Box::new(input[0..input_len - 2].trim().parse()?)))
            }
            _ => (),
        }

        Ok(match input {
            "void" | "Void" => Void,
            "object" | "Object" => Object,
            "bool" | "Bool" => Bool,
            "float" | "Single" => Float,
            "double" | "Double" => Double,
            "decimal" | "Decimal" => Decimal,
            "sbyte" | "SByte" => SByte,
            "byte" | "Byte" => Byte,
            "short" | "Int16" => Short,
            "ushort" | "UInt16" => UShort,
            "int" | "Int32" => Int,
            "uint" | "UInt32" => UInt,
            "long" | "Int64" => Long,
            "ulong" | "UInt64" => ULong,
            "nint" | "IntPtr" => NInt,
            "nuint" | "UIntPtr" => NUInt,
            "char" | "Char" => Char,
            "string" | "String" => String,
            _ => {
                if input.starts_with("System.") {
                    if input.starts_with("System.Collections.Generic.") && last_char == '>' {
                        let start_idx = input.find('<').ok_or(())?;

                        return Ok(match &input[27..start_idx] {
                            "List" if input_len > 34 && last_char == '>' => List(Box::new(
                                input[start_idx + 1..input_len - 1].trim().parse()?,
                            )),
                            "Dictionary" if input_len > 42 => {
                                let comma_idx = input.find(',').ok_or(())?;

                                Dictionary(
                                    Box::new(input[start_idx + 1..comma_idx].trim().parse()?),
                                    Box::new(input[comma_idx + 1..input_len - 1].trim().parse()?),
                                )
                            }
                            _ => return Err(()),
                        });
                    }

                    return input[8..].parse();
                }

                if input_len > 4 {
                    return Ok(match &input[..4] {
                        "ref " => RefMut(Box::new(input[4..].trim().parse()?)),
                        "out " => Out(Box::new(input[4..].trim().parse()?)),
                        _ => return Err(()),
                    });
                }

                return Err(());
            }
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Void => write!(f, "void"),
            Object => write!(f, "object"),
            // Tuple(types) => write!(
            //     f,
            //     "({})",
            //     types
            //         .into_iter()
            //         .map(|t| format!("{}", t))
            //         .collect::<Vec<_>>()
            //         .join(", ")
            // ),
            Bool => write!(f, "bool"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
            Decimal => write!(f, "decimal"),
            SByte => write!(f, "sbyte"),
            Byte => write!(f, "byte"),
            Short => write!(f, "short"),
            UShort => write!(f, "ushort"),
            Int => write!(f, "int"),
            UInt => write!(f, "uint"),
            Long => write!(f, "long"),
            ULong => write!(f, "ulong"),
            Int128 => write!(f, "System.Int128"),
            UInt128 => write!(f, "System.UInt128"),
            NInt => write!(f, "nint"),
            NUInt => write!(f, "nuint"),
            Range => write!(f, "System.Range"),
            Char => write!(f, "char"),
            String => write!(f, "string"),
            Array(t) => write!(f, "{}[]", t),
            List(t) => write!(f, "System.Collections.Generic.List<{}>", t),
            Dictionary(k, v) => write!(f, "System.Collections.Generic.Dictionary<{}, {}>", k, v),
            RefMut(t) => write!(f, "ref {}", t),
            Ptr(t) => write!(f, "const {}*", t),
            PtrMut(t) => write!(f, "{}*", t),
            Out(t) => write!(f, "out {}", t),
            Nullable(t) => write!(f, "{}?", t),
        }
    }
}

impl Named for Type {
    fn name(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self))
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Unit => Void,
            rust::Type::Any => Object,
            rust::Type::Bool => Bool,
            rust::Type::F32 => Float,
            rust::Type::F64 => Double,
            rust::Type::U8 => Byte,
            rust::Type::U16 => UShort,
            rust::Type::U32 => UInt,
            rust::Type::U64 => ULong,
            rust::Type::U128 => UInt128,
            rust::Type::Usize => NUInt,
            rust::Type::I8 => SByte,
            rust::Type::I16 => Short,
            rust::Type::I32 => Int,
            rust::Type::I64 => Long,
            rust::Type::I128 => Int128,
            rust::Type::Isize => NInt,
            rust::Type::Range(_) => Range, // NOTE(yurii): We lose inner type in this case...
            rust::Type::Char => Char,
            rust::Type::Str => String,
            rust::Type::String => String,
            rust::Type::Box(t) => return Self::try_from(*t).map(|t| t),
            rust::Type::Vec(t) => return Self::try_from(*t).map(|t| List(Box::new(t))),
            rust::Type::Map(k, v) => {
                Dictionary(Box::new(Self::try_from(*k)?), Box::new(Self::try_from(*v)?))
            }
            rust::Type::Ref(_) => return Err(()), // References are... weird...
            rust::Type::RefMut(t) => return Self::try_from(*t).map(|t| RefMut(Box::new(t))),
            rust::Type::Ptr(t) => return Self::try_from(*t).map(|t| Ptr(Box::new(t))),
            rust::Type::PtrMut(t) => return Self::try_from(*t).map(|t| PtrMut(Box::new(t))),
            #[cfg(feature = "language-c")]
            rust::Type::Ffi(_) => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Void => rust::Type::Unit,
            Object => rust::Type::Any,
            Bool => rust::Type::Bool,
            Float => rust::Type::F32,
            Double => rust::Type::F64,
            Decimal => return None,
            SByte => rust::Type::I8,
            Byte => rust::Type::U8,
            Short => rust::Type::I16,
            UShort => rust::Type::U16,
            Int => rust::Type::I32,
            UInt => rust::Type::U32,
            Long => rust::Type::I64,
            ULong => rust::Type::U64,
            Int128 => rust::Type::I128,
            UInt128 => rust::Type::U128,
            NInt => rust::Type::Isize,
            NUInt => rust::Type::Usize,
            Range => rust::Type::Range(Box::new(rust::Type::I32)),
            Char => rust::Type::Char,
            String => rust::Type::String,
            Array(t) | List(t) => rust::Type::Vec(Box::new(t.to_rust()?)),
            Dictionary(k, v) => rust::Type::Map(Box::new(k.to_rust()?), Box::new(v.to_rust()?)),
            RefMut(t) | Ptr(t) | PtrMut(t) | Out(t) => rust::Type::Ref(Box::new(t.to_rust()?)),
            Nullable(_) => return None,
        })
    }
}

impl crate::Type for Type {}
