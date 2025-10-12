use crossterm::style::Stylize;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;

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
    trees: [bool; 7],
}

impl GreenStripe {
    pub fn generate() -> Self {
        let mut trees = [false; 7];
        rand::fill(&mut trees);
        trees[3] = false;
        GreenStripe { trees }
    }

    fn update(&mut self) {}

    fn collides(&self, x: u8) -> bool {
        self.trees[x as usize]
    }

    fn visualize(&self) -> StripeRender {
        let tree = Block::Green;
        let grass = Block::BrightGreen;
        let blocks: [Block; 7] = core::array::from_fn(|i| {
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
            0..3 => [Block::Red; 7],
            3..12 => [Block::DarkYellow; 7],
            _ => [Block::Gray; 7],
        };
        StripeRender::new(blocks, None)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Road {
    cars: [bool; 7],
    left: bool,
    current_car_len: i32,
    /// Cycles in 0..=2.
    offset: usize,
}

impl Road {
    fn generate() -> Self {
        let mut road = Road {
            cars: [false; 7],
            current_car_len: 0,
            offset: 0,
            left: rand::random(),
        };
        for _ in 0..7 {
            road.advance_road();
        }

        road
    }

    fn update(&mut self) {
        self.offset += 1;
        self.offset %= 3;
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
            self.cars[6] = new_tile;
        } else {
            self.cars.rotate_right(1);
            self.cars[6] = new_tile;
        }
    }

    fn collides(&self, x: u8) -> bool {
        self.cars[x as usize]
    }

    fn visualize(&self) -> StripeRender {
        let car = Block::Red;
        let road = Block::Gray;
        let blocks: [Block; 7] = core::array::from_fn(|i| {
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
    fn color_coded(&self) -> String {
        self.render_len(3)
    }

    fn render_len(&self, len: usize) -> String {
        let mut block = String::new();
        for _ in 0..len {block.push('█') }
        match self {
            Block::Green => block.dark_green().to_string(),
            Block::BrightGreen => block.green().to_string(),
            Block::White => block.white().to_string(),
            Block::Gray => block.dark_grey().to_string(),
            Block::DarkYellow => block.dark_yellow().to_string(),
            Block::Red => block.red().to_string(),
            Block::Black => block.black().to_string(),
        }
    }
}

pub struct StripeRender {
    blocks: [Block; 7],

    offset: Option<Offset>,

    overlay: [Option<Block>; 7],
}

impl StripeRender {
    fn default() -> Self {
        Self::new([Block::Black; 7], None)
    }

    fn new(blocks: [Block; 7], offset: Option<Offset>) -> Self {
        StripeRender {
            blocks,
            offset,
            overlay: [None; 7],
        }
    }

    pub fn render(&self) -> String {
        // TODO:
        // - Fix displaying overlay at start/end position
        // - Don't render Offset for overlay
        let mut res = String::new();
        let mut blocks = self.blocks.iter().enumerate();

        if let Some(offset) = &self.offset {
            if offset.left {
                let (_, first) = blocks.next().unwrap();
                res.push_str(&first.render_len((3 - offset.offset).max(0)));
            } else {
                res.push_str(&offset.fill.render_len(offset.offset))
            }
        }

        while let Some((idx, block)) = blocks.next() {
            let mut block = *block;
            if let Some(overlay) = self.overlay[idx] {
                block = overlay;
            }
            res.push_str(&block.color_coded());
            if idx == 5 && self.offset.as_ref().is_some_and(|o| !o.left) {
                break;
            }
        }

        if let Some(offset) = &self.offset {
            if offset.left {
                res.push_str(&offset.fill.render_len(offset.offset))
            } else {
                let (_, first) = blocks.next().unwrap();
                res.push_str(&first.render_len((3 - offset.offset).max(0)));
            }
        }

        res
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