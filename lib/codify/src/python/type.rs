// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://docs.python.org/3/library/stdtypes.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://docs.python.org/3/library/stdtypes.html#boolean-type-bool
    Bool,
    /// See: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
    Float,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "bool" => Bool,
            "float" => Float,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "bool"),
            Float => write!(f, "float"),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Bool,
            rust::Type::F32 | rust::Type::F64 => Float,
            _ => return Err(()),
        })
    }
}

impl crate::Type for Type {
    fn to_rust(&self) -> rust::Type {
        use Type::*;
        match self {
            Bool => rust::Type::Bool,
            Float => rust::Type::F64,
        }
    }
}
