use super::*;

#[derive(Debug, Clone)]
pub struct StructureLiteral {
    pub identifier: String,
    pub fields: HashMap<String, ExplicitType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeLiteral {
    pub begin: i64,
    pub end: i64,
    pub inclusive: bool,
}

#[derive(Clone, Debug)]
pub struct ClosureLiteral {
    pub args: Vec<(String, InferredType)>,
    pub returns: Option<InferredType>,
    pub body: Id,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(Vec<Character>),
    Character(Character),
    Array(Vec<Expression>),
    Structure(StructureLiteral),
    Closure(ClosureLiteral),
    Range(RangeLiteral),
}
