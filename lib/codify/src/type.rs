// This is free and unencumbered software released into the public domain.

use core::fmt::{self, Debug, Display};

pub trait Type: Debug + Display {
    fn to_rust(&self) -> crate::rust::Type;
}
