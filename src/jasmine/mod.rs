pub use crate::prelude::*;

pub mod assignments;
pub mod chars;
pub mod enums;
pub mod expressions;
pub mod functions;
pub mod generics;
pub mod helpers;
pub mod ident;
pub mod literals;
pub mod loops_statements;
pub mod operators;
pub mod structs;
pub mod traits_impls;
pub mod types;
pub mod variables;

pub use assignments::*;
pub use chars::*;
pub use enums::*;
pub use expressions::*;
pub use functions::*;
pub use generics::*;
pub use helpers::*;
pub use ident::*;
pub use literals::*;
pub use loops_statements::*;
pub use operators::*;
pub use structs::*;
pub use traits_impls::*;
pub use types::*;
pub use variables::*;

pub trait ParseToTree {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>);
}

pub trait ParseToSelf {
    fn parse_to_self(parser: &mut Parser, pair: Pair<'_, Rule>) -> Self;
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub id: ScopeId,
    pub idents: HashMap<IdentName, Identifiable>,
    pub parent: Option<Box<Scope>>,
    pub tree: Vec<TreeItem>,
}

pub struct Parser {
    pub current_scope: Scope,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_scope: Scope {
                id: cuid(),
                idents: HashMap::new(),
                parent: None,
                tree: vec![],
            },
        }
    }

    // without clone?? I feel like im cheating
    pub fn add_child_scope(&mut self) -> ScopeId {
        let child = Scope {
            id: cuid(),
            idents: HashMap::new(),
            parent: None,
            tree: vec![],
        };

        let parent = mem::replace(&mut self.current_scope, child); // self.current_scope is now `child`

        self.current_scope.parent = Some(Box::new(parent));

        self.current_scope.id
    }

    pub fn escape_scope(&mut self) {
        if let Some(parent) = self.current_scope.parent.clone() {
            self.current_scope = *parent;
        } else {
            panic!("Cannot escape root scope");
        }
    }

    pub fn find_ident(&self, ident_str: &IdentName) -> Option<&Identifiable> {
        let mut current_scope = Some(&self.current_scope);

        while let Some(scope) = current_scope {
            current_scope = scope.parent.as_ref().map(|parent| &**parent);

            if let Some(ident) = scope.idents.get(ident_str) {
                return Some(ident);
            }
        }

        None
    }

    pub fn find_ident_mut(&mut self, ident_str: &IdentName) -> Option<&mut Identifiable> {
        let mut current_scope = Some(&mut self.current_scope);

        while let Some(scope) = current_scope {
            current_scope = scope.parent.as_mut().map(|parent| &mut **parent);

            if let Some(ident) = scope.idents.get_mut(ident_str) {
                return Some(ident);
            }
        }

        None
    }

    pub fn add_ident(&mut self, ident: Identifiable) {
        let previous = self
            .current_scope
            .idents
            .insert(*ident.ident_bytes(), ident);

        if previous.is_some() {
            panic!("Identifier already exists in current scope");
        }
    }

    pub fn current_scope(&self) -> &Scope {
        &self.current_scope
    }

    pub fn add_to_tree(&mut self, root: TreeItem) {
        self.current_scope.tree.push(root);
    }
}

#[derive(Clone, Debug)]
pub enum TreeItem {
    Structure(Structure),
    Impl(Implementation),
    Enum(Enumeration),
    Function(Function),
    Trait(Trait),
    Variable(Variable),
    Return(Option<Expression>),
    Break,
    Continue,
    Assignment(Assignment),
    Expression(Expression),
    If(IfStatement),
    While(WhileLoop),
    For(ForLoop),
    Match(MatchStatement),
}
