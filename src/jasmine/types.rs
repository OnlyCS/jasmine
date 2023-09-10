use super::*;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct ClosureTypeArgument {
    pub generic: bool,
    pub explicit_type: ExplicitType,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ExplicitType {
    Integer,
    Float,
    Boolean,
    String,
    Character,
    Custom(Identifier),
    Closure {
        arguments: Vec<ClosureTypeArgument>,
        return_type: Option<Box<ExplicitType>>,
    },
    Array(Box<ExplicitType>),
    Range,
    Generic {
        outer: Box<ExplicitType>,
        inner: Vec<ExplicitType>,
    },
}

impl ParseToSelf for ExplicitType {
    fn parse_to_self(parser: &mut Parser, pair: Pair<'_, Rule>) -> Self {
        let line_col = pair.line_col();
        let inner_pr = pair.into_inner().next().unwrap();

        match inner_pr.as_rule() {
            Rule::int_ty => ExplicitType::Integer,
            Rule::float_ty => ExplicitType::Float,
            Rule::bool_ty => ExplicitType::Boolean,
            Rule::string_ty => ExplicitType::String,
            Rule::char_ty => ExplicitType::Character,
            Rule::closure_ty => {
                let mut arguments = vec![];
                let mut return_type = None;
                let mut after_rparen = false;
                let mut current_arg = None;

                for rule in inner_pr.into_inner() {
                    match rule.as_rule() {
                        Rule::rparen => after_rparen = true,
                        Rule::generic_kwd => {
                            current_arg = Some(ClosureTypeArgument {
                                generic: true,
                                explicit_type: ExplicitType::Integer,
                            });
                        }
                        Rule::explicit_ty if !after_rparen => {
                            let mut taken = current_arg.take().unwrap_or(ClosureTypeArgument {
                                generic: false,
                                explicit_type: ExplicitType::Integer,
                            });

                            taken.explicit_type = ExplicitType::parse_to_self(parser, rule);

                            arguments.push(taken);
                        }
                        Rule::explicit_ty if after_rparen => {
                            return_type = Some(Box::new(ExplicitType::parse_to_self(parser, rule)));
                        }
                        _ => {}
                    }
                }

                ExplicitType::Closure {
                    arguments,
                    return_type,
                }
            }
            Rule::ident_ty => {
                let ident_str = inner_pr.as_str().to_string();
                let ident = Identifier {
                    name: ident_str,
                    scope_name: parser.current_scope().name.clone(),
                };

                ExplicitType::Custom(ident)
            }
            Rule::array_ty => {
                let mut inner = None;

                for rule in inner_pr.into_inner() {
                    let Rule::explicit_ty = rule.as_rule() else { continue };

                    inner = Some(Box::new(ExplicitType::parse_to_self(parser, rule)));
                    break;
                }

                ExplicitType::Array(
                    inner
                        .context(format!(
                            "Array type must have inner type, at {:?}",
                            line_col
                        ))
                        .unwrap(),
                )
            }
            Rule::range_ty => ExplicitType::Range,
            Rule::generic_ty => {
                let mut outer = None;
                let mut inner = vec![];

                for rule in inner_pr.into_inner() {
                    match rule.as_rule() {
                        Rule::explicit_ty if outer.is_none() => {
                            outer = Some(Box::new(ExplicitType::parse_to_self(parser, rule)));
                        }
                        Rule::explicit_ty => {
                            inner.push(ExplicitType::parse_to_self(parser, rule));
                        }
                        _ => {}
                    }
                }

                ExplicitType::Generic {
                    outer: outer
                        .context(format!(
                            "Generic type must have outer type, at {:?}",
                            line_col
                        ))
                        .unwrap(),
                    inner,
                }
            }
            _ => panic!("Type expected during parsing, at {:?}", line_col),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum InferredType {
    Integer,
    Float,
    Boolean,
    String,
    Character,
    Custom(Identifier),
    Closure {
        arguments: Vec<ClosureTypeArgument>,
        return_type: Option<Box<ExplicitType>>,
    },
    Array(Box<ExplicitType>),
    Range,
    Generic {
        outer: Box<ExplicitType>,
        inner: Vec<InferredType>,
    },
    Inferred,
}
