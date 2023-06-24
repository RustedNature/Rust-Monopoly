use super::FieldType;

#[derive(Clone, PartialEq, Eq)]

pub struct CommunityChest {
    name: String,
}

impl CommunityChest {
    pub fn new() -> CommunityChest {
        CommunityChest {
            name: "Gemeinschaftsfeld".to_string(),
        }
    }
}

impl Default for CommunityChest {
    fn default() -> Self {
        Self::new()
    }
}

//TODO
impl FieldType for CommunityChest {
    fn get_name(&self) -> &str {
        &self.name
    }
}
