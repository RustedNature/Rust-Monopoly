use std::hash::Hash;

mod buyable_field;
mod general_field;

use buyable_field::BuyableField;
use general_field::GeneralField;
pub enum Field {
    General(GeneralField),
    Buyable(BuyableField),
}

impl Eq for Field {}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
