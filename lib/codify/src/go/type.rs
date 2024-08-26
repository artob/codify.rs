// This is free and unencumbered software released into the public domain.

/// See: https://go.dev/ref/spec#Types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://go.dev/ref/spec#Boolean_types
    Bool,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "bool" => Self::Bool,
            _ => return Err(()),
        })
    }
}
