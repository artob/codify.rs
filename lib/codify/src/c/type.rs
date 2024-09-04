// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, String},
    rust,
};

/// See: https://en.cppreference.com/w/c/language/arithmetic_types
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Void,

    /// See: https://en.cppreference.com/w/c/types/boolean
    Bool,

    /// See: https://en.wikipedia.org/wiki/Single-precision_floating-point_format
    Float,

    /// See: https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    Double,

    Char,

    SChar,

    Short,

    Int,

    Long,

    LongLong,

    #[allow(non_camel_case_types)]
    SSize_t,

    UChar,

    UShort,

    UInt,

    ULong,

    ULongLong,

    #[allow(non_camel_case_types)]
    Size_t,

    Array(Box<Type>, Option<usize>),

    Ptr(Box<Type>),

    PtrMut(Box<Type>),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "void" => Void,
            "bool" => Bool,
            "float" => Float,
            "double" => Double,
            "char" => Char,
            "signed char" => SChar,
            "short" | "short int" | "signed short" | "signed short int" => Short,
            "int" | "signed" | "signed int" => Int,
            "long" | "long int" | "signed long" | "signed long int" => Long,
            "long long" | "long long int" | "signed long long" | "signed long long int" => LongLong,
            "ssize_t" => SSize_t,
            "unsigned char" => UChar,
            "unsigned short" | "unsigned short int" => UShort,
            "unsigned int" | "unsigned" => UInt,
            "unsigned long" | "unsigned long int" => ULong,
            "unsigned long long" | "unsigned long long int" => ULongLong,
            "size_t" => Size_t,
            _ if input.ends_with("[]") => {
                let input = input.trim_end_matches("[]");
                Array(Box::new(input.parse()?), None)
            }
            _ if input.ends_with(']') && input.contains('[') => {
                let input = input.trim_end_matches(']');
                let mut parts = input.split('[');
                let t = parts.next().unwrap().parse()?;
                let n = parts.next().unwrap().parse().ok();
                Array(Box::new(t), n)
            }
            _ if input.ends_with('*') && input.starts_with("const ") => {
                Ptr(Box::new(input[6..input.len() - 1].trim().parse()?))
            }
            _ if input.ends_with('*') => PtrMut(Box::new(input[..input.len() - 1].trim().parse()?)),
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Void => write!(f, "void"),
            Bool => write!(f, "bool"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
            Char => write!(f, "char"),
            SChar => write!(f, "signed char"),
            Short => write!(f, "short"),
            Int => write!(f, "int"),
            Long => write!(f, "long"),
            LongLong => write!(f, "long long"),
            SSize_t => write!(f, "ssize_t"),
            UChar => write!(f, "unsigned char"),
            UShort => write!(f, "unsigned short"),
            UInt => write!(f, "unsigned int"),
            ULong => write!(f, "unsigned long"),
            ULongLong => write!(f, "unsigned long long"),
            Size_t => write!(f, "size_t"),
            Array(t, None) => write!(f, "{}[]", t),
            Array(t, Some(n)) => write!(f, "{}[{}]", t, n),
            Ptr(t) => write!(f, "const {}*", t),
            PtrMut(t) => write!(f, "{}*", t),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Any => return Err(()),
            rust::Type::Unit => Void,
            rust::Type::Bool => Bool,
            rust::Type::F32 => Float,
            rust::Type::F64 => Double,
            rust::Type::U8 => UChar,
            rust::Type::U16 => UShort,
            rust::Type::U32 => ULong,
            rust::Type::U64 => ULongLong,
            rust::Type::U128 => return Err(()),
            rust::Type::Usize => return Err(()),
            rust::Type::I8 => SChar,
            rust::Type::I16 => Short,
            rust::Type::I32 => Long,
            rust::Type::I64 => LongLong,
            rust::Type::I128 => return Err(()),
            rust::Type::Isize => return Err(()),
            rust::Type::Range(_) => return Err(()),
            rust::Type::Char => Char,
            rust::Type::Str => Ptr(Box::new(Char)),
            rust::Type::String => return Err(()),
            rust::Type::Box(t) => return Self::try_from(*t).map(|t| Ptr(Box::new(t))),
            rust::Type::Vec(t) => return Self::try_from(*t).map(|t| Ptr(Box::new(t))),
            rust::Type::Map(_, _) => return Err(()),
            rust::Type::Ref(t) => return Self::try_from(*t).map(|t| Ptr(Box::new(t))),
            rust::Type::RefMut(t) => return Self::try_from(*t).map(|t| PtrMut(Box::new(t))),
            rust::Type::Ptr(t) => return Self::try_from(*t).map(|t| Ptr(Box::new(t))),
            rust::Type::PtrMut(t) => return Self::try_from(*t).map(|t| PtrMut(Box::new(t))),
            rust::Type::Ffi(t) => t,
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Void => rust::Type::Unit,
            Bool => rust::Type::Bool,
            t => rust::Type::Ffi(t.clone()),
        })
    }
}

impl crate::Type for Type {}
