// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Box, Cow, Named},
    rust,
};

/// See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#any
    Any,

    /// See: https://developer.mozilla.org/en-US/docs/Glossary/Undefined
    Undefined,

    /// See: https://developer.mozilla.org/en-US/docs/Glossary/Boolean
    Boolean,

    /// See: https://developer.mozilla.org/en-US/docs/Glossary/Number
    Number,

    /// See: https://developer.mozilla.org/en-US/docs/Glossary/BigInt
    BigInt,

    /// See: https://developer.mozilla.org/en-US/docs/Glossary/String
    String,

    /// See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
    Array(Box<Type>),

    /// See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map
    Map(Box<Type>, Box<Type>),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;

        let input_len = input.len();
        if input_len == 0 {
            return Err(());
        }

        let last_char = input.chars().last().unwrap();
        match last_char {
            ']' if input_len > 2 => {
                return Ok(Array(Box::new(input[0..input_len - 2].trim().parse()?)))
            }
            _ => (),
        }

        Ok(match input {
            "Any" | "any" => Any,
            "Undefined" | "undefined" => Undefined,
            "Boolean" | "boolean" => Boolean,
            "Number" | "number" => Number,
            "BigInt" | "bigint" => BigInt,
            "String" | "string" => String,
            _ => {
                if last_char == '>' {
                    if let Some(start_idx) = input.find('<') {
                        return Ok(match input[..start_idx].trim() {
                            "Array" if input_len > 8 => Array(Box::new(
                                input[start_idx + 1..input_len - 1].trim().parse()?,
                            )),
                            "Map" if input_len > 6 => {
                                let comma_idx = input.find(',').ok_or(())?;

                                Map(
                                    Box::new(input[start_idx + 1..comma_idx].trim().parse()?),
                                    Box::new(input[comma_idx + 1..input_len - 1].trim().parse()?),
                                )
                            }
                            _ => return Err(()),
                        });
                    }
                }

                return Err(());
            }
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Any => write!(f, "any"),
            Undefined => write!(f, "undefined"),
            Boolean => write!(f, "boolean"),
            Number => write!(f, "number"),
            BigInt => write!(f, "bigint"),
            String => write!(f, "string"),
            Array(t) => write!(f, "Array<{}>", t),
            Map(k, v) => write!(f, "Map<{}, {}>", k, v),
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
            rust::Type::Any => Any,
            rust::Type::Unit => return Err(()),
            rust::Type::Bool => Boolean,
            rust::Type::F32 | rust::Type::F64 => Number,
            rust::Type::I8
            | rust::Type::U8
            | rust::Type::I16
            | rust::Type::U16
            | rust::Type::I32
            | rust::Type::U32
            | rust::Type::I64
            | rust::Type::U64
            | rust::Type::Isize
            | rust::Type::Usize => Number,
            rust::Type::I128 | rust::Type::U128 => BigInt,
            rust::Type::Str => String,
            rust::Type::String => String,
            rust::Type::Box(t) => Self::try_from(*t)?,
            rust::Type::Vec(t) => Array(Box::new(Self::try_from(*t)?)),
            rust::Type::Map(k, v) => {
                Map(Box::new(Self::try_from(*k)?), Box::new(Self::try_from(*v)?))
            }
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Any => rust::Type::Any,
            Undefined => return None,
            Boolean => rust::Type::Bool,
            Number => unimplemented!(),
            BigInt => rust::Type::I128,
            String => rust::Type::String,
            Array(t) => rust::Type::Vec(Box::new(t.to_rust()?)),
            Map(k, v) => rust::Type::Map(Box::new(k.to_rust()?), Box::new(v.to_rust()?)),
        })
    }
}

impl crate::Type for Type {}
