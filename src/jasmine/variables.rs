use super::*;

#[derive(Clone, Debug)]
pub struct Variable {
    pub identifier: Identifier,
    pub ty: InferredType,
    pub expr: Expression,
}
