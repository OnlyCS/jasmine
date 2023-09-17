use super::*;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub ident: String,
    pub args: Vec<Expression>,
}
