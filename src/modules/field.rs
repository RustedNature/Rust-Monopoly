use crate::modules::buyable_field::BuyableField;
use crate::modules::general_field::GeneralField;

pub enum Field {
    General(GeneralField),
    Buyable(BuyableField),
}
