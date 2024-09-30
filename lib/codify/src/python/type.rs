// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, format, Box, Cow, Named, Vec},
    rust,
};

/// See: https://docs.python.org/3/library/stdtypes.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    NoneType,

    /// See: https://docs.python.org/3/library/stdtypes.html#boolean-type-bool
    Bool,

    /// See: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
    Int,

    /// See: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
    Float,

    /// See: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
    Complex,

    /// See: https://docs.python.org/3/library/stdtypes.html#text-sequence-type-str
    Str,

    /// See: https://docs.python.org/3/library/stdtypes.html#ranges
    Range,

    /// See: https://docs.python.org/3/library/stdtypes.html#lists
    List(Box<Type>),

    /// See: https://docs.python.org/3/library/stdtypes.html#mapping-types-dict
    Dict(Box<Type>, Box<Type>),

    /// See: https://docs.python.org/3/library/ctypes.html#fundamental-data-types
    #[cfg(feature = "language-c")]
    FFI(crate::c::Type),
}

impl Type {
    fn parse_ffi(input: &str) -> Result<crate::c::Type, ()> {
        use Type::*;

        if let Some(star_idx) = input.find('*') {
            let inner = Self::parse_ffi(input[..star_idx].trim())?;
            let n = input[star_idx + 1..]
                .trim()
                .parse::<usize>()
                .map_err(|_| ())?;
            return Ok(crate::c::Type::Array(Box::new(inner), Some(n)));
        }

        Ok(match input {
            "c_bool" => crate::c::Type::Bool,
            "c_float" => crate::c::Type::Float,
            "c_double" => crate::c::Type::Double,
            "c_char" => crate::c::Type::Char,
            "c_byte" => crate::c::Type::SChar,
            "c_short" => crate::c::Type::Short,
            "c_int" => crate::c::Type::Int,
            "c_long" => crate::c::Type::Long,
            "c_longlong" => crate::c::Type::LongLong,
            "c_ssize_t" => crate::c::Type::SSize_t,
            "c_ubyte" => crate::c::Type::UChar,
            "c_ushort" => crate::c::Type::UShort,
            "c_uint" => crate::c::Type::UInt,
            "c_ulong" => crate::c::Type::ULong,
            "c_ulonglong" => crate::c::Type::ULongLong,
            "c_size_t" => crate::c::Type::Size_t,
            "c_void_p" => crate::c::Type::PtrMut(Box::new(crate::c::Type::Void)),
            "c_char_p" => crate::c::Type::PtrMut(Box::new(crate::c::Type::Char)),
            #[cfg(feature = "libc")]
            "c_time_t" => crate::c::Type::Time_t,
            _ => return Err(()),
        })
    }

