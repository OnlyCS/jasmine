pub use crate::prelude::*;

pub mod helpers;
pub mod parsers;
pub mod structs;

pub use helpers::generics::*;

pub use parsers::{
    assignments::*, enums::*, functions::*, loops_statements::*, structs::*, traits_impls::*,
    variables::*,
};

pub use structs::{
    assign_type::*, chars::*, expressions::*, function_call::*, if_condition::*, literals::*,
    match_arm::*, operators::*, types::*,
};

pub trait ParseToTree {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) -> Id;
}

pub trait ParseToSelf {
    fn parse_to_self(parser: &mut Parser, pair: Pair<'_, Rule>) -> Self;
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: Id,
    pub parent: Option<Id>,

    // Ids are like having infinite mutable references lol
    // and hashmaps for literally everything is absolutely brilliant
    pub tree: Vec<Id>,
    pub idents: HashMap<String, Id>,
    pub unordered_tree: HashMap<Id, TreeItem>,
    pub generics: HashSet<String>,
    pub implable: HashMap<String, Id>,
}

pub struct Parser {
    scopes: HashMap<Id, Scope>,
    current_scope: Id,
}

impl Parser {
    pub fn new() -> Self {
        let mut hm = HashMap::new();
        let new_scope = Scope {
            id: cuid(),
            parent: None,
            tree: vec![],
            idents: HashMap::new(),
            generics: HashSet::new(),
            unordered_tree: HashMap::new(),
            implable: HashMap::new(),
        };

        let id = new_scope.id;
        hm.insert(id, new_scope);

        Self {
            current_scope: id,
            scopes: hm,
        }
    }

    pub fn new_scope(&mut self) -> Id {
        let new_scope = Scope {
            id: cuid(),
            parent: Some(self.current_scope),
            tree: vec![],
            idents: HashMap::new(),
            generics: HashSet::new(),
            unordered_tree: HashMap::new(),
            implable: HashMap::new(),
        };

        let id = new_scope.id;

        self.scopes.insert(id, new_scope);
        self.current_scope = id;

        id
    }

    pub fn escape_scope(&mut self) {
        if let Some(parent) = self.scope(&self.current_scope).map(|n| n.parent).flatten() {
            self.current_scope = parent;
        } else {
            panic!("Cannot escape root scope");
        }
    }

    pub fn set_scope(&mut self, id: Id) {
        self.current_scope = id;
    }

    pub fn scope(&self, id: &Id) -> Option<&Scope> {
        self.scopes.get(id)
    }

    pub fn scope_mut(&mut self, id: &Id) -> Option<&mut Scope> {
        self.scopes.get_mut(id)
    }

    pub fn current_scope_id(&self) -> &Id {
        &self.current_scope
    }

    pub fn current_scope(&self) -> &Scope {
        self.scope(&self.current_scope).unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        let id = self.current_scope;

        self.scope_mut(&id).unwrap()
    }

    pub fn ident(&self, ident: &str) -> Option<&TreeItem> {
        let mut working = self.current_scope;

        while let Some(current) = self.scope(&working) {
            if let Some(id) = current.idents.get(ident) {
                return self.tree_item(id);
            } else {
                working = current.parent?;
            }
        }

        None
    }

    pub fn ident_mut(&mut self, ident: &str) -> Option<&mut TreeItem> {
        let mut working = self.current_scope;
        let mut id = None;

        while let Some(current) = self.scope(&working) {
            if let Some(found) = current.idents.get(ident) {
                id = Some(*found);
                break;
            } else {
                working = current.parent?;
            }
        }

        if let Some(id) = id {
            return self.tree_item_mut(&id);
        }

        None
    }

    pub fn implable(&self, ident: &str) -> Option<&TreeItem> {
        let mut working = self.current_scope;

        while let Some(current) = self.scope(&working) {
            if let Some(id) = current.implable.get(ident) {
                return self.tree_item(id);
            } else {
                working = current.parent?;
            }
        }

        None
    }

    pub fn implable_mut(&mut self, ident: &str) -> Option<&mut TreeItem> {
        let mut working = self.current_scope;
        let mut id = None;

        while let Some(current) = self.scope(&working) {
            if let Some(found) = current.implable.get(ident) {
                id = Some(*found);
                break;
            } else {
                working = current.parent?;
            }
        }

        self.tree_item_mut(&id?)
    }

    pub fn tree_item(&self, id: &Id) -> Option<&TreeItem> {
        let mut working = self.current_scope;

        while let Some(current) = self.scope(&working) {
            if let Some(tree) = current.unordered_tree.get(id) {
                return Some(tree);
            } else {
                working = current.parent?;
            }
        }

        None
    }

    pub fn tree_item_mut(&mut self, id: &Id) -> Option<&mut TreeItem> {
        let mut working = self.current_scope;

        while let Some(current) = self.scope(&working) {
            if current.unordered_tree.get(id).is_some() {
                break;
            } else {
                working = current.parent?;
            }
        }

        self.scope_mut(&working)?.unordered_tree.get_mut(id)
    }

