// This is free and unencumbered software released into the public domain.

use super::Type;
use crate::{
    prelude::{Box, FromStr},
    rust,
};

/// A programming language.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    Rust,

    #[cfg(feature = "language-c")]
    C,

    #[cfg(feature = "language-cpp")]
    Cpp,

    #[cfg(feature = "language-csharp")]
    Csharp,

    #[cfg(feature = "language-dart")]
    Dart,

    #[cfg(feature = "language-go")]
    Go,

    #[cfg(feature = "language-java")]
    Java,

    #[cfg(feature = "language-javascript")]
    JavaScript,

    #[cfg(feature = "language-python")]
    Python,

    #[cfg(feature = "language-ruby")]
    Ruby,

    #[cfg(feature = "language-swift")]
    Swift,

    #[cfg(feature = "language-typescript")]
    TypeScript,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        use Language::*;
        match self {
            Rust => "rust",

            #[cfg(feature = "language-c")]
            C => "c",

            #[cfg(feature = "language-cpp")]
            Cpp => "cpp",

            #[cfg(feature = "language-csharp")]
            Csharp => "csharp",

            #[cfg(feature = "language-dart")]
            Dart => "dart",

            #[cfg(feature = "language-go")]
            Go => "go",

            #[cfg(feature = "language-java")]
            Java => "java",

            #[cfg(feature = "language-javascript")]
            JavaScript => "javascript",

            #[cfg(feature = "language-python")]
            Python => "python",

            #[cfg(feature = "language-ruby")]
            Ruby => "ruby",

            #[cfg(feature = "language-swift")]
            Swift => "swift",

            #[cfg(feature = "language-typescript")]
            TypeScript => "typescript",
        }
    }

    pub fn parse_type(&self, input: &str) -> Result<Box<dyn Type>, ()> {
        use Language::*;
        Ok(match self {
            Rust => Box::new(crate::rust::Type::from_str(input)?),

            #[cfg(feature = "language-c")]
            C => Box::new(crate::c::Type::from_str(input)?),

            #[cfg(feature = "language-cpp")]
            Cpp => Box::new(crate::cpp::Type::from_str(input)?),

            #[cfg(feature = "language-csharp")]
            Csharp => Box::new(crate::csharp::Type::from_str(input)?),

            #[cfg(feature = "language-dart")]
            Dart => Box::new(crate::dart::Type::from_str(input)?),

            #[cfg(feature = "language-go")]
            Go => Box::new(crate::go::Type::from_str(input)?),

            #[cfg(feature = "language-java")]
            Java => Box::new(crate::java::Type::from_str(input)?),

            #[cfg(feature = "language-javascript")]
            JavaScript => Box::new(crate::javascript::Type::from_str(input)?),

            #[cfg(feature = "language-python")]
            Python => Box::new(crate::python::Type::from_str(input)?),

            #[cfg(feature = "language-ruby")]
            Ruby => Box::new(crate::ruby::Type::from_str(input)?),

            #[cfg(feature = "language-swift")]
            Swift => Box::new(crate::swift::Type::from_str(input)?),

            #[cfg(feature = "language-typescript")]
            TypeScript => Box::new(crate::typescript::Type::from_str(input)?),

            _ => return Err(()),
        })
    }

    pub fn from_type(&self, input: rust::Type) -> Result<Box<dyn Type>, ()> {
        use Language::*;
        Ok(match self {
            Rust => Box::new(input),

            #[cfg(feature = "language-c")]
            C => Box::new(crate::c::Type::try_from(input)?),

            #[cfg(feature = "language-cpp")]
            Cpp => Box::new(crate::cpp::Type::try_from(input)?),

            #[cfg(feature = "language-csharp")]
            Csharp => Box::new(crate::csharp::Type::try_from(input)?),

            #[cfg(feature = "language-dart")]
            Dart => Box::new(crate::dart::Type::try_from(input)?),

            #[cfg(feature = "language-go")]
            Go => Box::new(crate::go::Type::try_from(input)?),

            #[cfg(feature = "language-java")]
            Java => Box::new(crate::java::Type::try_from(input)?),

            #[cfg(feature = "language-javascript")]
            JavaScript => Box::new(crate::javascript::Type::try_from(input)?),

            #[cfg(feature = "language-python")]
            Python => Box::new(crate::python::Type::try_from(input)?),

            #[cfg(feature = "language-ruby")]
            Ruby => Box::new(crate::ruby::Type::try_from(input)?),

            #[cfg(feature = "language-swift")]
            Swift => Box::new(crate::swift::Type::try_from(input)?),

            #[cfg(feature = "language-typescript")]
            TypeScript => Box::new(crate::typescript::Type::try_from(input)?),
        })
    }
}

impl core::str::FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Language::*;
        Ok(match input {
            "rust" => Rust,

            #[cfg(feature = "language-c")]
            "c" => C,

            #[cfg(feature = "language-cpp")]
            "cpp" => Cpp,

            #[cfg(feature = "language-csharp")]
            "csharp" => Csharp,

            #[cfg(feature = "language-dart")]
            "dart" => Dart,

            #[cfg(feature = "language-go")]
            "go" => Go,

            #[cfg(feature = "language-java")]
            "java" => Java,

            #[cfg(feature = "language-javascript")]
            "javascript" => JavaScript,

            #[cfg(feature = "language-python")]
            "python" => Python,

            #[cfg(feature = "language-ruby")]
            "ruby" => Ruby,

            #[cfg(feature = "language-swift")]
            "swift" => Swift,

            #[cfg(feature = "language-typescript")]
            "typescript" => TypeScript,

            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
