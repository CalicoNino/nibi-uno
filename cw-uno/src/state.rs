use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

// Define the Card struct, representing a card in the game
#[cw_serde]
pub struct Card {
    pub color: String,
    pub number: i8, // Using i8 to handle special cards with negative numbers
}

// Define the Player struct, representing a player in the game
#[cw_serde]
pub struct Player {
    pub address: Addr,
    pub hand: Vec<Card>,
}

// Define the State struct, representing the overall state of the game
#[cw_serde]
pub struct State {
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub direction: i8,
    pub game_started: bool,
    pub game_over: bool,
    pub winner: Option<Addr>,
}

// Define the storage items for the contract's state
pub const CONFIG: Item<State> = Item::new("config");
pub const PLAYER_STORAGE: Map<String, Player> = Map::new("players");

// Function to store the state in storage
pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    CONFIG.save(storage, state)
}

// Function to load the state from storage
pub fn load_state(storage: &dyn Storage) -> StdResult<State> {
    CONFIG.load(storage)
}
