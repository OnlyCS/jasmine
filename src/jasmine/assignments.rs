use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AssignType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub identifier: Identifier,
    pub kind: AssignType,
    pub expression: Expression,
}
