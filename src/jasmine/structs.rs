use super::*;

#[derive(Debug, Clone)]
pub struct Structure {
    pub identifier: Identifier,
    pub fields: HashMap<IdentName /* Properties shouldnt be compiler-scoped */, ExplicitType>,
    pub generics: GenericArguments,
    pub impls: Vec<Implementation>,
}

impl ParseToTree for Structure {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) {
        let line_col = pair.line_col();
        let outside_struct_scope = parser.current_scope().id;
        let in_struct_scope = parser.add_child_scope(); // generic arguments can't be leaked

        let mut fields = HashMap::new();
        let mut next_ident = None;
        let mut struct_ident = None;
        let mut generics = GenericArguments::new();

        for rule in pair.into_inner() {
            let line_col = rule.line_col();

            match rule.as_rule() {
                Rule::ident if struct_ident.is_none() => {
                    struct_ident = Some(Identifier {
                        name: encode_ident(rule.as_str()),
                        scope_id: outside_struct_scope,
                    })
                }
                Rule::generic_args => {
                    for generic in parse_generic_args(rule) {
                        let ident = Identifier {
                            name: generic,
                            scope_id: in_struct_scope,
                        };

                        parser.add_ident(Identifiable::GenericArgument(ident));

                        generics.insert(ident, vec![]);
                    }
                }
                Rule::where_clause => {
                    for rule in rule.into_inner() {
                        let Rule::where_unit = rule.as_rule() else {
                            continue;
                        };

                        let mut where_unit = rule.into_inner();
                        let ident_rule = where_unit.next().unwrap();
                        let constraint_rule = where_unit.next().unwrap();

                        let ident = Identifier {
                            name: encode_ident(ident_rule.as_str()),
                            scope_id: in_struct_scope,
                        };

                        if let Some(generic) = generics.get_mut(&ident) {
                            let constraint = ExplicitType::parse_to_self(parser, constraint_rule);

                            generic.push(constraint);
                        } else {
                            panic!(
                                "Generic argument not found, at (line, col): {:?}",
                                ident_rule.line_col()
                            );
                        }
                    }
                }
                Rule::ident if next_ident.is_none() => {
                    next_ident = Some(encode_ident(rule.as_str()));
                }
                Rule::explicit_ty => {
                    let ty = ExplicitType::parse_to_self(parser, rule);
                    let ident_str = next_ident
                        .take()
                        .context(format!(
                            "Expected identifier before type, at {:?}",
                            line_col
                        ))
                        .unwrap();

                    fields.insert(ident_str, ty);
                }
                _ => {}
            }
        }

        parser.escape_scope();

        let this = Self {
            identifier: struct_ident
                .context(format!("Expected identifier, at {:?}", line_col))
                .unwrap(),
            fields,
            generics,
            impls: vec![],
        };

        parser.add_ident(Identifiable::Struct(this.clone()));
        parser.add_to_tree(TreeItem::Structure(this));
    }
}
