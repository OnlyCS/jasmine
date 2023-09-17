pub use crate::parser::Rule;
pub use anyhow::*;
pub use pest::iterators::Pair;

pub trait Parse: Sized {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self>;
}

pub trait ParseMany: Sized {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>>;
}

pub trait Rewrite {
    fn rewrite(&self) -> String;
}

pub use itertools::Itertools;
pub use std::collections::{HashMap, HashSet};
pub use std::mem;

pub const SELF_IDENT: &str = "__jas_self"; // exactly 10 characters, will fit perfectly in byte array

pub fn cuid() -> Id {
    let mut bytes = [0; 24];

    for (i, byte) in cuid::cuid2().bytes().enumerate() {
        if i >= 24 {
            break;
        }

        bytes[i] = byte;
    }

    bytes
}

pub type Id = [u8; 24];
