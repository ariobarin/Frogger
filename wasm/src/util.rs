// Util.java: the images, the bitmap fonts and text drawing

use crate::assets;
use crate::gfx::{Graphics, Image};
use crate::random::Random;

pub const RED_FONT: usize = 0;
pub const WHITE_FONT: usize = 1;
pub const YELLOW_FONT: usize = 2;
pub const FROGGER_FONT: usize = 3;

// Characters in the fonts in order from left to right
const DEFAULT_FONT_CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-";
const FROGGER_FONT_CHARACTERS: &str = "FROGE";

pub struct Art {
    images: Vec<Image>,
    font_images: Vec<Vec<Option<Image>>>, // [font][character]
}

impl Art {
    // images: every asset, decoded, in the order assets.rs lists them
    pub fn new(images: Vec<Image>) -> Art {
        let mut font_images = Vec::new();
        for (font, asset) in [assets::FONT_RED, assets::FONT_WHITE, assets::FONT_YELLOW, assets::FONT_FROGGER].into_iter().enumerate() {
            let full = &images[asset];
            let characters = if font == FROGGER_FONT { FROGGER_FONT_CHARACTERS } else { DEFAULT_FONT_CHARACTERS };
            let char_width = full.width / characters.len() as i32;
            let mut glyphs: Vec<Option<Image>> = (0..128).map(|_| None).collect();
            for (i, c) in characters.chars().enumerate() {
                glyphs[c as usize] = Some(full.sub_image(i as i32 * char_width, 0, char_width, full.height));
            }
            font_images.push(glyphs);
        }
        Art { images, font_images }
    }

    pub fn image(&self, asset: usize) -> &Image {
        &self.images[asset]
    }
}

// Generate a random double between min and max
pub fn random_double(random: &mut Random, min: f64, max: f64) -> f64 {
    min + (max - min) * random.next_double()
}

pub fn draw_image(g: &mut Graphics, art: &Art, asset: usize, x: i32, y: i32, width: i32, height: i32) {
    g.draw_image(art.image(asset), x, y, width, height);
}

// Write text right-aligned to the x position
pub fn write_right_text(g: &mut Graphics, art: &Art, text: &str, x: i32, y: i32, height: i32, color: usize) {
    let text_width = length(text) * height;
    write_text_sized(g, art, text, x - text_width, y, text_width, height, color);
}

// Write text centered to the x position
pub fn write_centered_text(g: &mut Graphics, art: &Art, text: &str, center_x: i32, center_y: i32, height: i32, color: usize) {
    let text_width = length(text) * height;
    let x = center_x - text_width / 2;
    write_text_sized(g, art, text, x, center_y - height / 2, text_width, height, color);
}

// Write text to the screen
pub fn write_text(g: &mut Graphics, art: &Art, text: &str, x: i32, y: i32, height: i32, color: usize) {
    write_text_sized(g, art, text, x, y, length(text) * height, height, color);
}

pub fn write_text_sized(g: &mut Graphics, art: &Art, text: &str, x: i32, y: i32, width: i32, height: i32, color: usize) {
    if text.is_empty() {
        return;
    }
    let char_width = width / length(text);
    for (i, c) in text.chars().enumerate() {
        if c == ' ' {
            continue;
        }

        // Convert to uppercase - the font only has uppercase characters
        let c = c.to_ascii_uppercase();

        // The Java version stops drawing (with an exception) at a character the font doesn't have; this
        // skips it instead
        if let Some(Some(glyph)) = art.font_images[color].get(c as usize) {
            g.draw_image(glyph, x + i as i32 * char_width, y, char_width, height);
        }
    }
}

fn length(text: &str) -> i32 {
    text.encode_utf16().count() as i32
}
