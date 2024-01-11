use rand::prelude::*;
use std::{num, time};

pub const FPS: i32 = 30;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameResult {
    None,
    Bust,
    Win,
    Lose,
    Push,
}

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
    pub is_over: bool,
    pub result: GameResult,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub deck: Vec<Card>,
    pub win_count: i32,
    pub lose_count: i32,
    pub requested_sounds: Vec<&'static str>,
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
            is_over: false,
            result: GameResult::None,
            player_cards: Vec::new(),
            dealer_cards: Vec::new(),
            deck: Vec::new(),
            win_count: 0,
            lose_count: 0,
            requested_sounds: Vec::new(),
        };

        game
    }

    pub fn init(&mut self) {
        let mut ids: Vec<i32> = (1..=52).collect();
        ids.shuffle(&mut self.rng);

        for id in ids {
            self.deck.push(Card::new(id));
        }

        self.dealer_cards.push(self.deck.pop().unwrap());

        for _ in 0..2 {
            self.player_cards.push(self.deck.pop().unwrap());
        }
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over {
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
        self.player_cards.push(self.deck.pop().unwrap());
        if self.calc_point(&self.player_cards) > 21 {
            self.is_over = true;
            self.lose_count += 1;
            self.result = GameResult::Bust;
            self.requested_sounds.push("crash.wav");
        }
    }

    pub fn stand(&mut self) {
        self.decide_result();
    }

    pub fn decide_result(&mut self) {
        let player_point = self.calc_point(&self.player_cards);
        let dealer_point = self.calc_point(&self.dealer_cards);
        if player_point > dealer_point {
            self.result = GameResult::Win;
            self.win_count += 1;
            self.requested_sounds.push("clear.wav");
        } else if player_point < dealer_point {
            self.result = GameResult::Lose;
            self.lose_count += 1;
            self.requested_sounds.push("crash.wav");
        } else {
            self.result = GameResult::Push;
        }
        self.is_over = true;
    }

    pub fn calc_point(&self, cards: &Vec<Card>) -> i32 {
        let mut point = 0;
        for card in cards {
            let number = card.number();
            if number == 1 {
                point += 11;
            } else if 2 <= number && number <= 10 {
                point += number;
            } else {
                point += 10;
            }
        }
        point
    }
}
