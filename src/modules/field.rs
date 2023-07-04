use std::{any::Any, hash::Hash};

pub mod board_fields;
pub mod chance;
pub mod community_chest;
pub mod free_parking;
pub mod general_field;
pub mod go_to_jail;
pub mod jail_visiting;
pub mod rail_road;
mod rental_priceses;
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

pub trait Field {
    fn get_name(&self) -> &str;
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Buyable {
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

pub enum NonBuyable {
    Go,
    Tax,
    JailVisiting,
    CommunityChest,
    Chance,
    GoToJail,
    FreeParking,
}
