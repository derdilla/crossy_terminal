use std::collections::VecDeque;
use rayon::prelude::*;
use crate::stripe::{Block, Stripe};

const ROW_COUNT: usize = 20;

const MAX_PLAYER_Y_INDEX: usize = 3;

pub struct MapState {
    /// Queue of [ROW_COUNT] rows.
    state: VecDeque<Stripe>,
    /// The players position as an x coordinate in [0...6].
    ///
    /// 3 is the center. The players y coordinate is always at `state[MAX_PLAYER_Y_INDEX]` or below.
    player_x: u8,
    /// The amount of steps downward (y-direction) the player currently maintains.
    player_down: u8,
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
            player_down: 0,
            score: 0,
            alive: true,
        }
    }

    pub fn up(&mut self) {
        if self.player_down > 0 {
            self.player_down -= 1;
        } else {
            self.score += 1;
            self.state.push_back(Stripe::generate());
            self.state.pop_front();
        }
        self.detect_death();
    }

    pub fn down(&mut self) {
        self.player_down += 1;
        self.detect_death();
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

        if (MAX_PLAYER_Y_INDEX as i32 - self.player_down as i32) < 0
            || self.state.get(MAX_PLAYER_Y_INDEX - self.player_down as usize)
                    .unwrap().collides(self.player_x) {
            self.alive = false;
        }
    }

    pub fn render(&self) -> String {
        if !self.alive {
            return format!("You died! Score: {}", self.score);
        }

        self.state.par_iter()
            .enumerate()
            .map(|(idx, stripe)| {
                let mut stripe = stripe.visualize();
                if idx == MAX_PLAYER_Y_INDEX - self.player_down as usize {
                    stripe.add_overlay(self.player_x as usize, Block::White);
                }
                stripe.render()
            })
            .rev()
            .collect::<Vec<String>>()
            .join("\n\r")
    }
}
