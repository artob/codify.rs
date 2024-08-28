// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://doc.rust-lang.org/reference/types.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://doc.rust-lang.org/reference/types/boolean.html
    Bool,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types
    F32,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types
    F64,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "bool" => Bool,
            "f32" => F32,
            "f64" => F64,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Bool => write!(f, "bool"),
            F32 => write!(f, "f32"),
            F64 => write!(f, "f64"),
        }
    }
}

//impl TryFrom<rust::Type> for Type {}

impl crate::Type for Type {
    fn to_rust(&self) -> rust::Type {
        *self
    }
}
