// This is free and unencumbered software released into the public domain.

use core::fmt::Display;

pub trait Type: Display {
    fn to_rust(&self) -> crate::rust::Type;
}
