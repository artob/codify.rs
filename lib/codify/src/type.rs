// This is free and unencumbered software released into the public domain.

use core::fmt::{self, Debug, Display};

pub trait ToRust {
    fn to_rust(&self) -> Option<crate::rust::Type>;
}

pub trait Type: ToRust + Debug + Display {}
