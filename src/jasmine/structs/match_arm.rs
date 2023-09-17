use super::*;

#[derive(Clone, Debug, Hash)]
pub struct MatchArm {
    pub enum_name: String,
    pub variant: String,
    pub data: Option<(String, InferredType)>,
}
