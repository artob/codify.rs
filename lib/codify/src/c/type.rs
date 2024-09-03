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

    UChar,

    UShort,

    UInt,

    ULong,

    ULongLong,

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
            "unsigned char" => UChar,
            "unsigned short" | "unsigned short int" => UShort,
            "unsigned int" | "unsigned" => UInt,
            "unsigned long" | "unsigned long int" => ULong,
            "unsigned long long" | "unsigned long long int" => ULongLong,
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
            UChar => write!(f, "unsigned char"),
            UShort => write!(f, "unsigned short"),
            UInt => write!(f, "unsigned int"),
            ULong => write!(f, "unsigned long"),
            ULongLong => write!(f, "unsigned long long"),
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
