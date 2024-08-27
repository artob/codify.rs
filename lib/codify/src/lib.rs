// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use codify::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
mod prelude;

mod feature;
pub use feature::*;

mod language;
pub use language::*;

#[cfg(feature = "language-c")]
pub mod c {
    pub mod r#type;
}

#[cfg(feature = "language-cpp")]
pub mod cpp {
    pub mod r#type;
}

#[cfg(feature = "language-csharp")]
pub mod csharp {
    pub mod r#type;
}

#[cfg(feature = "language-dart")]
pub mod dart {
    pub mod r#type;
}

#[cfg(feature = "language-go")]
pub mod go {
    pub mod r#type;
}

#[cfg(feature = "language-java")]
pub mod java {
    pub mod r#type;
}

#[cfg(feature = "language-javascript")]
pub mod javascript {
    pub mod r#type;
}

#[cfg(feature = "language-python")]
pub mod python {
    pub mod r#type;
}

#[cfg(feature = "language-ruby")]
pub mod ruby {
    pub mod r#type;
}

#[cfg(feature = "language-rust")]
pub mod rust {
    pub mod r#type;
}

#[cfg(feature = "language-swift")]
pub mod swift {
    pub mod r#type;
}

#[cfg(feature = "language-typescript")]
pub mod typescript {
    pub mod r#type;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
