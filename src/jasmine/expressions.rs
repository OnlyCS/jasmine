use super::*;

#[derive(Debug, Clone)]
pub enum AfterDotExpressionType {
    FunctionCall(Identifier),
    Property(Identifier),
    IndexArray(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    FunctionCall(Identifier),
    Ident(Identifier),
    UnitEnum {
        type_ident: Identifier,
        variant: Identifier,
    },
    StaticFunction {
        ty: ExplicitType,
        function: FunctionCall,
    },
    TupleEnum {
        type_ident: Identifier,
        variant: Identifier,
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
