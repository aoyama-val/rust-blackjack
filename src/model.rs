use rand::prelude::*;
use std::time;

pub const FPS: i32 = 30;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
    None,
    Hit,
    Stand,
}

pub struct Card {
    pub id: i32, // [1,52]の整数。0始まりがベターだと思うが、cardXX.pngが1始まりだったので。
}

impl Card {
    pub fn new(id: i32) -> Card {
        assert!(1 <= id && id <= 52);
        Card { id: id }
    }

    pub fn number(&self) -> i32 {
        self.id % 13
    }

    pub fn mark(&self) -> i32 {
        (self.id - 1) / 13
    }
}

pub struct Game {
    pub rng: StdRng,
    pub is_debug: bool,
    pub is_over: bool,
    pub is_clear: bool,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub deck: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        let now = time::SystemTime::now();
        let timestamp = now
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        let rng = StdRng::seed_from_u64(timestamp);
        println!("random seed = {}", timestamp);
        // let rng = StdRng::seed_from_u64(0);

        let game = Game {
            rng: rng,
            is_debug: false,
            is_over: false,
            is_clear: false,
            player_cards: Vec::new(),
            dealer_cards: Vec::new(),
            deck: Vec::new(),
        };

        game
    }

    pub fn init(&mut self) {
        let mut ids: Vec<i32> = (1..=52).collect();
        ids.shuffle(&mut self.rng);
        println!("{:?}", ids);

        for id in ids {
            self.deck.push(Card::new(id));
        }

        self.dealer_cards.push(self.deck.pop().unwrap());

        for _ in 0..2 {
            self.player_cards.push(self.deck.pop().unwrap());
        }
        println!("desk.len = {}", self.deck.len());
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over || self.is_clear {
            return;
        }

        match command {
            Command::Hit => {
                self.hit();
            }
            Command::Stand => {
                self.stand();
            }
            Command::None => {}
        }
    }

    pub fn hit(&mut self) {
        println!("Hit");
        self.player_cards.push(self.deck.pop().unwrap());
    }

    pub fn stand(&mut self) {
        println!("Stand");
    }

    pub fn calc_point(&self, cards: &Vec<Card>) -> i32 {
        let mut point = 0;
        for card in cards {
            point += card.number();
        }
        point
    }
}
