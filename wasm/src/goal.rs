// Goal.java: the goal's drawing and logic

use crate::assets;
use crate::gfx::Graphics;
use crate::objects::Rect;
use crate::random::Random;
use crate::util::{self, Art};

const DANGER_SPEED: i32 = 120; // time in frames to spawn then disappear
const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;

#[derive(Clone)]
pub struct Goal {
    x: i32,
    y: i32,
    filled: bool,
    has_fly: bool,
    danger: bool,
    danger_timer: i32,
}

impl Goal {
    pub fn new(x: i32, y: i32) -> Goal {
        Goal { x, y, filled: false, has_fly: false, danger: false, danger_timer: 0 }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, WIDTH, HEIGHT)
    }

    pub fn update(&mut self, random: &mut Random) {
        // If the goal is filled, don't update
        if self.is_filled() {
            return;
        }

        // 1% chance to spawn a fly
        if util::random_double(random, 0.0, 1.0) < 0.001 {
            self.has_fly = !self.has_fly;
        }

        // 1% chance to spawn a danger
        if util::random_double(random, 0.0, 1.0) < 0.001 && !self.danger {
            self.danger = true;
            self.danger_timer = 0;
        }

        // Update danger timer
        if self.danger {
            self.danger_timer += 1;
            if self.danger_timer >= DANGER_SPEED {
                self.danger = false;
            }
        }
    }

    // If the goal has a fly, remove it
    pub fn fly(&mut self) -> bool {
        if !self.has_fly {
            return false;
        }
        self.has_fly = false;
        true
    }

    fn get_danger_frame(&self) -> i32 {
        1 - (self.danger_timer - DANGER_SPEED / 2).abs() / (DANGER_SPEED / 4)
    }

    pub fn fill(&mut self) -> bool {
        // If the goal is already filled, don't fill it
        if self.filled {
            return false;
        }
        self.filled = true;
        true
    }

    pub fn is_filled(&self) -> bool {
        self.filled
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the danger image if the goal is dangerous
        if self.danger {
            util::draw_image(g, art, assets::GOAL_DANGER_0 + self.get_danger_frame() as usize, self.x, self.y, WIDTH, HEIGHT);
        } else if self.has_fly {
            // Draw the fly image if the goal has a fly
            util::draw_image(g, art, assets::GOAL_FLY, self.x, self.y, WIDTH, HEIGHT);
        } else if self.filled {
            util::draw_image(g, art, assets::GOAL_FILLED, self.x, self.y, WIDTH, HEIGHT);
        }

        // Draw the goal image
        util::draw_image(g, art, assets::GOAL, self.x - 50, self.y - 50, WIDTH + 100, HEIGHT + 50);
    }
}
