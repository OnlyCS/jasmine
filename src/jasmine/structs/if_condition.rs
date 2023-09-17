use super::*;

#[derive(Clone, Debug)]
pub enum IfCondition {
    IfLet {
        enum_name: String,
        variant_ident: String,
        data_ident: String,
        data_type: InferredType,
        expression: Expression,
    },
    BooleanExpression(Expression),
}
