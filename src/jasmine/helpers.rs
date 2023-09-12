use super::*;

pub fn parse_generic_args(pair: Pair<'_, Rule>) -> Vec<IdentName> {
    let mut args = vec![];

    for rule in pair.into_inner() {
        match rule.as_rule() {
            Rule::ident => {
                args.push(encode_ident(rule.as_str()));
            }
            _ => {}
        }
    }

    args
}
