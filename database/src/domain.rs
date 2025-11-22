#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_str: String,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Field {
    #[allow(dead_code)]
    fn new(name: String, type_str: String) -> Self {
        Self { name, type_str }
    }
}