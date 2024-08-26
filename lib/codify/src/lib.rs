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

#[cfg(feature = "lang-c")]
pub mod c {
    pub mod r#type;
}

#[cfg(feature = "lang-cpp")]
pub mod cpp {
    pub mod r#type;
}

#[cfg(feature = "lang-csharp")]
pub mod csharp {
    pub mod r#type;
}

#[cfg(feature = "lang-dart")]
pub mod dart {
    pub mod r#type;
}

#[cfg(feature = "lang-go")]
pub mod go {
    pub mod r#type;
}

#[cfg(feature = "lang-java")]
pub mod java {
    pub mod r#type;
}

#[cfg(feature = "lang-javascript")]
pub mod javascript {
    pub mod r#type;
}

#[cfg(feature = "lang-python")]
pub mod python {
    pub mod r#type;
}

#[cfg(feature = "lang-ruby")]
pub mod ruby {
    pub mod r#type;
}

#[cfg(feature = "lang-rust")]
pub mod rust {
    pub mod r#type;
}

#[cfg(feature = "lang-swift")]
pub mod swift {
    pub mod r#type;
}

#[cfg(feature = "lang-typescript")]
pub mod typescript {
    pub mod r#type;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
