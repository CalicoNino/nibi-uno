use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{Card, Player};

// Define the InstantiateMsg struct, used for instantiating the contract
#[cw_serde]
pub struct InstantiateMsg {}

// Define the QueryMsg enum, representing different queryable messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GameStateResp)]
    GetGameState {},
    #[returns(PlayerHandResp)]
    GetPlayerHand { address: Addr },
}

#[cw_serde]
pub struct GameStateResp {
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub direction: i8,
    pub game_started: bool,
    pub game_over: bool,
    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct PlayerHandResp {
    pub hand: Vec<Card>,
    pub player: Addr,
}

// Define the ExecuteMsg enum, representing different executable messages
#[cw_serde]
pub enum ExecuteMsg {
    JoinGame {},
    DrawCard {},
    LeaveGame {},
    PlayCard { card: Card },
}

// Define the Coin struct for representing monetary amounts
#[cw_serde]
pub struct Coin {
    pub denom: String,
    pub amount: Uint128,
}

// Define the Uint128 struct for representing 128-bit unsigned integers
#[cw_serde]
pub struct Uint128(pub u128);
