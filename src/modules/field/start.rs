use super::FieldType;

#[derive(Clone, PartialEq, Eq)]
pub struct Start {
    name: String,
}

impl Start {
    pub fn new() -> Start {
        Start {
            name: "Startfeld".to_string(),
        }
    }
}

impl Default for Start {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldType for Start {
    fn get_name(&self) -> &str {
        &self.name
    }
}

//TODO
