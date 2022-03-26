use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub enum TileType {
    Blockade {},
    DamageInducer { damage: i32 },
    Normal {}, 
}

pub enum AbilityType {
    Fire {},
    Electric {},
    Explosive {},
    Blunt {},
    Sharp {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tile {
    pub tiletype: TileType,
    pub moveable: bool,
    pub damage: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Board {
    pub owner: Addr,
    pub mut layout: [[Tile{TileType:Normal{}, moveable: true, damage:0}; 8]; 8];,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Player {
    pub user: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Character {
    pub id: i32,
    pub health: i32,
    pub abilities: [AbilityType; 2],
    pub pos-x: i8,
    pub pos-y: i8,
}

pub const STATE: Item<State> = Item::new("state");
pub const CHARACTER: Item<Character> = Item::new("character");
pub const BOARD: Item<Board> = Item::new("board");
pub const TILE: Item<Tile> = Item::new("tile");
