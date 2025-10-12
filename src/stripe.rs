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

    pub fn visualize(&self) -> Vec<Block> {
        match self {
            Stripe::Empty => vec![], // Uninitialized
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

#[derive(Debug, Copy, Clone)]
struct Road {
    cars: [bool; 7],
    direction: bool,
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
            direction: rand::random(),
        };
        for x in 0..7 {
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

        if self.direction {
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

    fn visualize(&self) -> Vec<Block> {
        let car = Block::Red;
        let road = Block::Gray;
        self.cars
            .map(|t| if t { car } else { road })
            .to_vec()
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