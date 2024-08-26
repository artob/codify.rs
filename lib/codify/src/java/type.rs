// This is free and unencumbered software released into the public domain.

/// See: https://docs.oracle.com/javase%2Ftutorial%2F/java/nutsandbolts/datatypes.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://docs.oracle.com/javase/8/docs/api/java/lang/Boolean.html
    Boolean,
}
