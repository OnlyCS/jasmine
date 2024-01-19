use super::*;
use crate::prelude::*;

fn parse_constraints(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<Vec<UncheckedFullType>, ParserError> {
    let mut constraints = vec![];

    iterator
        .split(|a| match a {
            TokenTree::Punct(p) => p.as_char() == '+',
            _ => false,
        })
        .map(|mut iter| types::parse_full(&mut iter))
        .filter_map(Result::ok)
        .for_each(|generic| constraints.push(generic));

    Ok(constraints)
}

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<Vec<UncheckedGeneric>, ParserError> {
    let mut generics = vec![];

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '<' });

    iterator
        .copy_while(|a| match a {
            TokenTree::Punct(p) => p.as_char() != '>',
            _ => true,
        })
        .split(|a| match a {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        .map(|mut iter| {
            let ident = expect!(iter, TokenTree::Ident(i), ret { i.to_string() });
            let constraints = if iter
                .next_if(|a| matches!(a, TokenTree::Punct(p) if p.as_char() == ':'))
                .is_some()
            {
                parse_constraints(&mut iter)?
            } else {
                vec![]
            };

            Result::<_, ParserError>::Ok(UncheckedGeneric { ident, constraints })
        })
        .filter_map(Result::ok)
        .for_each(|generic| generics.push(generic));

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '>' });

    Ok(generics)
}
