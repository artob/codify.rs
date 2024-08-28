// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://en.wikibooks.org/wiki/Ruby_Programming/Data_types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://thoughtbot.com/blog/what-is-a-boolean
    Boolean,
    /// See: https://en.wikibooks.org/wiki/Ruby_Programming/Data_types
    Float,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "Boolean" | "TrueClass" | "FalseClass" => Boolean,
            "Float" => Float,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Boolean => write!(f, "Boolean"),
            Float => write!(f, "Float"),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Boolean,
            rust::Type::F32 | rust::Type::F64 => Float,
            _ => return Err(()),
        })
    }
}

impl crate::Type for Type {
    fn to_rust(&self) -> rust::Type {
        use Type::*;
        match self {
            Boolean => rust::Type::Bool,
            Float => rust::Type::F64,
        }
    }
}
