use super::*;

pub fn parse_generic_args(pair: Pair<'_, Rule>) -> Vec<String> {
    let mut args = vec![];

    for rule in pair.into_inner() {
        match rule.as_rule() {
            Rule::ident => {
                args.push(rule.as_str().to_string());
            }
            _ => {}
        }
    }

    args
}
