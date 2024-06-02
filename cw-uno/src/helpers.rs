use crate::state::Card;

// Function to create the initial deck of cards
pub fn create_initial_deck() -> Vec<Card> {
    let mut deck = vec![];
    let colors = vec!["red", "yellow", "blue", "green"];

    for color in colors.iter() {
        for number in 1..=10 {
            deck.push(Card {
                color: color.to_string(),
                number,
            });
        }
        for _ in 0..2 {
            deck.push(Card {
                color: color.to_string(),
                number: -1,
            }); // Block
            deck.push(Card {
                color: color.to_string(),
                number: -2,
            }); // +2
            deck.push(Card {
                color: color.to_string(),
                number: -3,
            }); // Reverse
        }
    }
    for _ in 0..4 {
        deck.push(Card {
            color: "wild".to_string(),
            number: -4,
        }); // Change color
        deck.push(Card {
            color: "wild".to_string(),
            number: -5,
        }); // +4 wild
    }

    deck
}
