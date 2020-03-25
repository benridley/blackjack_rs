use std::io;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut dollars = 500;
    println!("Welcome to blackjack");
    while dollars > 0 {
        println!("Enter bet (You have {} dollars).", dollars);
        let bet: i32;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim().parse::<i32>() {
            Ok(i) if i <= dollars => bet = i,
            _ => {
                println!("Invalid bet!");
                continue
            }
        }
        let result = play_round();
        match result {
            GameResult::Win => {
                println!("You won!");
                dollars += bet;
            }
            GameResult::Draw => println!("You drew!"),
            GameResult::Lose => {
                println!("You lost!");
                dollars -= bet;
            }
        }
    }
    println!("Game over!")
}

// Play round returns a true if player wins, false if player loses. 
fn play_round() -> GameResult {
    let mut deck = CardSet::new_shuffled();
    let mut player_hand = CardSet::new();
    let mut dealer_hand = CardSet::new();

    for _ in 0..2 {
        player_hand.push(deck.pop().unwrap());
    }
    dealer_hand.push(deck.pop().unwrap());

    let mut player_wins = false;
    let mut dealer_wins = false;

    while !dealer_wins && !player_wins {
        println!("Your hand:");
        player_hand.print_hand();
        println!();
        println!("Dealers hand:");
        dealer_hand.print_hand();
        
        let desired_action = match player_hand.has_blackjack() {
            true => {
                println!("Player blackjack!");
                player_wins = true;
                Some(Action::Stand)
            },
            false => get_player_action(),
        };

        match desired_action {
            Some(Action::Hit) => {
                player_hand.push(deck.pop().unwrap());
                if player_hand.get_value_optimum() > 21 {
                    dealer_wins = true;
                }
            }
            Some(Action::Stand) => {
                while !dealer_wins && !player_wins {
                    dealer_hand.push(deck.pop().unwrap());
                    println!("Dealer draws {}", dealer_hand.0.last().unwrap());
                    if dealer_hand.has_blackjack() {
                        println!("Dealer blackjack!");
                        dealer_wins = true;
                    } else if dealer_hand.get_value_optimum() <= 21 && dealer_hand.get_value_optimum() > player_hand.get_value_optimum(){
                        dealer_wins = true;
                    } else if dealer_hand.get_value_optimum() > 21 {
                        player_wins = true;
                    }
                }
            }
            None => {
                println!("Invalid action. Try again.");
                continue;
            }
        }
    }
    match player_wins {
        true if dealer_wins => GameResult::Draw,
        true  => GameResult::Win,
        false => GameResult::Lose,
    }
}

fn get_player_action() -> Option::<Action> {
    println!("Choose an action (Hit, Stand):");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().to_ascii_lowercase().as_str() {
        "hit" => Some(Action::Hit),
        "stand" => Some(Action::Stand),
        _ => None,
    }
}

enum GameResult {
    Win, Draw, Lose
}

enum Action {
    Hit, Stand
}
#[derive(Debug)]
enum Suit {
    Clubs, Diamonds, Hearts, Spades,
}

#[derive(Debug)]
enum Value {
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    fn from_value(s: Suit, v: i32) -> Card {
        let cv = match v {
            1 => Value::Ace,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            _  => Value::Ace,
        };
        Card { suit: s, value: cv}
    }

    fn to_value_aces_low(&self) -> i32{
        match &self.value {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
    fn to_value_aces_high(&self) -> i32 {
        match &self.value {
            Value::Ace => 11,
            _ => self.to_value_aces_low()
        }
    }

    fn get_value_string(&self) -> &str {
        match self.value {
            Value::Ace => "Ace",
            Value::Two => "Two",
            Value::Three => "Three",
            Value::Four => "Four",
            Value::Five => "Five",
            Value::Six => "Six",
            Value::Seven => "Seven",
            Value::Eight => "Eight",
            Value::Nine => "Nine",
            Value::Ten => "Ten",
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
        }
    }

    fn get_suit_string(&self) -> &str {
        match self.suit {
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.get_value_string(), self.get_suit_string())
    }
}

#[derive(Debug)]
struct CardSet(Vec<Card>);

impl CardSet {
    fn new() -> CardSet {
        CardSet(Vec::<Card>::new())
    }

    fn push(&mut self, c: Card) {
        self.0.push(c)
    }

    fn pop(&mut self) -> std::option::Option::<Card>{
        self.0.pop()
    }

    fn deck() -> CardSet {
        let mut cards = Vec::<Card>::with_capacity(52);
        for i in 1..=4 {
            for j in 1..=13 {
                let s = match i {
                    1 => Suit::Clubs,
                    2 => Suit::Diamonds,
                    3 => Suit::Hearts,
                    4 => Suit::Spades,
                    _ => Suit::Spades
                };
                cards.push(Card::from_value(s, j));
            }
        }
        CardSet(cards)
    }

    fn new_shuffled() -> CardSet {
        let mut cards = CardSet::deck();
        let mut rng = thread_rng();
        cards.0.shuffle(&mut rng);
        cards
    }

    fn get_value_aces_low(&self) -> i32 {
        self.0.iter().fold(0, |acc, s| acc + s.to_value_aces_low())
    }

    fn get_value_aces_high(&self) -> i32 {
        self.0.iter().fold(0, |acc, s| acc + s.to_value_aces_high())
    }

    // Picks the maximum value (aces high or low) depending on whether the highest score is valid or not
    fn get_value_optimum(&self) -> i32 {
        if self.get_value_aces_high() <= 21 {
            return self.get_value_aces_high();
        }
        self.get_value_aces_low()
    }

    fn has_blackjack(&self) -> bool {
        self.0.len() == 2 && self.get_value_aces_high() == 21
    }

    fn print_hand(&self) {
        self.0.iter().for_each(|card| println!{"{}", card});
        println!("Value: {}", self.get_value_optimum());
    }
}