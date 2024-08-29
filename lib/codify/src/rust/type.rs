// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, String},
    rust,
};

/// See: https://doc.rust-lang.org/reference/types.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://doc.rust-lang.org/reference/types/tuple.html
    Unit,
    /// See: https://doc.rust-lang.org/reference/types/boolean.html
    Bool,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types
    F32,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types
    F64,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    U8,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    U16,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    U32,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    U64,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    U128,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#machine-dependent-integer-types
    Usize,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    I8,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    I16,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    I32,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    I64,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#integer-types
    I128,
    /// See: https://doc.rust-lang.org/reference/types/numeric.html#machine-dependent-integer-types
    Isize,
    /// See: https://doc.rust-lang.org/reference/types/textual.html
    Char,
    /// See: https://doc.rust-lang.org/reference/types/textual.html
    Str,
    /// See: https://doc.rust-lang.org/std/string/struct.String.html
    String,
    /// See: https://doc.rust-lang.org/reference/types/pointer.html#shared-references-
    Ref(Box<Type>),
    /// See: https://doc.rust-lang.org/reference/types/pointer.html#mutable-references-mut
    RefMut(Box<Type>),
    /// See: https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut
    Ptr(Box<Type>),
    /// See: https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut
    PtrMut(Box<Type>),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "()" => Unit,
            "bool" => Bool,
            "f32" => F32,
            "f64" => F64,
            "u8" => U8,
            "u16" => U16,
            "u32" => U32,
            "u64" => U64,
            "u128" => U128,
            "usize" => Usize,
            "i8" => I8,
            "i16" => I16,
            "i32" => I32,
            "i64" => I64,
            "i128" => I128,
            "isize" => Isize,
            "char" => Char,
            "str" => Str,
            "String" => String,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Unit => write!(f, "()"),
            Bool => write!(f, "bool"),
            F32 => write!(f, "f32"),
            F64 => write!(f, "f64"),
            U8 => write!(f, "u8"),
            U16 => write!(f, "u16"),
            U32 => write!(f, "u32"),
            U64 => write!(f, "u64"),
            U128 => write!(f, "u128"),
            Usize => write!(f, "usize"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
            Isize => write!(f, "isize"),
            Char => write!(f, "char"),
            Str => write!(f, "str"),
            String => write!(f, "String"),
            Ref(t) => write!(f, "&{}", t),
            RefMut(t) => write!(f, "&mut {}", t),
            Ptr(t) => write!(f, "*const {}", t),
            PtrMut(t) => write!(f, "*mut {}", t),
        }
    }
}

//impl TryFrom<rust::Type> for Type {}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        Some(self.clone())
    }
}

impl crate::Type for Type {}
