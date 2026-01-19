use std::ops::Div;
use crossterm::style::Stylize;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;
use rayon::prelude::*;

// TODO: add 2 for padding, allowing to display more from the side
pub const STRIPE_LENGTH: usize = 7;

const TILE_WIDTH: usize = 3;

#[derive(Debug, Copy, Clone)]
pub enum Stripe {
    Empty,
    Green(GreenStripe),
    Rail(Railroad),
    Road(Road),
}

impl Stripe {
    pub fn generate() -> Self {
        let dist = WeightedIndex::new([5, 3, 5]).unwrap();
        match dist.sample(&mut rand::rng()) {
            0 => Stripe::Green(GreenStripe::generate()),
            1 => Stripe::Rail(Railroad::generate()),
            2 => Stripe::Road(Road::generate()),
            _ => panic!("Weighted index out of expected range"),
        }

    }

    pub fn update(&mut self) {
        match self {
            Stripe::Empty => {},
            Stripe::Green(stripe) => stripe.update(),
            Stripe::Rail(stripe) => stripe.update(),
            Stripe::Road(stripe) => stripe.update(),
        }
    }

    pub fn collides(&self, x: u8) -> bool {
        match self {
            Stripe::Empty => false,
            Stripe::Green(stripe) => stripe.collides(x),
            Stripe::Rail(stripe) => stripe.collides(x),
            Stripe::Road(stripe) => stripe.collides(x),
        }
    }

