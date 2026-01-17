#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
}

impl TypeAnnotation {
    pub fn from_identifier(name: &str) -> Option<Self> {
        match name {
            "int" => Some(TypeAnnotation::IntegerType),
            "float" => Some(TypeAnnotation::FloatType),
            "bool" => Some(TypeAnnotation::BooleanType),
            "string" => Some(TypeAnnotation::StringType),
            _ => None,
        }
    }
}
