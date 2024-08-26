// This is free and unencumbered software released into the public domain.

/// See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Data_structures#boolean_type
    Boolean,
}
