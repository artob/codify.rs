// This is free and unencumbered software released into the public domain.

/// See: https://en.cppreference.com/w/c/language/arithmetic_types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://en.cppreference.com/w/c/types/boolean
    Bool,
}
