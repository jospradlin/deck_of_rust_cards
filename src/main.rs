use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        // List of 'suits'
        let suits = ["H", "S", "D", "C"];
        // List of 'values'
        let values = ["2", "3", "4", "5", "6" ,"7", "8", "9", "10", "J", "Q", "K", "A"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck{ cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {



    let mut deck = Deck::new();

    //deck.shuffle();

    // Need to add error handling... this is bad since it doesn't check input.
    let cards = deck.deal(3);

    println!("Here's your cards: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck);
}


