#[derive(Clone, Debug, PartialEq, Hash)]
pub enum UnaryOperator {
    Not,
    Neg,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}
