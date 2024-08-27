// This is free and unencumbered software released into the public domain.

/// See: https://docs.oracle.com/javase%2Ftutorial%2F/java/nutsandbolts/datatypes.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://docs.oracle.com/javase/8/docs/api/java/lang/Boolean.html
    Boolean,
}

impl crate::Type for Type {}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "boolean" => Boolean,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Boolean => write!(f, "boolean"),
        }
    }
}
