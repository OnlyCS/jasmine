use super::*;

pub type GenericArguments = HashMap<String, Vec<ExplicitType>>;

pub fn parse_generic_args(rule: Pair<'_, Rule>) -> impl Iterator<Item = String> + '_ {
    rule.into_inner()
        .filter(|n| matches!(n.as_rule(), Rule::ident))
        .map(|r| r.as_str())
        .map(str::to_string)
}

pub fn parse_where_clause(
    rule: Pair<'_, Rule>,
    parser: &mut Parser,
    generics: &mut GenericArguments,
) {
    rule.into_inner()
        .filter(|n| matches!(n.as_rule(), Rule::where_unit))
        .map(|n| n.into_inner())
        .flatten()
        .filter(|n| matches!(n.as_rule(), Rule::ident | Rule::explicit_ty))
        .chunks(2)
        .into_iter()
        .map(|mut n| (n.next().unwrap(), n.next().unwrap()))
        .for_each(|(ident_rule, ty_rule)| {
            let ident = ident_rule.as_str().to_string();
            let ty = ExplicitType::parse_to_self(parser, ty_rule);

            generics
                .get_mut(&ident)
                .context(format!(
                    "Could not find generic argument, at {:?}",
                    ident_rule.line_col()
                ))
                .unwrap()
                .push(ty);
        });
}
