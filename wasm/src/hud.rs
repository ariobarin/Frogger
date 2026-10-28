// Terrain.java, TimerBar.java, LifeCounter.java and Button.java

use crate::assets;
use crate::gfx::{Color, Graphics};
use crate::objects::Rect;
use crate::util::{self, Art};

pub const GRASS: i32 = 0;
pub const WATER: i32 = 1;

#[derive(Clone)]
pub struct Terrain {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    kind: i32,
}

impl Terrain {
    pub fn new(x: i32, y: i32, width: i32, height: i32, kind: i32) -> Terrain {
        Terrain { x, y, width, height, kind }
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the grass in 50x50 tiles; WATER and ROAD are handled in the panel
        if self.kind == GRASS {
            for i in 0..self.width / 50 {
                util::draw_image(g, art, assets::GRASS, self.x + i * 50, self.y, 50, self.height);
            }
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

const OUTLINE_WIDTH: i32 = 2;

#[derive(Clone)]
pub struct TimerBar {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    time_limit: f64,
    time_spent: f64,
    fps: i32,
}

impl TimerBar {
    pub fn new(x: i32, y: i32, width: i32, height: i32, time_limit: f64, fps: i32) -> TimerBar {
        TimerBar { x, y, width, height, time_limit, time_spent: 0.0, fps }
    }

    pub fn update(&mut self) {
        // Update the time spent
        self.time_spent += 1.0 / self.fps as f64;
        self.time_spent = self.time_spent.min(self.time_limit);
    }

    pub fn is_over_time_limit(&self) -> bool {
        self.time_spent >= self.time_limit
    }

    pub fn get_time_limit(&self) -> f64 {
        self.time_limit
    }

    pub fn get_time_spent(&self) -> f64 {
        self.time_spent
    }

    pub fn reset(&mut self) {
        self.time_spent = 0.0;
    }

    pub fn draw(&self, g: &mut Graphics) {
        // White outline
        g.draw_rect(Color::WHITE, self.x, self.y, self.width, self.height);

        // Black inner rectangle
        g.fill_rect(Color::BLACK, self.x + OUTLINE_WIDTH, self.y + OUTLINE_WIDTH, self.width - 2 * OUTLINE_WIDTH, self.height - 2 * OUTLINE_WIDTH);

        // Calculate the ratio of the time spent
        let ratio = 1f64.min((self.time_spent + 0.3 * self.time_limit) / self.time_limit);

        // Calculate the color of the bar - red for the time spent, green for the time left
        let red = (255.0 * ratio) as i32;
        let green = (255.0 * (1.0 - ratio)) as i32;
        let color = Color::rgb(red as u8, green as u8, 0);

        // Draw the bar
        let bar = ((self.time_limit - self.time_spent) / self.time_limit * (self.width - 4 * OUTLINE_WIDTH) as f64) as i32;
        g.fill_rect(color, self.x + 2 * OUTLINE_WIDTH, self.y + 2 * OUTLINE_WIDTH, bar, self.height - 4 * OUTLINE_WIDTH);
    }
}

#[derive(Clone)]
pub struct LifeCounter {
    lives: i32,
    x: i32,
    y: i32,
}

const LIFE_WIDTH: i32 = 30;
const LIFE_HEIGHT: i32 = 30;

impl LifeCounter {
    pub fn new(x: i32, y: i32) -> LifeCounter {
        LifeCounter { lives: 3, x, y }
    }

    pub fn reset(&mut self) {
        self.lives = 3;
    }

    pub fn lose_life(&mut self) {
        self.lives -= 1;
    }

    pub fn is_game_over(&self) -> bool {
        self.lives <= 0
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the life counter
        for i in 0..self.lives {
            util::draw_image(g, art, assets::LIFE, self.x + (2 - i) * LIFE_WIDTH, self.y, LIFE_WIDTH, LIFE_HEIGHT);
        }
    }
}

#[derive(Clone)]
pub struct Button {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text: &'static str, // Text on the button
    hovered: bool,
    color: Color,
    hover_color: Color,
}

impl Button {
    pub fn new(text: &'static str, x: i32, y: i32, width: i32, height: i32, color: Color) -> Button {
        // Half transparency for hover effect
        let hover_color = Color::rgba(color.r, color.g, color.b, color.a / 2);
        Button { x, y, width, height, text, hovered: false, color, hover_color }
    }

    pub fn draw(&self, g: &mut Graphics, art: &Art) {
        // Draw the button - hovered or not
        g.fill_rect(if self.hovered { self.hover_color } else { self.color }, self.x, self.y, self.width, self.height);

        // Draw the text
        util::write_centered_text(g, art, self.text, self.x + self.width / 2, self.y + self.height / 2, 20, util::WHITE_FONT);
    }

    pub fn is_hovered(&self, mx: i32, my: i32) -> bool {
        // Check if the mouse is hovering over the button
        mx >= self.x && mx <= self.x + self.width && my >= self.y && my <= self.y + self.height
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }
}
