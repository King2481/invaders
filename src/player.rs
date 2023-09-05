use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS}, supermeter::SuperMeter,
};
use std::time::Duration;
use rusty_time::timer::Timer;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    super_active: bool,
    super_timer: Timer,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
            super_active: false,
            super_timer: Timer::from_millis(1000),
        }
    }
    pub fn move_in_direction(&mut self, direction: i32) {
        match direction {
            -1 => {
                if self.x > 0 {
                    self.x -= 1;
                }
            },
            1 => {
                if self.x < NUM_COLS - 1 {
                    self.x += 1;
                }
            },
            _ => {},
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < self.get_shot_limit() {
            self.shots.push(Shot::new(self.x, self.y - 1, true));
            true
        } else {
            false
        }
    }
    fn get_shot_limit(&self) -> usize {
        if self.super_active {
            15
        } else {
            4
        }
    }
    pub fn unleash_super(&mut self) {
        self.super_active = true;
        self.super_timer.reset();

        for num in 1..=12 {
            self.shots.push(Shot::new(num * 3, NUM_ROWS - 1, false));
        }
    }
    pub fn update(&mut self, delta: Duration) {
        if self.super_active {
            self.super_timer.update(delta);
        }
        if self.super_timer.ready {
            self.super_active = false;
        }

        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders, super_meter: &mut SuperMeter) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
                    shot.explode();
                    if shot.builds_meter {
                        super_meter.increment_meter();
                    }
                }
            }
        }
        hit_something
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = 'A';
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
