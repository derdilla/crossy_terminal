use std::collections::VecDeque;
use crossterm::style::Stylize;
use crate::stripe::{Block, Render, Stripe};

const ROW_COUNT: usize = 20;

pub struct MapState {
    /// Queue of [ROW_COUNT] rows.
    state: VecDeque<Stripe>,
    /// The players position as an x coordinate in [0...6].
    ///
    /// 3 is the center. The players y coordinate is always at `state[5]` or below.
    player_x: u8,
    score: u64,
    pub alive: bool,
}

impl MapState {
    pub fn new() -> MapState {
        let mut state = [Stripe::Empty; ROW_COUNT];
        state.fill_with(Stripe::generate);
        MapState {
            state: VecDeque::from(state),
            player_x: 3,
            score: 0,
            alive: true,
        }
    }

    pub fn up(&mut self) {
        self.score += 1;
        self.state.push_back(Stripe::generate());
        self.state.pop_front();
        self.detect_death();
    }

    pub fn down(&mut self) {
        todo!()
    }

    pub fn left(&mut self) {
        if self.player_x > 0 {
            self.player_x -= 1;
        }
        self.detect_death();
    }

    pub fn right(&mut self) {
        if self.player_x < 6 {
            self.player_x += 1;
        }
        self.detect_death();
    }

    pub fn update(&mut self) {
        for stripe in &mut self.state {
            stripe.update();
        }
        self.detect_death();
    }

    fn detect_death(&mut self) {
        if self.state.get(3).unwrap().collides(self.player_x) {
            self.alive = false;
        }
    }

    pub fn render(&self) -> String {
        if !self.alive {
            return format!("You died! Score: {}", self.score);
        }

        let mut render = String::new();
        for (row_idx, stripe) in self.state.iter().rev().enumerate() {
            let mut stripe = stripe.visualize();
            if row_idx == ROW_COUNT - 4 {
                stripe[self.player_x as usize] = Block::White;
            }
            render.push_str(&stripe.render());
            render.push_str("\n\r");
        }

        render
    }
}
