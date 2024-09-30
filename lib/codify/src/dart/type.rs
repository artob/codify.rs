// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Cow, Named},
    rust,
};

/// See: https://dart.dev/language/built-in-types
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://dart.dev/language/built-in-types
    Void,

    /// See: https://api.dart.dev/stable/dart-core/Object-class.html
    Object,

    /// See: https://api.dart.dev/stable/dart-core/bool-class.html
    Bool,

    /// See: https://api.dart.dev/stable/dart-core/double-class.html
    Double,

    /// See: https://api.dart.dev/stable/dart-core/int-class.html
    Int,

    /// See: https://api.dart.dev/stable/dart-core/BigInt-class.html
    BigInt,

    /// See: https://api.dart.dev/stable/dart-core/String-class.html
    String,

    /// See: https://api.dart.dev/stable/dart-core/List-class.html
    List(Box<Type>),

    /// See: https://api.dart.dev/stable/dart-core/Map-class.html
    Map(Box<Type>, Box<Type>),

    /// See: https://dart.dev/null-safety/understanding-null-safety
    Nullable(Box<Type>),
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
            '?' if input_len > 1 => {
                return Ok(Nullable(Box::new(input[0..input_len - 1].trim().parse()?)))
            }
            _ => (),
        }

        Ok(match input {
            "void" => Void,
            "Object" => Object,
            "bool" => Bool,
            "double" => Double,
            "int" => Int,
            "BigInt" => BigInt,
            "String" => String,
            _ => {
                if last_char == '>' {
                    if let Some(start_idx) = input.find('<') {
                        return Ok(match input[..start_idx].trim() {
                            "List" if input_len > 7 => List(Box::new(
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
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Void => write!(f, "void"),
            Object => write!(f, "Object"),
            Bool => write!(f, "bool"),
            Double => write!(f, "double"),
            Int => write!(f, "int"),
            BigInt => write!(f, "BigInt"),
            String => write!(f, "String"),
            List(t) => write!(f, "List<{}>", t),
            Map(k, v) => write!(f, "Map<{}, {}>", k, v),
            Nullable(t) => write!(f, "{}?", t),
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
            rust::Type::Unit => Void,
            rust::Type::Any => Object,
            rust::Type::Bool => Bool,
            rust::Type::F32 | rust::Type::F64 => Double,
            rust::Type::I8
            | rust::Type::U8
            | rust::Type::I16
            | rust::Type::U16
            | rust::Type::I32
            | rust::Type::U32
            | rust::Type::I64
            | rust::Type::U64
            | rust::Type::Isize
            | rust::Type::Usize => Int,
            rust::Type::I128 | rust::Type::U128 => BigInt,
            rust::Type::Range(_) => return Err(()),
            rust::Type::Char => String,
            rust::Type::Str => String,
            rust::Type::String => String,
            rust::Type::Box(t) => Self::try_from(*t)?,
            rust::Type::Vec(t) => Self::try_from(*t).map(|t| List(Box::new(t)))?,
            rust::Type::Map(k, v) => {
                Map(Box::new(Self::try_from(*k)?), Box::new(Self::try_from(*v)?))
            }
            rust::Type::Ref(t) => return Err(()),
            rust::Type::RefMut(t) => Self::try_from(*t)?,
            rust::Type::Ptr(t) => return Err(()),
            rust::Type::PtrMut(t) => return Err(()),
            #[cfg(feature = "language-c")]
            rust::Type::Ffi(_) => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            Void => rust::Type::Unit,
            Object => rust::Type::Any,
            Bool => rust::Type::Bool,
            Double => rust::Type::F64,
            Int => rust::Type::Isize,
            BigInt => rust::Type::I128,
            String => rust::Type::String,
            List(t) => rust::Type::Vec(Box::new(t.to_rust()?)),
            Map(k, v) => rust::Type::Map(Box::new(k.to_rust()?), Box::new(v.to_rust()?)),
            Nullable(t) => return None,
        })
    }
}

impl crate::Type for Type {}
