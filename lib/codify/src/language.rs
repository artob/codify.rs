// This is free and unencumbered software released into the public domain.

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
