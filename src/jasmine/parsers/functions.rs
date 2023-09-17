use super::*;

pub struct Function;

impl ParseToTree for Function {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) -> Id {
        // output vars
        let item_id = cuid();
        let body = parser.new_scope();
        let mut fn_name = None;
        let mut args = vec![];
        let mut returns = None;
        let mut generics = GenericArguments::new();
        let mut _abstract = false;
        let mut _static = true;

        // working vars
        let mut working_ident = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if fn_name.is_none() => fn_name = Some(rule.as_str().to_string()),
                Rule::generic_args => parse_generic_args(rule).for_each(|n| {
                    generics.insert(n, vec![]);
                }),
                Rule::where_clause => parse_where_clause(rule, parser, &mut generics),
				Rule::self_kwd => _static = false,
				Rule::ident if working_ident.is_none() => working_ident = Some(rule.as_str().to_string()),
				Rule::explicit_ty if let Some(ident) = working_ident.take() => {
					let ty = ExplicitType::parse_to_self(parser, rule);

					args.push((ident, ty));
				}
				Rule::explicit_ty => returns = Some(ExplicitType::parse_to_self(parser, rule)),
                Rule::block => {
					generics.keys().for_each(|n| parser.add_generic(n.clone()));
					parser.parse_block(rule);
					parser.escape_scope();
				}
				_ => {}
            }
        }

        parser.add_tree_item(
            TreeItem::Function {
                item_id,
                fn_name: fn_name.unwrap(),
                args,
                returns,
                generics,
                body,
                _abstract,
                _static,
            },
            true,
        );

        item_id
    }
}
