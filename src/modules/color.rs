use std::hash::Hash;

pub enum Color {
    Brown,
    Pink,
    LightBlue,
    Orange,
    Red,
    Yellow,
    Green,
    DarkBlue,
    TrainStation,
    Utility,
}

impl Eq for Color {}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
