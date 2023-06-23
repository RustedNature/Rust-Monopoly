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
//TODO
