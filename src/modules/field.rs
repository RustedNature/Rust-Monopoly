use std::hash::Hash;
pub mod board_fields;
pub mod chance;
pub mod community_chest;
pub mod free_parking;
pub mod general_field;
pub mod go_to_jail;
pub mod jail_visiting;
pub mod rail_road;
pub mod start;
pub mod street;
pub mod tax;
pub mod utility;

use chance::Chance;
use community_chest::CommunityChest;
use free_parking::FreeParking;
use general_field::General;
use go_to_jail::GoToJail;
use jail_visiting::JailVisiting;
use rail_road::RailRoad;
use start::Start;
use street::Street;
use tax::Tax;
use utility::Utility;

pub enum Field {
    Chance(Chance),
    CommunityChest(CommunityChest),
    FreeParking(FreeParking),
    General(General),
    GoToJail(GoToJail),
    JailVisiting(JailVisiting),
    RailRoad(RailRoad),
    Start(Start),
    Street(Street),
    Tax(Tax),
    Utility(Utility),
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