    pub fn visualize(&self) -> StripeRender {
        match self {
            Stripe::Empty => StripeRender::default(),
            Stripe::Green(stripe) => stripe.visualize(),
            Stripe::Rail(stripe) => stripe.visualize(),
            Stripe::Road(stripe) => stripe.visualize(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GreenStripe {
    trees: [bool; STRIPE_LENGTH],
}

impl GreenStripe {
    pub fn generate() -> Self {
        let mut trees = [false; STRIPE_LENGTH];
        rand::fill(&mut trees);
        trees[STRIPE_LENGTH.div(2)] = false;
        GreenStripe { trees }
    }

    fn update(&mut self) {}

    fn collides(&self, x: u8) -> bool {
        self.trees[x as usize]
    }

    fn visualize(&self) -> StripeRender {
        let tree = Block::Green;
        let grass = Block::BrightGreen;
        let blocks: [Block; STRIPE_LENGTH] = core::array::from_fn(|i| {
            if self.trees[i] { tree } else { grass }
        });
        StripeRender::new(blocks, None)
    }
}

/// Railroads are deadly as a whole.
///
/// [cycle_pos] is initialized to cycle length and counts downward.
/// - On values 0..3 it is deadly
/// - On values 3..12 it warns
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

    fn visualize(&self) -> StripeRender {
        let blocks = match self.cycle_pos {
            0..3 => [Block::Red; STRIPE_LENGTH],
            3..12 => [Block::DarkYellow; STRIPE_LENGTH],
            _ => [Block::Gray; STRIPE_LENGTH],
        };
        StripeRender::new(blocks, None)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Road {
    cars: [bool; STRIPE_LENGTH],
    left: bool,
    current_car_len: i32,
    /// Cycles in 0..=2.
    offset: usize,
}

impl Road {
    fn generate() -> Self {
        let mut road = Road {
            cars: [false; STRIPE_LENGTH],
            current_car_len: 0,
            offset: 0,
            left: rand::random(),
        };
        for _ in 0..STRIPE_LENGTH {
            road.advance_road();
        }

        road
    }

    fn update(&mut self) {
        self.offset += 1;
        self.offset %= TILE_WIDTH;
        if self.offset == 0 {
            self.advance_road();
        }
    }

    fn advance_road(&mut self) {
        let new_tile = match self.current_car_len {
            -1..=0 => false, // 2 tiles space between cars
            ..-1 => rand::random(),
            1 => true,
            2 => rand::random(),
            3.. => false,
        };
        if new_tile {
            self.current_car_len = (self.current_car_len + 1).max(1);
        } else { // The car ended
            if self.current_car_len > 1 {
                self.current_car_len = 1;
            }
            self.current_car_len -= 1;
        }

        if self.left {
            self.cars.rotate_left(1);
            self.cars[STRIPE_LENGTH - 1] = new_tile;
        } else {
            self.cars.rotate_right(1);
            self.cars[0] = new_tile;
        }
    }

    fn collides(&self, x: u8) -> bool {
        self.cars[x as usize]
    }

    fn visualize(&self) -> StripeRender {
        let car = Block::Red;
        let road = Block::Gray;
        let blocks: [Block; STRIPE_LENGTH] = core::array::from_fn(|i| {
            if self.cars[i] { car } else { road }
        });
        StripeRender::new(blocks, Some(Offset {
            offset: self.offset,
            left: self.left,
            fill: Block::Gray,
        }))
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
    fn color_coded(&self) -> Vec<ColoredChar> {
        self.render_len(TILE_WIDTH)
    }

    fn render_len(&self, len: usize) -> Vec<ColoredChar> {
        let mut block = Vec::new();
        for _ in 0..len {
            block.push(self.to_char())
        }
        block
    }

    fn to_char(&self) -> ColoredChar {
        match self {
            Block::Green => ColoredChar::Green,
            Block::BrightGreen => ColoredChar::BrightGreen,
            Block::White => ColoredChar::White,
            Block::Gray => ColoredChar::Gray,
            Block::DarkYellow => ColoredChar::DarkYellow,
            Block::Red => ColoredChar::Red,
            Block::Black => ColoredChar::Black,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum ColoredChar {
    Green,
    BrightGreen,
    White,
    Gray,
    DarkYellow,
    Red,
    Black,
}

impl ColoredChar {
    fn get_color(&self) -> String {
        match self {
            ColoredChar::Green => '█'.dark_green().to_string(),
            ColoredChar::BrightGreen => '█'.green().to_string(),
            ColoredChar::White => '█'.white().to_string(),
            ColoredChar::Gray => '█'.dark_grey().to_string(),
            ColoredChar::DarkYellow => '█'.dark_yellow().to_string(),
            ColoredChar::Red => '█'.red().to_string(),
            ColoredChar::Black => '█'.black().to_string(),
        }
    }
}

pub struct StripeRender {
    blocks: [Block; STRIPE_LENGTH],

    offset: Option<Offset>,

    overlay: [Option<Block>; STRIPE_LENGTH],
}

impl StripeRender {
    fn default() -> Self {
        Self::new([Block::Black; STRIPE_LENGTH], None)
    }

    fn new(blocks: [Block; STRIPE_LENGTH], offset: Option<Offset>) -> Self {
        StripeRender {
            blocks,
            offset,
            overlay: [None; STRIPE_LENGTH],
        }
    }

    /// Renders everything but the overlay to row of colored characters.
    fn render_base(&self) -> Vec<ColoredChar> {
        let mut res = Vec::new();
        let mut blocks = self.blocks.iter().enumerate();

        // First block. Requires special handling to apply offset for the rest of the stripe
        if let Some(offset) = &self.offset {
            if offset.left {
                let (_, first) = blocks.next().unwrap();
                res.append(&mut first.render_len((TILE_WIDTH - offset.offset).max(0)));
            } else {
                res.append(&mut offset.fill.render_len(offset.offset))
            }
        }

        // Middle
        while let Some((idx, block)) = blocks.next() {
            res.append(&mut block.color_coded());
            if idx == 5 && self.offset.as_ref().is_some_and(|o| !o.left) {
                break;
            }
        }

        // End. Requires special handling to cancel offset effects
        if let Some(offset) = &self.offset {
            if offset.left {
                res.append(&mut offset.fill.render_len(offset.offset));
            } else {
                let (_, first) = blocks.next().unwrap();
                res.append(&mut first.render_len((3 - offset.offset).max(0)));
            }
        }
        res
    }

    pub fn render(&self) -> String {
        let mut stripe = self.render_base();

        // apply overlay
        for (idx, block) in self.overlay.iter().enumerate() {
            if let Some(block) = block {
                for i in 0..TILE_WIDTH {
                    stripe[idx * TILE_WIDTH + i] = block.to_char();
                }
            }
        }

        // render
        stripe.par_iter().map(|e| e.get_color()).collect()
    }

    pub fn add_overlay(&mut self, idx: usize, block: Block) {
        self.overlay[idx] = Some(block);
    }
}

struct Offset {
    /// How many third of a block the render should be offset.
    offset: usize,
    fill: Block,
    /// Weather offset should be applied to the left instead of the right.
    left: bool,
}