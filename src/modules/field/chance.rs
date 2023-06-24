use super::FieldType;

#[derive(Clone, PartialEq, Eq)]
pub struct Chance {
    name: String,
}
impl Chance {
    pub fn new() -> Chance {
        Chance {
            name: "Ereignisfeld".to_string(),
        }
    }
}

impl Default for Chance {
    fn default() -> Self {
        Self::new()
    }
}
impl FieldType for Chance {
    fn get_name(&self) -> &str {
        &self.name
    }
}

//TODO
