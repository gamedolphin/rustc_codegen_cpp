use super::{project::Project, typ::Type};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Constant {
    pub typ: Type,
    pub value: Vec<u8>,
    pub size: u64,
    pub deps: Vec<String>,
}
