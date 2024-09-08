// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Cow, Named},
    rust,
};

/// See: https://go.dev/ref/spec#Types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://go.dev/ref/spec#Boolean_types
    Bool,

    /// See: https://go.dev/ref/spec#Numeric_types
    Float32,

    /// See: https://go.dev/ref/spec#Numeric_types
    Float64,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "bool" => Bool,
            "float32" => Float32,
            "float64" => Float64,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "bool"),
            Float32 => write!(f, "float32"),
            Float64 => write!(f, "float64"),
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
            rust::Type::F32 => Float32,
            rust::Type::F64 => Float64,
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Bool => rust::Type::Bool,
            Float32 => rust::Type::F32,
            Float64 => rust::Type::F64,
        })
    }
}

impl crate::Type for Type {}
