// This is free and unencumbered software released into the public domain.

/// See: https://en.wikibooks.org/wiki/Ruby_Programming/Data_types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://thoughtbot.com/blog/what-is-a-boolean
    Boolean,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "Boolean" => Self::Boolean,
            _ => return Err(()),
        })
    }
}
