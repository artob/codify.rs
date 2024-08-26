// This is free and unencumbered software released into the public domain.

/// See: https://doc.rust-lang.org/reference/types.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://doc.rust-lang.org/reference/types/boolean.html
    Bool,
}
