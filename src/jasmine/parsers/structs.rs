use super::*;

#[derive(Debug, Clone)]
pub struct Structure;

impl ParseToTree for Structure {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) -> Id {
        let scope = parser.new_scope();

        // output vars
        let item_id = cuid();
        let mut struct_name = None;
        let mut generics = GenericArguments::new();
        let mut fields = HashMap::new();

        // temp vars
        let mut working_ident = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if struct_name.is_none() => {
                    struct_name = Some(rule.as_str().to_string());
                }
                Rule::generic_args => parse_generic_args(rule).for_each(|n| {
                    generics.insert(n, vec![]);
                }),
                Rule::where_clause => parse_where_clause(rule, parser, &mut generics),
                Rule::ident if working_ident.is_none() => working_ident = Some(rule.as_str().to_string()),
				Rule::explicit_ty if let Some(ident) = working_ident.take() => {
					fields.insert(ident, ExplicitType::parse_to_self(parser, rule));
				}
				_ => {}
            }
        }

        generics.keys().for_each(|f| parser.add_generic(f.clone()));

        let struct_tree = TreeItem::Structure {
            item_id,
            struct_name: struct_name.unwrap(),
            generics,
            fields,
            impls: vec![],
            scope,
        };

        parser.escape_scope();

        parser.add_tree_item(struct_tree, true); // automatically adds ident

        item_id
    }
}
