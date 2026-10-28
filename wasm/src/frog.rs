// Frog.java: the frog's movement, jumping, drawing, and collision detection

use crate::assets;
use crate::gfx::Graphics;
use crate::objects::{Alligator, Log, Rect, Turtle};
use crate::util::{self, Art};

const JUMP_DIST: i32 = 50;
const RIGHT: usize = 0;
const UP: usize = 1;
const LEFT: usize = 2;
const DOWN: usize = 3;
const STACK_SIZE: usize = 2; // Maximum number of jumps in the stack

const DEATH_TIME: f64 = 30.0; // Number of game frames the frog's death animation lasts
const DEATH_FRAMES: i32 = 7; // Number of frames in the frog's death animation

#[derive(Clone)]
pub struct Frog {
    x: i32,
    y: i32,
    jump_x: i32, // Where the frog is jumping to
    jump_y: i32,
    facing: usize, // Direction the frog is facing
    movement_keys: [usize; 4],
    jump_stack: Vec<usize>,
    jump_progress: f64, // Progress of the jump
    in_air: bool,       // Whether the frog is in the air
    momentum: i32,      // Momentum of the frog
    death_timer: f64,   // 1.0 -> 0.0
}

impl Frog {
    pub fn new(move_right: usize, move_up: usize, move_left: usize, move_down: usize) -> Frog {
        let mut frog = Frog {
            x: 0,
            y: 0,
            jump_x: 0,
            jump_y: 0,
            facing: UP,
            movement_keys: [move_right, move_up, move_left, move_down],
            jump_stack: Vec::new(),
            jump_progress: 0.0,
            in_air: false,
            momentum: 0,
            death_timer: 0.0,
        };
        frog.reset();
        frog
    }

    pub fn reset(&mut self) {
        self.x = 400;
        self.y = 700;
        self.jump_x = self.x;
        self.jump_y = self.y;
        self.facing = UP;
        self.in_air = false;
        self.death_timer = 0.0;
        self.clear_jump_stack();
    }

    pub fn die(&mut self) {
        // Clear the jump stack to prevent the frog from jumping while dead and after death
        self.clear_jump_stack();

        // If the frog is already dead, do nothing - this is to prevent the frog from dying multiple times
        if self.death_timer > 0.0 {
            return;
        }

        // Start the death animation
        self.death_timer = 1.0;
    }

    pub fn move_with_keys(&mut self, keys: &[bool]) {
        // Check if the movement keys are pressed and queue the jump
        for i in 0..4 {
            if keys[self.movement_keys[i]] {
                self.queue_jump(i);
            }
        }

        // Move the frog
        self.move_frog();
    }

    fn queue_jump(&mut self, direction: usize) {
        // If the frog is dead, do nothing
        if self.death_timer > 0.0 {
            return;
        }

        // If the jump stack is full, do nothing
        if self.jump_stack.len() >= STACK_SIZE {
            return;
        }

        // Add the direction to the jump stack
        self.jump_stack.push(direction);
    }

    fn clear_jump_stack(&mut self) {
        self.jump_stack.clear();
    }

    pub fn reset_momentum(&mut self) {
        self.momentum = 0;
    }

