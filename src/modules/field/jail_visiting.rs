use super::FieldType;

#[derive(Clone, PartialEq, Eq)]
pub struct JailVisiting {
    name: String,
}

impl JailVisiting {
    pub fn new(name: String) -> JailVisiting {
        JailVisiting { name }
    }
}

impl FieldType for JailVisiting {
    fn get_name(&self) -> &str {
        &self.name
    }
}
//TODO
