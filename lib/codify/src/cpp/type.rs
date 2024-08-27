// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://en.cppreference.com/w/cpp/language/types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://en.cppreference.com/w/cpp/keyword/bool
    Bool,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "bool" => Bool,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "bool"),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Bool,
            _ => return Err(()),
        })
    }
}

impl crate::Type for Type {
    fn to_rust(&self) -> rust::Type {
        use Type::*;
        match self {
            Bool => rust::Type::Bool,
        }
    }
}
