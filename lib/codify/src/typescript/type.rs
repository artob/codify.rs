// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Cow, Named},
    rust,
};

/// See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Data_structures#boolean_type
    Boolean,

    /// See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html
    Number,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "boolean" => Boolean,
            "number" => Number,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Boolean => write!(f, "boolean"),
            Number => write!(f, "number"),
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
            rust::Type::Bool => Boolean,
            rust::Type::F32 | rust::Type::F64 => Number,
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Boolean => rust::Type::Bool,
            Number => rust::Type::F64,
        })
    }
}

impl crate::Type for Type {}
