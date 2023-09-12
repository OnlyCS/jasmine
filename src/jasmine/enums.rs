use super::*;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct EnumVariant {
    pub variant: Identifier,
    pub data: Option<ExplicitType>,
}

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub identifier: Identifier,
    pub variants: HashSet<EnumVariant>,
    pub generics: GenericArguments,
    pub impls: Vec<Implementation>,
}
