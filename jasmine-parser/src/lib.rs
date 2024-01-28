#![feature(error_generic_member_access)]

extern crate syn;

mod error;
mod mod_resolve;
mod prelude;
#[cfg(test)]
mod tests;

use prelude::*;
use syn::parse_file;

pub(crate) fn _parse(file_path: &mut PathBuf) -> Result<File, ParserError> {
    trace!("Attempting to read file to string: {}", file_path.display());
    let content = read_file(&file_path)?;

    trace!("Attempting to parse file content to AST");
    let mut file = parse_file(&content)?;

    trace!("Resolving modules in file: {}", file_path.display());
    mod_resolve::resolve(file_path, &mut file)?;

    Ok(file)
}

pub fn parse(mut file_path: PathBuf) -> Result<File, ParserError> {
    _parse(&mut file_path)
}
