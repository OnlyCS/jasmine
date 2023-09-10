use std::{fs::File, io::Read, path::PathBuf};

use crate::prelude::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "jasmine.pest"]
struct JasmineParser;

pub fn parse(file: PathBuf) -> Result<()> {
    let mut file_str = String::new();

    File::open(file)?.read_to_string(&mut file_str)?;

    let pest_parsed = JasmineParser::parse(Rule::program, &file_str)
        .map(|mut n| n.next().context("Failed to parse").map(Pair::into_inner))??;

    Ok(())
}
