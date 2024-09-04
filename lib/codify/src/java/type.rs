// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://docs.oracle.com/javase%2Ftutorial%2F/java/nutsandbolts/datatypes.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://docs.oracle.com/javase/8/docs/api/java/lang/Boolean.html
    Boolean,

    /// See: https://docs.oracle.com/javase/8/docs/api/java/lang/Float.html
    Float,

    /// See: https://docs.oracle.com/javase/8/docs/api/java/lang/Double.html
    Double,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "boolean" | "Boolean" => Boolean,
            "float" | "Float" => Float,
            "double" | "Double" => Double,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Boolean => write!(f, "boolean"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Boolean,
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
            Boolean => rust::Type::Bool,
            Float => rust::Type::F32,
            Double => rust::Type::F64,
        })
    }
}

impl crate::Type for Type {}
