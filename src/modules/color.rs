use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Clone)]
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
