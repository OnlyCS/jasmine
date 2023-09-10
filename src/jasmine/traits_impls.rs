use super::*;

#[derive(Debug, Clone)]
pub struct ImplementationMethod {
    pub identifier: Identifier,
    pub generics: GenericArguments,
    pub arguments: Vec<(Identifier, ExplicitType)>,
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub identifier: Identifier,
    pub generics: GenericArguments,
    pub arguments: Vec<(Identifier, ExplicitType)>,
    pub is_abstract: bool,
}

#[derive(Debug, Clone)]
pub struct Implementation {
    pub identifier: Identifier,
    pub for_trait: Option<ExplicitType>,
    pub methods: HashSet<ImplementationMethod>,
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub identifier: Identifier,
    pub menthods: HashSet<ImplementationMethod>,
}
