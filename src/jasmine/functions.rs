use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: Identifier,
    pub args: Vec<(Identifier, ExplicitType)>, // args are positional, can't use hashmap :(
    pub returns: Option<ExplicitType>,
    pub generics: GenericArguments,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Closure {
    pub args: Vec<(Identifier, ExplicitType)>,
    pub returns: Option<ExplicitType>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub ident: Identifier,
    pub args: Vec<Expression>,
}
