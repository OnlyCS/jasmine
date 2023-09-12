#![feature(mem_copy_fn, if_let_guard)]

extern crate anyhow;
extern crate clap;
extern crate itertools;
extern crate pest;
extern crate pest_derive;

mod args;
mod jasmine;
mod parser;
mod prelude;

use crate::prelude::*;

fn main() -> Result<()> {
    Ok(())
}
