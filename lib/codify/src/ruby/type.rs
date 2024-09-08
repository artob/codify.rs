// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Box, Cow, Named, String},
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

    /// TBD
    Other(String),

    /// See: https://rubygems.org/gems/ffi
    #[cfg(feature = "language-c")]
    Ffi(crate::c::Type),
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            #[cfg(feature = "language-c")]
            Ffi(t) => match t {
                // See: https://github.com/ffi/ffi/wiki/Types
                crate::c::Type::Void => write!(f, ":void"),
                crate::c::Type::Bool => write!(f, ":bool"),
                crate::c::Type::Float => write!(f, ":float"),
                crate::c::Type::Double => write!(f, ":double"),
                crate::c::Type::Char => write!(f, ":char"),
                crate::c::Type::SChar => write!(f, ":char"),
                crate::c::Type::Short => write!(f, ":short"),
                crate::c::Type::Int => write!(f, ":int"),
                crate::c::Type::Long => write!(f, ":long"),
                crate::c::Type::LongLong => write!(f, ":long_long"),
                crate::c::Type::SSize_t => write!(f, ":ssize_t"),
                crate::c::Type::UChar => write!(f, ":uchar"),
                crate::c::Type::UShort => write!(f, ":ushort"),
                crate::c::Type::UInt => write!(f, ":uint"),
                crate::c::Type::ULong => write!(f, ":ulong"),
                crate::c::Type::ULongLong => write!(f, ":ulong_long"),
                crate::c::Type::Size_t => write!(f, ":size_t"),
                crate::c::Type::Array(_t, None) => write!(f, ":pointer"),
                crate::c::Type::Array(t, Some(n)) => write!(f, "[{}, {}]", Ffi((**t).clone()), n),
                crate::c::Type::Ptr(t) if **t == crate::c::Type::Char => write!(f, ":string"),
                crate::c::Type::PtrMut(t) if **t == crate::c::Type::Char => write!(f, ":pointer"),
                crate::c::Type::Ptr(_) | crate::c::Type::PtrMut(_) => write!(f, ":pointer"),
                #[cfg(feature = "libc")]
                crate::c::Type::Time_t => write!(f, ":time_t"),
            },
        }
    }
}

impl Named for Type {
    fn name(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self))
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Any => Object,
            rust::Type::Unit => NilClass,
            rust::Type::Bool => Boolean,
            rust::Type::F32 | rust::Type::F64 => Float,
            rust::Type::I8 | rust::Type::U8 => Integer,
            rust::Type::I16 | rust::Type::U16 => Integer,
            rust::Type::I32 | rust::Type::U32 => Integer,
            rust::Type::I64 | rust::Type::U64 => Integer,
            rust::Type::I128 | rust::Type::U128 => Integer,
            rust::Type::Isize | rust::Type::Usize => Integer,
            rust::Type::Range(_) => Range,
            rust::Type::Char => String,
            rust::Type::Str => String,
            rust::Type::String => String,
            rust::Type::Box(t) => Type::try_from(*t)?,
            rust::Type::Vec(t) => Array(Box::new(Type::try_from(*t)?)),
            rust::Type::Map(k, v) => {
                Hash(Box::new(Type::try_from(*k)?), Box::new(Type::try_from(*v)?))
            }
            rust::Type::Ref(_) => return Err(()),
            rust::Type::RefMut(_) => return Err(()),
            rust::Type::Ptr(_) => return Err(()),
            rust::Type::PtrMut(_) => return Err(()),
            #[cfg(feature = "language-c")]
            rust::Type::Ffi(t) => Type::Ffi(t),
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
            #[cfg(feature = "language-c")]
            Ffi(t) => rust::Type::Ffi(t.clone()),
        })
    }
}

impl crate::Type for Type {}
