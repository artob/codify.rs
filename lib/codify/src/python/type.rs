// This is free and unencumbered software released into the public domain.

/// See: https://docs.python.org/3/library/stdtypes.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://docs.python.org/3/library/stdtypes.html#boolean-type-bool
    Bool,
}
