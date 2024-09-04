// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://developer.apple.com/documentation/swift#Standard-Library
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://developer.apple.com/documentation/swift/bool
    Bool,

    /// See: https://developer.apple.com/documentation/swift/double
    Double,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "Bool" => Bool,
            "Double" => Double,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "Bool"),
            Double => write!(f, "Double"),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Bool,
            rust::Type::F32 | rust::Type::F64 => Double,
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Bool => rust::Type::Bool,
            Double => rust::Type::F64,
        })
    }
}

impl crate::Type for Type {}
