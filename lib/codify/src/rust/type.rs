// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, String},
    rust,
};

/// See: https://doc.rust-lang.org/reference/types.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://doc.rust-lang.org/core/any/index.html
    Any,

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

    /// See: https://doc.rust-lang.org/core/ops/struct.Range.html
    Range(Box<Type>),

    /// See: https://doc.rust-lang.org/reference/types/textual.html
    Char,

    /// See: https://doc.rust-lang.org/reference/types/textual.html
    Str,

    /// See: https://doc.rust-lang.org/alloc/string/struct.String.html
    String,

    /// See: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html
    Box(Box<Type>),

    /// See: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html
    Vec(Box<Type>),

    /// See: https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html
    Map(Box<Type>, Box<Type>),

    /// See: https://doc.rust-lang.org/reference/types/pointer.html#shared-references-
    Ref(Box<Type>),

    /// See: https://doc.rust-lang.org/reference/types/pointer.html#mutable-references-mut
    RefMut(Box<Type>),

    /// See: https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut
    Ptr(Box<Type>),

    /// See: https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut
    PtrMut(Box<Type>),

    /// See: https://doc.rust-lang.org/std/ffi/index.html
    #[cfg(feature = "language-c")]
    Ffi(crate::c::Type),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use crate::prelude::Box;
        use Type::*;
        Ok(match input {
            "Any" => Any,
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
            _ => {
                if input.starts_with("Range<") && input.ends_with('>') {
                    Range(Box::new(input[6..input.len() - 1].parse()?))
                } else if input.starts_with("Box<") && input.ends_with('>') {
                    Type::Box(Box::new(input[4..input.len() - 1].parse()?))
                } else if input.starts_with("Vec<") && input.ends_with('>') {
                    Vec(Box::new(input[4..input.len() - 1].parse()?))
                } else if input.starts_with("Map<") && input.ends_with('>') {
                    let mut parts = input[4..input.len() - 1].split(',');
                    Map(
                        Box::new(parts.next().unwrap().parse()?),
                        Box::new(parts.next().unwrap().parse()?),
                    )
                } else if input.starts_with("BTreeMap<") && input.ends_with('>') {
                    let mut parts = input[9..input.len() - 1].split(',');
                    Map(
                        Box::new(parts.next().unwrap().parse()?),
                        Box::new(parts.next().unwrap().parse()?),
                    )
                } else if input.starts_with("&mut ") {
                    RefMut(Box::new(input[5..input.len()].parse()?))
                } else if input.starts_with("&") {
                    Ref(Box::new(input[1..input.len()].parse()?))
                } else if input.starts_with("*const ") {
                    Ptr(Box::new(input[7..input.len()].parse()?))
                } else if input.starts_with("*mut ") {
                    PtrMut(Box::new(input[5..].parse()?))
                } else if input.starts_with("c_") {
                    #[cfg(not(feature = "language-c"))]
                    return Err(());
                    #[cfg(feature = "language-c")]
                    match input {
                        "c_void" => Ffi(crate::c::Type::Void),
                        "c_float" => Ffi(crate::c::Type::Float),
                        "c_double" => Ffi(crate::c::Type::Double),
                        "c_char" => Ffi(crate::c::Type::Char),
                        "c_schar" => Ffi(crate::c::Type::SChar),
                        "c_short" => Ffi(crate::c::Type::Short),
                        "c_int" => Ffi(crate::c::Type::Int),
                        "c_long" => Ffi(crate::c::Type::Long),
                        "c_longlong" => Ffi(crate::c::Type::LongLong),
                        "c_uchar" => Ffi(crate::c::Type::UChar),
                        "c_ushort" => Ffi(crate::c::Type::UShort),
                        "c_uint" => Ffi(crate::c::Type::UInt),
                        "c_ulong" => Ffi(crate::c::Type::ULong),
                        "c_ulonglong" => Ffi(crate::c::Type::ULongLong),
                        _ => return Err(()),
                    }
                } else {
                    return Err(());
                }
            }
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            Any => write!(f, "Any"),
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
            Range(t) => write!(f, "Range<{}>", t),
            Char => write!(f, "char"),
            Str => write!(f, "str"),
            String => write!(f, "String"),
            Box(t) => write!(f, "Box<{}>", t),
            Vec(t) => write!(f, "Vec<{}>", t),
            Map(k, v) => write!(f, "BTreeMap<{}, {}>", k, v),
            Ref(t) => write!(f, "&{}", t),
            RefMut(t) => write!(f, "&mut {}", t),
            Ptr(t) => write!(f, "*const {}", t),
            PtrMut(t) => write!(f, "*mut {}", t),
            #[cfg(feature = "language-c")]
            Ffi(t) => match t {
                crate::c::Type::Void => write!(f, "c_void"),
                crate::c::Type::Bool => write!(f, "bool"),
                crate::c::Type::Float => write!(f, "c_float"),
                crate::c::Type::Double => write!(f, "c_double"),
                crate::c::Type::Char => write!(f, "c_char"),
                crate::c::Type::SChar => write!(f, "c_schar"),
                crate::c::Type::Short => write!(f, "c_short"),
                crate::c::Type::Int => write!(f, "c_int"),
                crate::c::Type::Long => write!(f, "c_long"),
                crate::c::Type::LongLong => write!(f, "c_longlong"),
                crate::c::Type::UChar => write!(f, "c_uchar"),
                crate::c::Type::UShort => write!(f, "c_ushort"),
                crate::c::Type::UInt => write!(f, "c_uint"),
                crate::c::Type::ULong => write!(f, "c_ulong"),
                crate::c::Type::ULongLong => write!(f, "c_ulonglong"),
                crate::c::Type::Ptr(t) => write!(f, "*const {}", Ffi((**t).clone())),
                crate::c::Type::PtrMut(t) => write!(f, "*mut {}", Ffi((**t).clone())),
            },
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
