// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use codify::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

#[doc(hidden)]
mod prelude;

mod feature;
pub use feature::*;

mod language;
pub use language::*;

mod r#type;
pub use r#type::*;

#[cfg(feature = "language-c")]
/// Support for the C programming language.
pub mod c {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-cpp")]
/// Support for the C++ programming language.
pub mod cpp {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-csharp")]
/// Support for the C# programming language.
pub mod csharp {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-dart")]
/// Support for the Dart programming language.
pub mod dart {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-go")]
/// Support for the Go programming language.
pub mod go {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-java")]
/// Support for the Java programming language.
pub mod java {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-javascript")]
/// Support for the JavaScript programming language.
pub mod javascript {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-python")]
/// Support for the Python programming language.
pub mod python {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-ruby")]
/// Support for the Ruby programming language.
pub mod ruby {
    pub mod r#type;
    pub use r#type::*;
}

/// Support for the Rust programming language.
pub mod rust {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-swift")]
/// Support for the Swift programming language.
pub mod swift {
    pub mod r#type;
    pub use r#type::*;
}

#[cfg(feature = "language-typescript")]
/// Support for the TypeScript programming language.
pub mod typescript {
    pub mod r#type;
    pub use r#type::*;
}

//#[doc = include_str!("../../../README.md")]
//#[cfg(doctest)]
//pub struct ReadmeDoctests;
