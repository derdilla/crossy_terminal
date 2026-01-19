use std::collections::VecDeque;
use std::ops::Div;
use rayon::prelude::*;
use crate::stripe::{Block, GreenStripe, Stripe, WallOfDeathPhase, STRIPE_LENGTH};

const ROW_COUNT: usize = 20;

const MAX_PLAYER_Y_INDEX: usize = 3;

pub struct MapState {
    /// Queue of [ROW_COUNT] rows.
    state: VecDeque<Stripe>,
    /// The players position as an x coordinate in [0...6].
    ///
    /// 3 is the center. The players y coordinate is always at `state[MAX_PLAYER_Y_INDEX]` or below.
    player_x: u8,
    /// The y (score) value of the lowest visible row.
    bottom_y: u64,
    /// The amount of steps downward (y-direction) the player currently maintains.
    player_down: u8,
    score: u64,
    /// A wall of death moves upwards to discourage standing still.
    wall_of_death: u64,
    wall_of_death_phase: WallOfDeathPhase,
    tick: u64,
    /// False until the first key is pressed
    game_started: bool,
    pub alive: bool,
}

impl MapState {
    pub fn new() -> MapState {
        let mut state = [Stripe::Empty; ROW_COUNT];
        state.fill_with(Stripe::generate);
        for i in 0..MAX_PLAYER_Y_INDEX + 1 {
            state[i] = Stripe::Green(GreenStripe::generate());
        }
        MapState {
            state: VecDeque::from(state),
            player_x: STRIPE_LENGTH.div(2) as u8,
            bottom_y: 0,
            player_down: 0,
            score: 0,
            alive: true,
            wall_of_death: 0,
            wall_of_death_phase: WallOfDeathPhase::Normal,
            tick: 0,
            game_started: false,
        }
    }

    pub fn up(&mut self) {
        if !self.game_started { self.game_started = true; }

        if self.player_down > 0 {
            self.player_down -= 1;
        } else {
            self.score += 1;
            self.state.push_back(Stripe::generate());
            self.state.pop_front();
            self.bottom_y += 1;
            if self.wall_of_death < self.bottom_y {
                self.wall_of_death_phase = WallOfDeathPhase::Normal;
                self.wall_of_death = self.bottom_y;
            }
        }
        self.detect_death();
    }

    pub fn down(&mut self) {
        if !self.game_started { self.game_started = true; }

        self.player_down += 1;
        self.detect_death();
    }

    pub fn left(&mut self) {
        if !self.game_started { self.game_started = true; }

        if self.player_x > 0 {
            self.player_x -= 1;
        }
        self.detect_death();
    }

    pub fn right(&mut self) {
        if !self.game_started { self.game_started = true; }

        if self.player_x < (STRIPE_LENGTH - 1) as u8 {
            self.player_x += 1;
        }
        self.detect_death();
    }

    pub fn update(&mut self) {
        self.tick += 1;

        for stripe in &mut self.state {
            stripe.update();
        }

        if self.game_started && self.tick % 5 == 0 {
            match self.wall_of_death_phase {
                WallOfDeathPhase::Normal => { self.wall_of_death_phase = WallOfDeathPhase::Muddy }
                WallOfDeathPhase::Muddy => { self.wall_of_death_phase = WallOfDeathPhase::Shaky }
                WallOfDeathPhase::Shaky => { self.wall_of_death_phase = WallOfDeathPhase::Risky }
                WallOfDeathPhase::Risky => { self.wall_of_death_phase = WallOfDeathPhase::Gone }
                WallOfDeathPhase::Gone => {
                    self.wall_of_death_phase = WallOfDeathPhase::Normal;
                    self.wall_of_death += 1;
                }
            }
        }

        self.detect_death();
    }

    fn detect_death(&mut self) {
        let player_pos = self.y_pos((MAX_PLAYER_Y_INDEX as i32 - self.player_down as i32) as usize);
        if player_pos < self.wall_of_death
            || (MAX_PLAYER_Y_INDEX as i32 - self.player_down as i32) < 0
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
                let phase = if self.y_pos(idx) == self.wall_of_death {
                    self.wall_of_death_phase.clone()
                } else if self.y_pos(idx) < self.wall_of_death {
                    WallOfDeathPhase::Gone
                } else {
                    WallOfDeathPhase::Normal
                };
                stripe.render(phase)
            })
            .rev()
            .collect::<Vec<String>>()
            .join("\n\r")
    }

    fn y_pos(&self, idx: usize) -> u64 {
        self.bottom_y + idx as u64
    }
}
