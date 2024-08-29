// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, String},
    rust,
};
use itertools::Itertools;

/// See: https://en.wikibooks.org/wiki/Ruby_Programming/Data_types
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://ruby-doc.org/3.3.0/Object.html
    Object,
    /// See: https://ruby-doc.org/3.3.0/NilClass.html
    NilClass,
    /// See: https://ruby-doc.org/3.3.0/TrueClass.html
    /// See: https://thoughtbot.com/blog/what-is-a-boolean
    Boolean,
    /// See: https://ruby-doc.org/3.3.0/Float.html
    Float,
    /// See: https://ruby-doc.org/3.3.0/Integer.html
    Integer,
    /// See: https://ruby-doc.org/3.3.0/Range.html
    Range,
    /// See: https://ruby-doc.org/3.3.0/String.html
    String,
    /// See: https://ruby-doc.org/3.3.0/Symbol.html
    Symbol,
    /// See: https://ruby-doc.org/3.3.0/Array.html
    Array(Box<Type>),
    /// See: https://ruby-doc.org/3.3.0/Hash.html
    Hash(Box<Type>, Box<Type>),
    Other(String),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "Object" => Object,
            "NilClass" => NilClass,
            "Boolean" | "TrueClass" | "FalseClass" => Boolean,
            "Float" => Float,
            "Integer" => Integer,
            "Range" => Range,
            "String" => String,
            "Symbol" => Symbol,
            "Array" => Array(Box::new(Type::Object)),
            "Hash" => Hash(Box::new(Type::Object), Box::new(Type::Object)),
            _ => {
                if input.starts_with("Array<") {
                    let inner = input
                        .trim_start_matches("Array<")
                        .trim_end_matches('>')
                        .parse()?;
                    Array(Box::new(inner))
                } else if input.starts_with("Hash{") {
                    let (k, v) = input
                        .trim_start_matches("Hash{")
                        .trim_end_matches('}')
                        .split(" => ")
                        .collect_tuple()
                        .ok_or(())?;
                    Hash(Box::new(k.parse()?), Box::new(v.parse()?))
                } else {
                    Other(input.into())
                }
            }
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Object => write!(f, "Object"),
            NilClass => write!(f, "NilClass"),
            Boolean => write!(f, "Boolean"),
            Float => write!(f, "Float"),
            Integer => write!(f, "Integer"),
            Range => write!(f, "Range"),
            String => write!(f, "String"),
            Symbol => write!(f, "Symbol"),
            Array(t) => write!(f, "Array<{}>", t),
            Hash(k, v) => write!(f, "Hash{{{} => {}}}", k, v),
            Other(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Bool => Boolean,
            rust::Type::F32 | rust::Type::F64 => Float,
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Object => rust::Type::Any,
            NilClass => rust::Type::Unit,
            Boolean => rust::Type::Bool,
            Float => rust::Type::F64,
            Integer => rust::Type::I64, // TODO: what is the best choice here?
            Range => rust::Type::Range(Box::new(rust::Type::I64)),
            String => rust::Type::String,
            Symbol => return None, // no equivalent in Rust
            Array(t) => return t.to_rust().map(|t| rust::Type::Vec(Box::new(t))),
            Hash(k, v) => {
                return k.to_rust().and_then(|k| {
                    v.to_rust()
                        .and_then(|v| Some(rust::Type::Map(Box::new(k), Box::new(v))))
                })
            }
            Other(_) => return None, // unknown equivalent in Rust
        })
    }
}

impl crate::Type for Type {}
