use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};
use rand::Rng;

const RARE_ENEMY_CHANCE: usize = 15;
const RARE_POINTS: u16 = 5;
const NORMAL_POINTS: u16 = 1;

pub struct Invader {
    pub x: usize,
    pub y: usize,
    is_rare: bool,
    points: u16,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    pub total_count: usize,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    let is_rare: bool = rand::thread_rng().gen_range(0..100) <= RARE_ENEMY_CHANCE;
                    let points = if is_rare { RARE_POINTS } else { NORMAL_POINTS };

                    army.push(Invader { x, y, is_rare, points});
                }
            }
        }
        let total_count = army.len();
        Self {
            army,
            total_count,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().any(|invader| invader.y >= NUM_ROWS - 1)
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> u16 {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            let points = self.army[idx].points;
            self.army.remove(idx);
            points
        } else {
            0
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                if invader.is_rare {'o'} else {'x'}
            } else {
                if invader.is_rare {'0'} else {'+'}
            }
        }
    }
}
