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
//TODO
