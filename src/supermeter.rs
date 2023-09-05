use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;
use std::cmp::min;

const MAX_METER_AMOUNT: usize = 10;

#[derive(Default)]
pub struct SuperMeter {
    meter: usize,
    pub super_ready: bool,
    visible: bool,
    flash_timer: Timer,
}

impl SuperMeter {
    pub fn new() -> Self {
        Self { 
            meter: 0,
            super_ready: false,
            visible: true,
            flash_timer: Timer::from_millis(150),
        }
    }
    pub fn increment_meter(&mut self) {
        self.meter = min(self.meter + 1, MAX_METER_AMOUNT);
        if self.meter >= 10 {
            self.super_ready = true;
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.flash_timer.update(delta);
        if self.super_ready && self.flash_timer.ready {
            self.visible = !self.visible;
            self.flash_timer.reset();
        }
    }
    pub fn reset(&mut self) {
        self.meter = 0;
        self.super_ready = false;
        self.visible = true;
    }
}

impl Drawable for SuperMeter {
    fn draw(&self, frame: &mut Frame) {
        let bar = if self.visible { "=".repeat(self.meter) } else {" ".repeat(self.meter)};
        let padding = " ".repeat(MAX_METER_AMOUNT - self.meter);
        let ready_message= if self.super_ready {"Press F!".to_string()} else {"".to_string()};
        let formatted = format!("SUPER: [{}{}] {}", bar, padding, ready_message);

        // iterate over all characters
        for (i, c) in formatted.chars().enumerate() {
            // put them in the first row
            frame[i][1] = c;
        }
    }
}