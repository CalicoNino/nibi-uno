use crate::{
    error::ContractError,
    helpers::create_initial_deck,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Card, Player, State, CONFIG},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

// Implement the entry point for initializing the contract
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Initialize the game state with the joining fee and fee recipient address
    let mut state = State {
        deck: create_initial_deck(),
        discard_pile: vec![],
        players: vec![],
        current_turn: 0,
        direction: 1,
        game_started: false,
        game_over: false,
        winner: None,
    };

    // Add the sender of the instantiate message as the first player
    let player = Player {
        address: info.sender.clone(),
        hand: vec![],
    };
    state.players.push(player);

    // Save the initial state to storage
    CONFIG.save(deps.storage, &state)?;

    // Return a successful response with attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("player", info.sender))
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        GetGameState {} => to_json_binary(&query::get_game_state(deps)?),
        GetPlayerHand { address } => to_json_binary(&query::get_player_hand(deps, address)?),
    }
}

mod query {
    use super::*;

    use crate::msg::{GameStateResp, PlayerHandResp};
    use cosmwasm_std::{Addr, StdError};

    pub fn get_game_state(deps: Deps) -> StdResult<GameStateResp> {
        let state = crate::state::CONFIG.load(deps.storage)?;

        Ok(GameStateResp {
            players: state.players,
            current_turn: state.current_turn,
            direction: state.direction,
            game_started: state.game_started,
            game_over: state.game_over,
            winner: state.winner,
        })
    }

    pub fn get_player_hand(deps: Deps, address: Addr) -> StdResult<PlayerHandResp> {
        let state = crate::state::CONFIG.load(deps.storage)?;
        let player = state.players.iter().find(|p| p.address == address);

        if let Some(player) = player {
            // Clone the player's hand directly
            let hand = player.hand.clone();

            let response = PlayerHandResp {
                hand,
                player: address,
            };

            Ok(response)
        } else {
            Err(StdError::generic_err("Player not found"))
        }
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        JoinGame {} => execute::join_game(deps, info),
        DrawCard {} => execute::draw_card(deps, info),
        LeaveGame {} => execute::leave_game(deps, info),
        PlayCard { card } => execute::play_card(deps, info, card),
    }
}

mod execute {
    use super::*;

    use crate::error::ContractError::Std;
    use crate::state::Player;
    use cosmwasm_std::StdError;

    pub fn join_game(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Load the current game state from storage
        let mut state = crate::state::CONFIG.load(deps.storage)?;

        // Ensure the game has not already started and there is room for more players
        if state.players.len() >= 2 {
            return Err(Std(StdError::generic_err("Game already has four players")));
        }

        // Ensure the player has not already joined the game
        if state.players.iter().any(|p| p.address == info.sender) {
            return Err(Std(StdError::generic_err("Player already joined")));
        }

        let joining_player_address = info.sender;

        // Add the player to the game with an empty hand
        state.players.push(Player {
            address: joining_player_address.clone(),
            hand: vec![],
        });

        // Start the game if four players have joined
        if state.players.len() == 2 {
            state.game_started = true;

            // Distribute 7 cards to each player
            for player in state.players.iter_mut() {
                for _ in 0..7 {
                    if let Some(card) = state.deck.pop() {
                        player.hand.push(card);
                    } else {
                        // Handle the case when the deck runs out of cards
                        // You may want to abort the game or handle it differently based on your requirements
                    }
                }
            }
        }
        // Save the updated game state to storage
        crate::state::CONFIG.save(deps.storage, &state)?;

        // Return a successful response with attributes
        Ok(Response::new()
            .add_attribute("method", "join_game")
            .add_attribute("player", joining_player_address.clone()))
    }

    pub fn leave_game(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Load the current game state from storage
        let mut state = crate::state::CONFIG.load(deps.storage)?;

        // Check if the player is in the game
        let player_index = state.players.iter().position(|p| p.address == info.sender);

        if let Some(index) = player_index {
            // Remove the player from the game
            state.players.remove(index);

            // Update the game state
            if state.players.is_empty() {
                // Reset the game state if no players are left
                state.game_over = true;
            } else if state.current_turn >= state.players.len() {
                // Adjust the current turn index if necessary
                state.current_turn = state.current_turn % state.players.len();
            }

            // Save the updated game state to storage
            crate::state::CONFIG.save(deps.storage, &state)?;

            // Return a successful response
            Ok(Response::new()
                .add_attribute("method", "leave_game")
                .add_attribute("player", info.sender))
        } else {
            // Return an error if the player is not in the game
            Err(Std(StdError::generic_err("Player not found in the game")))
        }
    }

    pub fn draw_card(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Load the current game state from storage
        let mut state = crate::state::CONFIG.load(deps.storage)?;

        // Ensure the game has started
        if !state.game_started {
            return Err(Std(StdError::generic_err("Game not started yet")));
        }

        // Ensure it is the player's turn
        let player_index = state.players.iter().position(|p| p.address == info.sender);
        if player_index.is_none() || state.players[state.current_turn].address != info.sender {
            return Err(Std(StdError::generic_err("Not your turn")));
        }

        // Draw a card for the player
        let card = state
            .deck
            .pop()
            .ok_or_else(|| cosmwasm_std::StdError::generic_err("Deck is empty"))?;
        let current_player_index = state.current_turn;
        let mut current_player = state.players[current_player_index].clone();
        current_player.hand.push(card);
        state.players[current_player_index] = current_player;

        // Save the updated game state to storage
        crate::state::CONFIG.save(deps.storage, &state)?;

        // Return a successful response
        Ok(Response::new()
            .add_attribute("method", "draw_card")
            .add_attribute("player", info.sender))
    }

    pub fn play_card(
        deps: DepsMut,
        info: MessageInfo,
        card: Card,
    ) -> Result<Response, ContractError> {
        // Load the current game state from storage
        let mut state = crate::state::CONFIG.load(deps.storage)?;

        // Ensure the game has started
        if !state.game_started {
            return Err(Std(StdError::generic_err("Game not started yet")));
        }

        // Ensure it is the player's turn
        let player_index = state.players.iter().position(|p| p.address == info.sender);
        if player_index.is_none() || state.players[state.current_turn].address != info.sender {
            return Err(Std(StdError::generic_err("Not your turn")));
        }

        // Check if the player has the card in their hand
        let current_player_index = state.current_turn;
        let current_player = &mut state.players[current_player_index];
        let card_index = current_player.hand.iter().position(|c| *c == card);
        if card_index.is_none() {
            return Err(Std(StdError::generic_err("Card not found in hand")));
        }
        let card_index = card_index.unwrap();

        // Check if the played card is valid
        let played_card = &current_player.hand[card_index];

        // Remove the card from the player's hand and add it to the discard pile
        let card = current_player.hand.remove(card_index);
        state.discard_pile.push(card);

        // // Update the game state
        // state.current_turn =
        //     (state.current_turn + state.direction + state.players.len()) % state.players.len();

        // Save the updated game state to storage
        crate::state::CONFIG.save(deps.storage, &state)?;

        // Return a successful response
        Ok(Response::new()
            .add_attribute("method", "play_card")
            .add_attribute("player", info.sender)
            .add_attribute("message", "Card played successfully"))
    }
}
