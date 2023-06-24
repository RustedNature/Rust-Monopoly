use super::FieldType;

#[derive(Clone, PartialEq, Eq)]
pub struct GoToJail {
    name: String,
}

impl GoToJail {
    pub fn new() -> GoToJail {
        GoToJail {
            name: "Gehe ins Gefängnis".to_string(),
        }
    }
}

impl Default for GoToJail {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldType for GoToJail {
    fn get_name(&self) -> &str {
        &self.name
    }
}
//TODO