    fn write_ffi(f: &mut fmt::Formatter, t: &crate::c::Type) -> fmt::Result {
        use crate::c::Type::*;

        match t {
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
            crate::c::Type::SSize_t => write!(f, "c_ssize_t"),
            crate::c::Type::UChar => write!(f, "c_ubyte"),
            crate::c::Type::UShort => write!(f, "c_ushort"),
            crate::c::Type::UInt => write!(f, "c_uint"),
            crate::c::Type::ULong => write!(f, "c_ulong"),
            crate::c::Type::ULongLong => write!(f, "c_ulonglong"),
            crate::c::Type::Size_t => write!(f, "c_size_t"),
            crate::c::Type::Array(t, Some(n)) => {
                // Format: {ffi} * {n}
                // Example: c_uint * 10
                Self::write_ffi(f, &**t)?;
                write!(f, " * {}", n);
                Ok(())
            }
            crate::c::Type::Ptr(t) if **t == crate::c::Type::Char => write!(f, "c_char_p"),
            crate::c::Type::PtrMut(t) if **t == crate::c::Type::Char => write!(f, "c_char_p"),
            crate::c::Type::Ptr(t) if **t == crate::c::Type::Void => write!(f, "c_void_p"),
            crate::c::Type::PtrMut(t) if **t == crate::c::Type::Void => write!(f, "c_void_p"),
            crate::c::Type::Ptr(t) | crate::c::Type::PtrMut(t) | crate::c::Type::Array(t, None) => {
                // POINTER({ffi})
                // POINTER(c_uint)
                write!(f, "POINTER(")?;
                Self::write_ffi(f, &**t)?;
                write!(f, ")")?;
                Ok(())
            }
            #[cfg(feature = "libc")]
            crate::c::Type::Time_t => write!(f, "c_time_t"),
        }
    }
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
            ']' if input_len > 3 => {
                let start_idx = input.find('[').ok_or(())?;

                return Ok(match input[..start_idx].trim() {
                    "list" if input_len > 7 => List(Box::new(
                        input[start_idx + 1..input_len - 1].trim().parse()?,
                    )),
                    "dict" if input_len > 7 => {
                        let comma_idx = input.find(',').ok_or(())?;

                        Dict(
                            Box::new(input[start_idx + 1..comma_idx].trim().parse()?),
                            Box::new(input[comma_idx + 1..input_len - 1].trim().parse()?),
                        )
                    }
                    _ => return Err(()),
                });
            }
            ')' if input_len > 3 => {
                let start_idx = input.find('(').ok_or(())?;

                return Ok(match input[..start_idx].trim() {
                    "POINTER" if input_len > 10 => FFI(crate::c::Type::PtrMut(Box::new(
                        Self::parse_ffi(&input[start_idx + 1..input_len - 1].trim())?,
                    ))),
                    _ => return Err(()),
                });
            }
            _ => (),
        }

        Ok(match input {
            "NoneType" => NoneType,
            "bool" => Bool,
            "int" => Int,
            "float" => Float,
            "complex" => Complex,
            "str" => Str,
            "range" => Range,
            _ => FFI(Self::parse_ffi(input)?),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Type::*;
        match self {
            NoneType => write!(f, "NoneType"),
            Bool => write!(f, "bool"),
            Int => write!(f, "int"),
            Float => write!(f, "float"),
            Complex => write!(f, "complex"),
            Str => write!(f, "str"),
            List(t) => write!(f, "list[{}]", t),
            Range => write!(f, "range"),
            Dict(k, v) => write!(f, "dict[{}, {}]", k, v),
            #[cfg(feature = "language-c")]
            FFI(t) => Self::write_ffi(f, t),
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
            rust::Type::Any => return Err(()),
            rust::Type::Unit => NoneType,
            rust::Type::Bool => Bool,
            rust::Type::I8
            | rust::Type::U8
            | rust::Type::I16
            | rust::Type::U16
            | rust::Type::I32
            | rust::Type::U32
            | rust::Type::I64
            | rust::Type::U64
            | rust::Type::Isize
            | rust::Type::Usize
            | rust::Type::I128
            | rust::Type::U128 => Int,
            rust::Type::F32 | rust::Type::F64 => Float,
            rust::Type::Range(_) => Range, // NOTE(yurii): We lose inner type in this case...
            rust::Type::Char => Str,
            rust::Type::Str | rust::Type::String => Str,
            rust::Type::Box(t) => return Self::try_from(*t),
            rust::Type::Vec(t) => List(Box::new(Self::try_from(*t)?)),
            rust::Type::Map(k, v) => {
                Dict(Box::new(Self::try_from(*k)?), Box::new(Self::try_from(*v)?))
            }
            rust::Type::Ref(_) | rust::Type::RefMut(_) => return Err(()),
            rust::Type::Ptr(_) | rust::Type::PtrMut(_) => return Err(()),
            #[cfg(feature = "language-c")]
            rust::Type::Ffi(t) => Type::FFI(t),
        })
    }
}

impl crate::ToRust for Type {
    fn to_rust(&self) -> Option<rust::Type> {
        use Type::*;
        Some(match self {
            NoneType => rust::Type::Unit,
            Bool => rust::Type::Bool,
            Int => rust::Type::I64,
            Float => rust::Type::F64,
            Complex => return None,
            Str => rust::Type::String,
            Range => return None,
            List(t) => rust::Type::Vec(Box::new(t.to_rust()?)),
            Dict(k, v) => rust::Type::Map(Box::new(k.to_rust()?), Box::new(v.to_rust()?)),
            #[cfg(feature = "language-c")]
            FFI(t) => rust::Type::Ffi(t.clone()),
        })
    }
}

impl crate::Type for Type {}
