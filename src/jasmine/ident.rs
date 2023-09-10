use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub scope_name: String,
}

#[derive(Clone, Debug)]
pub enum Identifiable {
    Function(Function),
    Variable(Variable),
    Struct(Structure),
    Enum(Enumeration),
    GenericArgument(Identifier),
}

impl Identifiable {
    pub fn ident_str(&self) -> &str {
        match self {
            Identifiable::Function(f) => &f.identifier.name,
            Identifiable::Variable(v) => &v.identifier.name,
            Identifiable::Struct(s) => &s.identifier.name,
            Identifiable::Enum(e) => &e.identifier.name,
            Identifiable::GenericArgument(g) => &g.name,
        }
    }
}
