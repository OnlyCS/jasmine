use super::*;

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub argument: (Identifier, InferredType),
    pub iterator: Expression,
}

#[derive(Clone, Debug)]
pub struct WhileLoop(Expression);

#[derive(Clone, Debug)]
pub enum IfCondition {
    IfLet {
        enum_type: Identifier,
        variant_ident: Identifier,
        data_ident: Identifier,
        data_type: InferredType,
        expression: Expression,
    },
    BooleanExpression(Expression),
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: IfCondition,
    pub else_ifs: Vec<IfCondition>, // order matters, cannot hashset :(
    pub has_else: bool,
}

#[derive(Clone, Debug, Hash)]
pub struct MatchArm {
    pub type_ident: Identifier,
    pub variant: Identifier,
    pub data: Option<(Identifier, InferredType)>,
}

#[derive(Clone, Debug)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: HashSet<MatchArm>, // only used for enums, and no extra if statements/or gates
}
