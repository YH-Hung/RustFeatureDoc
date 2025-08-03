use rand::{rng, seq::SliceRandom};

#[derive(Debug)]    // Compiler will automatically provide Debug implementation
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // A normal function in rust, use new just for convention.
    fn new() -> Self {
        // suits
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

        // values
        let values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];

        // generate combination
        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Implicit return last expression.
        Deck { cards }
    }

    // Rust compiler detects mutation even for use-defined function.
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    // Rust compiler detects mutation even for use-defined function.
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    
    println!("Here is your hand: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);
}
