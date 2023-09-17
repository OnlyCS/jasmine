use super::*;

#[derive(Debug, Clone)]
pub enum AfterDotExpressionType {
    FunctionCall(String),
    Property(String),
    IndexArray(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    FunctionCall(String),
    Ident(String),
    UnitEnum {
        type_ident: String,
        variant: String,
    },
    StaticFunction {
        ty: ExplicitType,
        function: FunctionCall,
    },
    TupleEnum {
        type_ident: String,
        variant: String,
        inner: Box<Expression>,
    },
    TypeCast {
        expression: Box<Expression>,
        ty: ExplicitType,
    },
    Literal(Literal),
    Iife,
}

#[derive(Debug, Clone)]
pub struct AfterDotExpression {
    pub kind: AfterDotExpressionType,
    pub next: Option<Box<AfterDotExpression>>,
}

#[derive(Debug, Clone)]
pub struct BaseExpression {
    pub unarys: Vec<UnaryOperator>,
    pub kind: ExpressionType,
    pub dot: Option<AfterDotExpression>,
}

#[derive(Debug, Clone)]
pub struct FullExpression {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Base(BaseExpression),
    Full(FullExpression),
}
