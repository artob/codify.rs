// This is free and unencumbered software released into the public domain.

/// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/value-types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/bool
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
