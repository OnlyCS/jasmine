use std::collections::{HashMap, HashSet};

use rand::Rng;

use crate::functions::Function;

pub type TypeId = u32;

pub fn new_type_id() -> TypeId {
    rand::thread_rng().gen()
}

#[derive(Clone, Debug)]
pub struct Generic {
    pub id: TypeId,
    pub name: String,
    pub constraints: HashSet<TypeId>, // TODO: See if we actually need HashSet > Vec
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub id: TypeId,
    pub name: String,
    pub fields: HashMap<String, TypeId>,
    pub generics: Vec<TypeId>,
    pub methods: HashMap<String, Function>,
}

#[derive(Clone, Debug)]
pub enum EnumVariantData {
    Struct(HashMap<String, TypeId>),
    Tuple(Vec<TypeId>),
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<EnumVariantData>, // None == unit variant
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub id: TypeId,
    pub name: String,
    pub variants: HashMap<String, EnumVariant>,
    pub generics: Vec<TypeId>,
    pub methods: HashMap<String, Function>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Struct(Struct),
    Enum(Enum),
    Generic(Generic),
    Alias(String, TypeId),
    JavaBuiltin(String),
}
