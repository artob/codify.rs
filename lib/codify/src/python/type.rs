// This is free and unencumbered software released into the public domain.

use crate::rust;

/// See: https://docs.python.org/3/library/stdtypes.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    NoneType,

    /// See: https://docs.python.org/3/library/stdtypes.html#boolean-type-bool
    Bool,

    /// See: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
    Float,

    /// See: https://rubygems.org/gems/ffi
    #[cfg(feature = "language-c")]
    Ffi(crate::c::Type),
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match input {
            "NoneType" => NoneType,
            "bool" => Bool,
            "float" => Float,
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Type::*;
        match self {
            NoneType => write!(f, "NoneType"),
            Bool => write!(f, "bool"),
            Float => write!(f, "float"),
            #[cfg(feature = "language-c")]
            Ffi(t) => match t {
                // See: https://docs.python.org/3/library/ctypes.html#fundamental-data-types
                crate::c::Type::Void => write!(f, "None"),
                crate::c::Type::Bool => write!(f, "c_bool"),
                crate::c::Type::Float => write!(f, "c_float"),
                crate::c::Type::Double => write!(f, "c_double"),
                crate::c::Type::Char => write!(f, "c_char"),
                crate::c::Type::SChar => write!(f, "c_byte"),
                crate::c::Type::Short => write!(f, "c_short"),
                crate::c::Type::Int => write!(f, "c_int"),
                crate::c::Type::Long => write!(f, "c_long"),
                crate::c::Type::LongLong => write!(f, "c_longlong"),
                crate::c::Type::UChar => write!(f, "c_ubyte"),
                crate::c::Type::UShort => write!(f, "c_ushort"),
                crate::c::Type::UInt => write!(f, "c_uint"),
                crate::c::Type::ULong => write!(f, "c_ulong"),
                crate::c::Type::ULongLong => write!(f, "c_ulonglong"),
                crate::c::Type::Array(t, None) => write!(f, "POINTER({})", Ffi((**t).clone())),
                crate::c::Type::Array(t, Some(n)) => write!(f, "{} * {}", Ffi((**t).clone()), n),
                crate::c::Type::Ptr(t) if **t == crate::c::Type::Char => write!(f, "c_char_p"),
                crate::c::Type::PtrMut(t) if **t == crate::c::Type::Char => write!(f, "c_char_p"),
                crate::c::Type::Ptr(t) if **t == crate::c::Type::Void => write!(f, "c_void_p"),
                crate::c::Type::PtrMut(t) if **t == crate::c::Type::Void => write!(f, "c_void_p"),
                crate::c::Type::Ptr(t) | crate::c::Type::PtrMut(t) => {
                    write!(f, "POINTER({})", Ffi((**t).clone()))
                }
            },
        }
    }
}

impl TryFrom<rust::Type> for Type {
    type Error = ();

    fn try_from(input: rust::Type) -> Result<Self, Self::Error> {
        use Type::*;
        Ok(match input {
            rust::Type::Any => return Err(()),
            rust::Type::Unit => NoneType,
            rust::Type::Bool => Bool,
            rust::Type::F32 | rust::Type::F64 => Float,
            #[cfg(feature = "language-c")]
            rust::Type::Ffi(t) => Type::Ffi(t),
            _ => return Err(()),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            NoneType => rust::Type::Unit,
            Bool => rust::Type::Bool,
            Float => rust::Type::F64,
            #[cfg(feature = "language-c")]
            Ffi(t) => rust::Type::Ffi(t.clone()),
        })
    }
}

impl crate::Type for Type {}
