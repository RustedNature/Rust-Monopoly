use super::FieldType;

#[derive(Clone, PartialEq, Eq)]

pub struct FreeParking {
    name: String,
}

impl FreeParking {
    pub fn new() -> FreeParking {
        FreeParking {
            name: "Frei Parken".to_string(),
        }
    }
}

impl Default for FreeParking {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldType for FreeParking {
    fn get_name(&self) -> &str {
        &self.name
    }
}
//TODO