    pub fn custom_type_exists(&self, ident: &str) -> bool {
        let mut working = self.current_scope;

        while let Some(current) = self.scope(&working) {
            if current.idents.get(ident).is_some() || current.generics.get(ident).is_some() {
                return true;
            } else {
                let Some(parent) = current.parent else {
                    return false;
                };

                working = parent;
            }
        }

        false
    }

    pub fn add_generic(&mut self, generic: String) {
        self.current_scope_mut().generics.insert(generic);
    }

    pub fn add_tree_item(&mut self, tree_item: TreeItem, add_to_ordered_tree: bool) {
        if let Some(ident) = tree_item.is_type() {
            self.current_scope_mut()
                .implable
                .insert(ident.to_string(), *tree_item.id());
        }

        if let Some(ident) = tree_item.ident() {
            self.current_scope_mut()
                .idents
                .insert(ident.to_string(), *tree_item.id());
        }

        if add_to_ordered_tree {
            self.current_scope_mut().tree.push(*tree_item.id());
        }

        self.current_scope_mut()
            .unordered_tree
            .insert(*tree_item.id(), tree_item);
    }

    pub fn parse_block(&self, rule: Pair<'_, Rule>) {}
}

#[derive(Clone, Debug)]
pub enum TreeItem {
    Structure {
        item_id: Id,
        struct_name: String,
        generics: GenericArguments,
        fields: HashMap<String, ExplicitType>,
        impls: Vec<Id>,
        scope: Id, // for generic types
    },
    Enumeration {
        item_id: Id,
        enum_name: String,
        variants: HashMap<String, Option<ExplicitType>>,
        generics: GenericArguments,
        impls: Vec<Id>,
        scope: Id, // for generic types
    },
    Function {
        item_id: Id,
        fn_name: String,
        args: Vec<(String, ExplicitType)>,
        returns: Option<ExplicitType>,
        generics: GenericArguments,
        body: Id,

        // traits and impls
        _abstract: bool,
        _static: bool,
    },
    Trait {
        item_id: Id,
        trait_name: String,
        generics: GenericArguments,
        functions: Vec<Id>,
    },
    Implementation {
        item_id: Id,
        trait_id: Option<Id>,
        type_id: Id,
        functions: Vec<Id>,
    },

    Variable {
        item_id: Id,
        var_name: String,
        ty: InferredType,
        value: Expression,
        mutable: bool,
    },

    StmtReturn {
        item_id: Id,
        expression: Expression,
    },
    StmtBreak {
        item_id: Id,
    },
    StmtContinue {
        item_id: Id,
    },

    Assignment {
        item_id: Id,
        lhs: String,
        rhs: Expression,
    },
    Expression {
        item_id: Id,
        expression: Expression,
    },

    If {
        item_id: Id,
        condition: IfCondition,
        body: Id,
        else_ifs: Vec<(IfCondition, Id)>,
        else_block: bool,
    },
    While {
        item_id: Id,
        condition: Expression,
        body: Id,
    },
    For {
        item_id: Id,
        arg: (String, InferredType),
        iter: Expression,
        body: Id,
    },
    Match {
        item_id: Id,
        arms: HashMap<MatchArm, Id>,
    },
}

impl TreeItem {
    pub fn id(&self) -> &Id {
        match self {
            TreeItem::Assignment { item_id, .. } => item_id,
            TreeItem::Enumeration { item_id, .. } => item_id,
            TreeItem::Expression { item_id, .. } => item_id,
            TreeItem::For { item_id, .. } => item_id,
            TreeItem::Function { item_id, .. } => item_id,
            TreeItem::If { item_id, .. } => item_id,
            TreeItem::Implementation { item_id, .. } => item_id,
            TreeItem::Match { item_id, .. } => item_id,
            TreeItem::StmtBreak { item_id, .. } => item_id,
            TreeItem::StmtContinue { item_id, .. } => item_id,
            TreeItem::StmtReturn { item_id, .. } => item_id,
            TreeItem::Structure { item_id, .. } => item_id,
            TreeItem::Trait { item_id, .. } => item_id,
            TreeItem::Variable { item_id, .. } => item_id,
            TreeItem::While { item_id, .. } => item_id,
        }
    }

    pub fn ident(&self) -> Option<&String> {
        match self {
            TreeItem::Structure { struct_name, .. } => Some(struct_name),
            TreeItem::Enumeration { enum_name, .. } => Some(enum_name),
            TreeItem::Function { fn_name, .. } => Some(fn_name),
            TreeItem::Trait { trait_name, .. } => Some(trait_name),
            TreeItem::Variable { var_name, .. } => Some(var_name),
            _ => None,
        }
    }

    pub fn is_type(&self) -> Option<&String> {
        match self {
            TreeItem::Structure {
                struct_name: name, ..
            }
            | TreeItem::Enumeration {
                enum_name: name, ..
            } => Some(name),
            _ => None,
        }
    }
}