    fn move_frog(&mut self) {
        // If the frog is dead, update the death timer
        if self.death_timer > 0.0 {
            self.death_timer -= 1.0 / DEATH_TIME;
            if self.death_timer < 0.0 {
                self.death_timer = 0.0;
                self.reset();
            }
            return;
        }

        // If the frog is not in the air and the jump stack is empty, the frog is not moving
        if self.jump_stack.is_empty() && self.x == self.jump_x && self.y == self.jump_y {
            self.in_air = false;

            // Move the frog by the momentum
            self.x += self.momentum;
            self.jump_x += self.momentum;
            return;
        }

        // If the frog is at the destination and the jump stack is not empty, the frog is starting a jump
        if self.x == self.jump_x && self.y == self.jump_y && !self.jump_stack.is_empty() {
            self.facing = self.jump_stack.remove(0);
            self.jump_progress = 0.0;

            // If the frog is at the edge of the screen and it is trying to move off the screen, don't jump
            if (self.facing == LEFT && self.x <= 0)
                || (self.facing == RIGHT && self.x >= 800 - JUMP_DIST)
                || (self.facing == UP && self.y <= 0)
                || (self.facing == DOWN && self.y >= 750 - JUMP_DIST)
            {
                // Move the frog by the momentum
                self.x += self.momentum;
                self.jump_x += self.momentum;
                return;
            }

            self.in_air = true;
            match self.facing {
                RIGHT => self.jump_x += JUMP_DIST,
                UP => self.jump_y -= JUMP_DIST,
                LEFT => self.jump_x -= JUMP_DIST,
                _ => self.jump_y += JUMP_DIST,
            }
        }

        // Ease the frog into the jump
        let remaining = 1.0 - self.jump_progress;
        let progress = 1.0 - remaining * remaining;
        self.x = (self.x as f64 + (self.jump_x - self.x) as f64 * progress + self.momentum as f64) as i32;
        self.y = (self.y as f64 + (self.jump_y - self.y) as f64 * progress) as i32;

        self.jump_x += self.momentum;

        self.jump_progress += 0.2;

        if self.jump_progress >= 1.0 || (self.x == self.jump_x && self.y == self.jump_y) {
            self.x = self.jump_x;
            self.y = self.jump_y;
            self.in_air = false;
        }
    }

    pub fn is_colliding(&self, r: Rect) -> bool {
        // Calculate the distance between the frog and the rectangle - circle collision detection (more forgiving)
        let frog_center_x = self.x + 25;
        let frog_center_y = self.y + 25;
        let frog_radius = 25;

        let rect_center_x = r.x + r.width / 2;
        let rect_center_y = r.y + r.height / 2;

        // Calculate the distance between the frog and the rectangle
        let delta_x = frog_center_x - rect_center_x;
        let delta_y = frog_center_y - rect_center_y;
        let distance = ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt();

        // If the distance is less than the sum of the frog's radius and half the rectangle's width or
        // height, the frog is colliding
        distance < (frog_radius + r.width.min(r.height) / 2) as f64
    }

    pub fn is_being_eaten(&self, a: &Alligator) -> bool {
        // Check if the frog is colliding with the alligator's mouth and the alligator's mouth is open
        self.is_colliding(a.get_mouth_rect()) && a.mouth_is_open()
    }

    pub fn is_riding_turtle(&self, t: &Turtle) -> bool {
        // Check if the frog is colliding with the turtle and the turtle is not underwater
        t.base.get_rect().contains(self.x + 25, self.y + 25) && !self.in_air && !t.is_underwater()
    }

    pub fn is_riding_log(&self, l: &Log) -> bool {
        // Check if the frog is colliding with the log and the frog is not in the air
        l.base.get_rect().contains(self.x + 25, self.y + 25) && !self.in_air
    }

    pub fn is_riding_alligator(&self, a: &Alligator) -> bool {
        // Check if the frog is colliding with the alligator and the frog is not in the air
        a.base.get_rect().contains(self.x + 25, self.y + 25) && !self.in_air
    }

    pub fn slide(&mut self, velocity: i32) {
        // Set the momentum to the log's, turtle's or alligator's velocity
        self.momentum = velocity;
    }

    #[doc(hidden)]
    pub fn is_idle(&self) -> bool {
        self.jump_stack.is_empty() && self.x == self.jump_x && self.y == self.jump_y
    }

    #[doc(hidden)]
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn in_air(&self) -> bool {
        self.in_air
    }

    pub fn off_screen(&self) -> bool {
        // Check if the frog is off the screen - 25 pixels off the edge
        self.x <= -25 || self.x >= 775 || self.y <= -25 || self.y >= 775
    }

    pub fn is_dead(&self) -> bool {
        self.death_timer > 0.0
    }

    fn get_death_frame(&self) -> i32 {
        // Get the current frame of the death animation
        6 - (self.death_timer * DEATH_FRAMES as f64) as i32
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the frog - death animation or not
        if self.death_timer > 0.0 {
            util::draw_image(g, art, assets::DEATH_0 + self.get_death_frame() as usize, self.x, self.y, 50, 50);
        } else {
            let asset = assets::FROG_00 + 2 * self.facing + if self.in_air { 1 } else { 0 };
            util::draw_image(g, art, asset, self.x, self.y, 50, 50);
        }
    }
}
