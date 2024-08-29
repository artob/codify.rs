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
    Dynamic,
    /// See: https://ruby-doc.org/3.3.0/NilClass.html
    NilClass,
    /// See: https://ruby-doc.org/3.3.0/TrueClass.html
    /// See: https://thoughtbot.com/blog/what-is-a-boolean
    Boolean,
    /// See: https://ruby-doc.org/3.3.0/Float.html
    Float,
    /// See: https://ruby-doc.org/3.3.0/Integer.html
    Integer,
    /// See: https://ruby-doc.org/3.3.0/String.html
    String,
    /// See: https://ruby-doc.org/3.3.0/Symbol.html
    Symbol,
    /// See: https://ruby-doc.org/3.3.0/Array.html
    Array(Box<Type>),
    /// See: https://ruby-doc.org/3.3.0/Hash.html
    Hash(Box<Type>, Box<Type>),
    /// See: https://ruby-doc.org/3.3.0/Range.html
    Range,
    Other(String),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "Object" => Dynamic,
            "NilClass" => NilClass,
            "Boolean" | "TrueClass" | "FalseClass" => Boolean,
            "Float" => Float,
            "Integer" => Integer,
            "String" => String,
            "Symbol" => Symbol,
            "Array" => Array(Box::new(Type::Dynamic)),
            "Hash" => Hash(Box::new(Type::Dynamic), Box::new(Type::Dynamic)),
            "Range" => Range,
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
            Dynamic => write!(f, "Object"),
            NilClass => write!(f, "NilClass"),
            Boolean => write!(f, "Boolean"),
            Float => write!(f, "Float"),
            Integer => write!(f, "Integer"),
            String => write!(f, "String"),
            Symbol => write!(f, "Symbol"),
            Array(t) => write!(f, "Array<{}>", t),
            Hash(k, v) => write!(f, "Hash{{{} => {}}}", k, v),
            Range => write!(f, "Range"),
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
            Dynamic => return None, //rust::Type::Any,
            NilClass => rust::Type::Unit,
            Boolean => rust::Type::Bool,
            Float => rust::Type::F64,
            Integer => rust::Type::I64, // TODO: what is the best choice here?
            String => rust::Type::String,
            Symbol => return None,
            Array(t) => todo!(),   //rust::Type::Array(Box::new(t.to_rust())),
            Hash(k, v) => todo!(), //rust::Type::Map(Box::new(k.to_rust()), Box::new(v.to_rust())),
            Range => todo!(),      //rust::Type::Range,
            Other(_) => return None,
        })
    }
}

impl crate::Type for Type {}
