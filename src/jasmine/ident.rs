use super::*;

pub type IdentName = [u8; 10];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: IdentName,
    pub scope_id: ScopeId,
}

#[derive(Clone, Debug)]
pub enum Identifiable {
    Variable(Variable),
    Struct(Structure),
    Enum(Enumeration),
    GenericArgument(Identifier),
    FunctionArgument(Identifier, ExplicitType),
}

impl Identifiable {
    pub fn ident_bytes(&self) -> &IdentName {
        match self {
            Identifiable::Variable(v) => &v.identifier.name,
            Identifiable::Struct(s) => &s.identifier.name,
            Identifiable::Enum(e) => &e.identifier.name,
            Identifiable::GenericArgument(g) => &g.name,
            Identifiable::FunctionArgument(a, _) => &a.name,
        }
    }

    pub fn full_ident(&self) -> &Identifier {
        match self {
            Identifiable::Variable(v) => &v.identifier,
            Identifiable::Struct(s) => &s.identifier,
            Identifiable::Enum(e) => &e.identifier,
            Identifiable::GenericArgument(g) => g,
            Identifiable::FunctionArgument(g, _) => g,
        }
    }
}
