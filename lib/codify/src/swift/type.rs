// This is free and unencumbered software released into the public domain.

/// See: https://developer.apple.com/documentation/swift#Standard-Library
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://developer.apple.com/documentation/swift/bool
    Bool,
}
