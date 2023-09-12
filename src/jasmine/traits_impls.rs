use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplementationMethod {
    pub identifier: IdentName, // functions should not be compiler-scoped
    pub generics: GenericArguments,
    pub arguments: Vec<(Identifier, ExplicitType)>,
    pub return_type: Option<ExplicitType>,
    pub is_static: bool,
}

impl ParseToSelf for ImplementationMethod {
    fn parse_to_self<'a>(parser: &mut Parser, pair: Pair<'a, Rule>) -> Self {
        let inner_scope = parser.add_child_scope();
        let mut identifier = None;
        let mut generics = GenericArguments::new();
        let mut arguments = vec![];
        let mut return_type = None;
        let mut is_static = true;

        let mut current_arg_ident = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if identifier.is_none() => {
					identifier = Some(encode_ident(rule.as_str()));
                }
                Rule::generic_args => {
                    for generic in parse_generic_args(rule) {
                        let ident = Identifier {
                            name: generic,
                            scope_id: inner_scope,
                        };

                        parser.add_ident(Identifiable::GenericArgument(ident));

                        generics.insert(ident, vec![]);
                    }
                }
                Rule::self_kwd => {
                    is_static = false;

                    let ident = Identifier {
                        name: encode_ident("self"),
                        scope_id: inner_scope,
                    };

                    parser.add_ident(Identifiable::FunctionArgument(
                        ident,
                        ExplicitType::SelfType
                    ));
                }
                Rule::ident if current_arg_ident.is_none() => {
                    let ident = Identifier {
                        name: encode_ident(rule.as_str()),
                        scope_id: inner_scope,
                    };

                    current_arg_ident = Some(ident);
                }
				Rule::explicit_ty if let Some(ident) = current_arg_ident.take() => {
					let explicit_ty = ExplicitType::parse_to_self(parser, rule);

					parser.add_ident(Identifiable::FunctionArgument(ident, explicit_ty.clone()));

					arguments.push((ident, explicit_ty));
				}
				Rule::explicit_ty if current_arg_ident.is_none() => {
					return_type = Some(ExplicitType::parse_to_self(parser, rule));
				}
				Rule::block => {
					todo!("parser.parse_block(rule);");
				}
                _ => {}
            }
        }

        parser.escape_scope();

        Self {
            identifier: identifier.unwrap(),
            generics,
            arguments,
            return_type,
            is_static,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub identifier: Identifier,
    pub generics: GenericArguments,
    pub arguments: Vec<(Identifier, ExplicitType)>,
    pub return_type: Option<ExplicitType>,
    pub is_abstract: bool,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct Implementation {
    pub for_type: Identifier,
    pub impl_trait: Option<ExplicitType>,
    pub methods: Vec<ImplementationMethod>,
}

impl ParseToTree for Implementation {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) {
        let mut for_type = None;
        let mut impl_trait = None;
        let mut methods = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::explicit_ty => {
                    impl_trait = Some(ExplicitType::parse_to_self(parser, rule));
                }
                Rule::ident => {
                    let ident_str = encode_ident(rule.as_str());
                    let ident = parser.find_ident(&ident_str).context(format!("Could not find this type! This struct or enum MUST be declared before it is impl'd. At {:?}", rule.line_col())).cloned().unwrap();

                    if !matches!(ident, Identifiable::Struct(_) | Identifiable::Enum(_)) {
                        panic!(
                            "Cannot write an impl for a non-struct or non-enum type. At {:?}",
                            rule.line_col()
                        );
                    }

                    if for_type.is_some() {
                        panic!(
                            "Cannot impl for more than one type. At {:?}",
                            rule.line_col()
                        );
                    }

                    parser.add_ident(ident.clone());
                    for_type = Some(*ident.full_ident());
                }
                Rule::impl_fn_def => {
                    let method = ImplementationMethod::parse_to_self(parser, rule);

                    methods.push(method);
                }
                _ => {}
            }
        }

        let this = Self {
            for_type: for_type.unwrap(),
            impl_trait,
            methods,
        };

        parser.add_to_tree(TreeItem::Impl(this));
    }
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub identifier: Identifier,
    pub menthods: HashSet<ImplementationMethod>,
}
