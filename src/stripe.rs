use std::fmt::{Display, Formatter};
use crossterm::style::Stylize;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;

#[derive(Debug, Copy, Clone)]
pub enum Stripe {
    Empty,
    Green(GreenStripe),
    Rail(Railroad),
}

impl Stripe {
    pub fn generate() -> Self {
        let dist = WeightedIndex::new([5, 3]).unwrap();
        match dist.sample(&mut rand::rng()) {
            0 => Stripe::Green(GreenStripe::generate()),
            1 => Stripe::Rail(Railroad::generate()),
            _ => panic!("Weighted index out of expected range"),
        }

    }

    pub fn update(&mut self) {
        match self {
            Stripe::Empty => {},
            Stripe::Green(stripe) => stripe.update(),
            Stripe::Rail(stripe) => stripe.update(),
        }
    }

    pub fn collides(&self, x: u8) -> bool {
        match self {
            Stripe::Empty => false,
            Stripe::Green(stripe) => stripe.collides(x),
            Stripe::Rail(stripe) => stripe.collides(x),
        }
    }

    pub fn visualize(&self) -> Vec<Block> {
        match self {
            Stripe::Empty => vec![], // Uninitialized
            Stripe::Green(stripe) => stripe.visualize(),
            Stripe::Rail(stripe) => stripe.visualize(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GreenStripe {
    trees: [bool; 7],
}

impl GreenStripe {
    fn generate() -> Self {
        let mut trees = [false; 7];
        rand::fill(&mut trees);
        trees[3] = false;
        GreenStripe { trees }
    }

    fn update(&mut self) {}

    fn collides(&self, x: u8) -> bool {
        self.trees[x as usize]
    }

    fn visualize(&self) -> Vec<Block> {
        let tree = Block::Green;
        let grass = Block::BrightGreen;
        self.trees
            .map(|t| if t { tree } else { grass })
            .to_vec()
    }
}

/// Railroads are deadly as a whole.
///
/// [cycle_pos] is initialized to cycle length and counts downward.
/// - On values 0..3 it is deadly
/// - On values 3..8 it warns
#[derive(Debug, Copy, Clone)]
pub struct Railroad {
    cycle_length: usize,
    cycle_pos: usize,
}

impl Railroad {
    fn generate() -> Self {
        let cycle_length = rand::random_range(20..50);
        Railroad {
            cycle_length,
            cycle_pos: cycle_length,
        }
    }

    fn update(&mut self) {
        if self.cycle_pos == 0 {
            self.cycle_pos = self.cycle_length;
        } else {
            self.cycle_pos -= 1;
        }
    }

    fn collides(&self, _x: u8) -> bool {
        self.cycle_pos < 3
    }

    fn visualize(&self) -> Vec<Block> {
        match self.cycle_pos {
            0..3 => [Block::Red; 7],
            3..8 => [Block::DarkYellow; 7],
            _ => [Block::Gray; 7],
        }.to_vec()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Block {
    Green,
    BrightGreen,
    White,
    Gray,
    DarkYellow,
    Red,
    Black,
}

impl Block {
    fn color_coded(&self) -> String {
        match self {
            Block::Green => "███".dark_green().to_string(),
            Block::BrightGreen => "███".green().to_string(),
            Block::White => "███".white().to_string(),
            Block::Gray => "███".dark_grey().to_string(),
            Block::DarkYellow => "███".dark_yellow().to_string(),
            Block::Red => "███".red().to_string(),
            Block::Black => "███".black().to_string(),
        }
    }
}

pub trait Render {
    fn render(&self) -> String;
}

impl Render for Vec<Block> {
    fn render(&self) -> String {
        let mut res = String::new();
        for block in self {
            res.push_str(&block.color_coded());
        }
        res
    }
}