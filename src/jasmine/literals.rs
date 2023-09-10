use super::*;

#[derive(Debug, Clone)]
pub struct StructureLiteral {
    pub identifier: Identifier,
    pub fields: HashMap<Identifier, ExplicitType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub begin: i64,
    pub end: i64,
    pub inclusive: bool,
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
    Closure(Function),
    Range(Range),
}
