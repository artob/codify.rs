// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Cow, Named},
    rust,
};

/// See: https://en.cppreference.com/w/cpp/language/types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://en.cppreference.com/w/cpp/keyword/bool
    Bool,

    /// See: https://en.wikipedia.org/wiki/Single-precision_floating-point_format
    Float,

    /// See: https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    Double,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "bool" => Bool,
            "float" => Float,
            "double" => Double,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "bool"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
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
            rust::Type::Bool => Bool,
            rust::Type::F32 => Float,
            rust::Type::F64 => Double,
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Bool => rust::Type::Bool,
            Float => rust::Type::F32,
            Double => rust::Type::F64,
        })
    }
}

impl crate::Type for Type {}
