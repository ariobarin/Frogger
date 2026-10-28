// MovingObject.java and its subclasses: Car, Log, Turtle and Alligator

use crate::assets;
use crate::gfx::Graphics;
use crate::random::Random;
use crate::util::{self, Art};

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect { x, y, width, height }
    }

    // java.awt.Rectangle.contains(Point)
    pub fn contains(&self, px: i32, py: i32) -> bool {
        if (self.width | self.height) < 0 {
            return false;
        }
        if px < self.x || py < self.y {
            return false;
        }
        let w = self.width + self.x;
        let h = self.height + self.y;
        (w < self.x || w > px) && (h < self.y || h > py)
    }
}

pub const HEIGHT: i32 = 50;

// MovingObject: moves left or right and wraps around the screen
#[derive(Clone)]
pub struct MovingObject {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub width: i32,
    pub left: bool, // Is the object moving left?
}

impl MovingObject {
    pub fn new(x: i32, y: i32, width: i32, speed: i32, left: bool) -> MovingObject {
        MovingObject { x, y, width, speed, left }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, HEIGHT)
    }

    pub fn move_object(&mut self) {
        // Move the object left or right
        if self.left {
            self.x -= self.speed;
        } else {
            self.x += self.speed;
        }

        // If the object is not visible, move it to the other side
        if self.not_visible() {
            self.x = if self.left { 800 } else { -self.width };
        }
    }

    pub fn get_velocity(&self) -> i32 {
        // Get the velocity of the object
        self.speed * if self.left { -1 } else { 1 }
    }

    pub fn not_visible(&self) -> bool {
        // Return if the object is not visible
        self.x < -self.width || self.x > 800
    }
}

// Car colors - different images for each color
pub const WHITE: usize = 0;
pub const PINK: usize = 1;
pub const YELLOW: usize = 2;
pub const RED: usize = 3;
pub const GREEN: usize = 4;

#[derive(Clone)]
pub struct Car {
    pub base: MovingObject,
    color: usize,
}

impl Car {
    pub fn new(x: i32, y: i32, width: i32, speed: i32, left: bool, color: usize) -> Car {
        Car { base: MovingObject::new(x, y, width, speed, left), color }
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the car
        let b = &self.base;
        util::draw_image(g, art, assets::CAR_0 + self.color, b.x, b.y + 4, b.width, HEIGHT - 8);
    }
}

#[derive(Clone)]
pub struct Log {
    pub base: MovingObject,
}

impl Log {
    pub fn new(x: i32, y: i32, width: i32, speed: i32, left: bool) -> Log {
        Log { base: MovingObject::new(x, y, width, speed, left) }
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        let b = &self.base;
        // Draw the left image
        util::draw_image(g, art, assets::LOG_LEFT, b.x, b.y, 50, HEIGHT);

        // Draw the middle images - extend the log
        for i in 0..(b.width - 100) / 50 {
            util::draw_image(g, art, assets::LOG_MIDDLE, b.x + 50 * i + 50, b.y, 50, HEIGHT);
        }

        // Draw the right image
        util::draw_image(g, art, assets::LOG_RIGHT, b.x + b.width - 50, b.y, 50, HEIGHT);
    }
}

const TURTLE_FRAMES: i32 = 5;
const STROKE_SPEED: f64 = 25.0; // the number of game frames per stroke cycle (not breathing)
const BREATH_SPEED: f64 = 75.0; // Breathing cycle duration in frames

#[derive(Clone)]
pub struct Turtle {
    pub base: MovingObject,
    breathing: bool,
    stroke: f64,
}

impl Turtle {
    pub fn new(x: i32, y: i32, width: i32, speed: i32, left: bool, breathing: bool, random: &mut Random) -> Turtle {
        Turtle {
            base: MovingObject::new(x, y, width, speed, left),
            breathing,
            // Start the strokes at a random point so they aren't all in sync
            stroke: util::random_double(random, 0.0, 1.0),
        }
    }

    fn get_frame(&self) -> i32 {
        if !self.breathing {
            return (self.stroke * (TURTLE_FRAMES - 2) as f64) as i32; // Desmos
        }
        (self.stroke * (TURTLE_FRAMES + 3) as f64) as i32 % TURTLE_FRAMES // Desmos
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        let asset = assets::TURTLE_0 + self.get_frame() as usize;
        let mut x = self.base.x;
        for _ in 0..self.base.width / 50 {
            util::draw_image(g, art, asset, x, self.base.y, 50, 50);
            x += 50;
        }
    }

    pub fn is_underwater(&self) -> bool {
        // Turtle is underwater if the frame is 3 or 4
        self.get_frame() >= 3
    }

    pub fn move_object(&mut self) {
        // Update the stroke based on whether the turtle is breathing or not
        if !self.breathing {
            self.stroke = (self.stroke + 1.0 / STROKE_SPEED) % 1.0;
        } else {
            self.stroke = (self.stroke + 1.0 / BREATH_SPEED) % 1.0;
        }

        // Move
        self.base.move_object();
    }
}

const MOUTH_CYCLE: f64 = 100.0; // number of frames in a mouth cycle

#[derive(Clone)]
pub struct Alligator {
    pub base: MovingObject,
    mouth: f64, // opening / closing mouth (0 - 0.5 is closed, 0.5 - 1 is open)
}

impl Alligator {
    pub fn new(x: i32, y: i32, width: i32, speed: i32, left: bool) -> Alligator {
        Alligator { base: MovingObject::new(x, y, width, speed, left), mouth: 0.0 }
    }

    pub fn mouth_is_open(&self) -> bool {
        self.mouth >= 0.5 // Mouth open for 50% of the cycle
    }

    // Get the rectangle of the alligator's mouth
    pub fn get_mouth_rect(&self) -> Rect {
        let b = &self.base;
        if b.left {
            Rect::new(b.x, b.y, 50, HEIGHT)
        } else {
            Rect::new(b.x + b.width - 50, b.y, 50, HEIGHT)
        }
    }

    pub fn move_object(&mut self) {
        self.base.move_object();
        self.mouth = (self.mouth + 1.0 / MOUTH_CYCLE) % 1.0; // Update mouth position
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the body
        let asset = if self.mouth_is_open() { assets::ALLIGATOR_1 } else { assets::ALLIGATOR_0 };
        util::draw_image(g, art, asset, self.base.x, self.base.y, self.base.width, HEIGHT);
    }
}
