use std::fmt::{Display, Formatter};
use crossterm::style::Stylize;

#[derive(Debug, Copy, Clone)]
pub enum Stripe {
    Empty,
    Green(GreenStripe),
}

impl Stripe {
    pub fn generate() -> Self {
        Stripe::Green(GreenStripe::generate())
    }

    pub fn update(&mut self) {
        match self {
            Stripe::Empty => {},
            Stripe::Green(stripe) => stripe.update(),
        }
    }

    pub fn collides(&self, x: u8) -> bool {
        match self {
            Stripe::Empty => false,
            Stripe::Green(stripe) => stripe.collides(x),
        }
    }

    pub fn visualize(&self) -> Vec<Block> {
        match self {
            Stripe::Empty => vec![], // Uninitialized
            Stripe::Green(stripe) => stripe.visualize(),
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Block {
    Green,
    BrightGreen,
    White,
}

impl Block {
    fn color_coded(&self) -> String {
        match self {
            Block::Green => "███".dark_green().to_string(),
            Block::BrightGreen => "███".green().to_string(),
            Block::White => "███".white().to_string(),
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