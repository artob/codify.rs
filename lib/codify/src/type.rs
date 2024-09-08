// This is free and unencumbered software released into the public domain.

use crate::prelude::{
    fmt::{Debug, Display},
    Named,
};

pub trait ToRust {
    fn to_rust(&self) -> Option<crate::rust::Type>;
}

pub trait Type: ToRust + Named + Display + Debug {}
