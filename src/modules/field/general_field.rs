use super::FieldType;

#[derive(Clone, PartialEq, Eq)]

pub struct General {
    name: String,
}

impl General {
    pub fn new() -> General {
        General {
            name: "()".to_string(),
        }
    }
}

impl Default for General {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldType for General {
    fn get_name(&self) -> &str {
        &self.name
    }
}

//TODO
