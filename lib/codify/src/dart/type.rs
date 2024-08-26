// This is free and unencumbered software released into the public domain.

/// See: https://dart.dev/language/built-in-types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://dart.dev/language/built-in-types#booleans
    Bool,
}
