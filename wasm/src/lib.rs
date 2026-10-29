// Frogger, compiled from Rust to WebAssembly. The game logic is a line-by-line port of the Java version
// in ../src; web/index.html is the page that runs it.
//
// The page talks to the game through the functions below:
//  1. for each of asset_count() images it decodes the PNG at asset_bytes(i) and copies the pixels into
//     image_pixels(i, width, height)
//  2. it writes the saved scores into text_buffer(length) and calls start(seed, length)
//  3. on every animation frame it calls frame(now) and, when that returns 1, shows screen_pixels()
//  4. keyboard and mouse events go to key(), mouse_move() and mouse_press(); the game over dialog
//     comes back through name_entered()

mod assets;
mod frog;
mod gfx;
mod goal;
mod hud;
mod objects;
mod panel;
mod random;
mod scores;
mod util;

use std::cell::RefCell;

use gfx::{Graphics, Image};
use panel::FroggerPanel;
use util::Art;

// the Swing timer ticks at FPS: one game step (and repaint) per tick
const STEP_MS: f64 = 1000.0 / panel::FPS as f64;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    // shows the "Game Over! Enter your name:" dialog; the answer comes back through name_entered
    fn ask_name();
    // keeps the scores file's text
    fn save_scores(text: *const u8, length: u32);
}

// off the web (tests) there is no page to ask
#[cfg(not(target_arch = "wasm32"))]
unsafe fn ask_name() {}
#[cfg(not(target_arch = "wasm32"))]
unsafe fn save_scores(_text: *const u8, _length: u32) {}

struct Game {
    panel: FroggerPanel,
    art: Art,
    graphics: Graphics,
    clock: f64,         // game time owed, in ms
    last: Option<f64>,  // the previous frame's time
    repaint: bool,
}

#[derive(Default)]
struct State {
    images: Vec<Image>,
    text: Vec<u8>,
    game: Option<Game>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

fn with_game(f: impl FnOnce(&mut Game)) {
    with(|state| {
        if let Some(game) = state.game.as_mut() {
            f(game)
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn asset_count() -> u32 {
    assets::PNGS.len() as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn asset_bytes(i: u32) -> *const u8 {
    assets::PNGS[i as usize].as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn asset_length(i: u32) -> u32 {
    assets::PNGS[i as usize].len() as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn image_pixels(i: u32, width: i32, height: i32) -> *mut u8 {
    with(|state| {
        let i = i as usize;
        while state.images.len() <= i {
            state.images.push(Image::new(0, 0));
        }
        state.images[i] = Image::new(width, height);
        state.images[i].pixels.as_mut_ptr() as *mut u8
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn text_buffer(length: u32) -> *mut u8 {
    with(|state| {
        state.text = vec![0; length as usize];
        state.text.as_mut_ptr()
    })
}

fn take_text(state: &mut State, length: u32) -> String {
    let text = std::mem::take(&mut state.text);
    String::from_utf8_lossy(&text[..length as usize]).into_owned()
}

#[unsafe(no_mangle)]
pub extern "C" fn start(seed_high: u32, seed_low: u32, scores_length: u32) {
    with(|state| {
        let scores = take_text(state, scores_length);
        let images = std::mem::take(&mut state.images);
        state.game = Some(Game {
            panel: FroggerPanel::new((seed_high as u64) << 32 | seed_low as u64, &scores),
            art: Art::new(images),
            graphics: Graphics::new(panel::WIDTH, panel::HEIGHT),
            clock: 0.0,
            last: None,
            repaint: true, // the window paints once when it opens
        });
    })
}

// Runs the game steps owed since the last frame. Returns 1 when the picture changed.
#[unsafe(no_mangle)]
pub extern "C" fn frame(now: f64) -> u32 {
    let mut painted = 0;
    with_game(|game| {
        let elapsed = now - game.last.unwrap_or(now);
        game.last = Some(now);
        // after a pause (a hidden tab, the dialog) carry on rather than catching up
        if game.panel.asking_name || elapsed > 250.0 {
            game.clock = 0.0;
        } else {
            game.clock += elapsed;
        }
        // round to the nearest step so a 60 Hz screen gets exactly one step per frame despite jitter
        let mut steps = ((game.clock + STEP_MS / 4.0) / STEP_MS).floor().max(0.0) as u32;
        game.clock -= steps as f64 * STEP_MS;
        if steps > 4 {
            steps = 4;
            game.clock = 0.0;
        }
        for _ in 0..steps {
            game.repaint = true;
            if !game.panel.step() {
                unsafe { ask_name() };
                break;
            }
        }
        if game.repaint {
            game.panel.paint(&mut game.graphics, &game.art);
            game.repaint = false;
            painted = 1;
        }
    });
    painted
}

#[unsafe(no_mangle)]
pub extern "C" fn screen_pixels() -> *const u8 {
    with(|state| state.game.as_ref().map_or(std::ptr::null(), |game| game.graphics.pixels.as_ptr() as *const u8))
}

// key: a java.awt.event.KeyEvent code
#[unsafe(no_mangle)]
pub extern "C" fn key(code: u32, down: u32) {
    with_game(|game| {
        if down != 0 {
            game.panel.key_pressed(code as usize);
        } else {
            game.panel.key_released(code as usize);
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    with_game(|game| {
        game.panel.mouse_moved(x, y);
        game.repaint = true;
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn mouse_press(x: i32, y: i32) {
    with_game(|game| {
        if !game.panel.asking_name {
            game.panel.mouse_pressed(x, y);
        }
    })
}

// The dialog's answer: the name is in text_buffer; ok is 0 for Cancel
#[unsafe(no_mangle)]
pub extern "C" fn name_entered(length: u32, ok: u32) {
    with(|state| {
        let name = take_text(state, length);
        let Some(game) = state.game.as_mut() else { return };
        let named = ok != 0 && !name.is_empty();
        game.panel.name_entered(if ok != 0 { Some(&name) } else { None });
        game.repaint = true;
        if named {
            let text = game.panel.scores_text();
            unsafe { save_scores(text.as_ptr(), text.len() as u32) };
        }
    })
}

// For tests: the game without the page
pub fn new_game(images: Vec<Image>, seed: u64, scores: &str) -> (FroggerPanel, Art, Graphics) {
    (FroggerPanel::new(seed, scores), Art::new(images), Graphics::new(panel::WIDTH, panel::HEIGHT))
}

pub use gfx::Image as GameImage;
pub use panel::FroggerPanel as Panel;

pub fn asset_pngs() -> &'static [&'static [u8]] {
    assets::PNGS
}

// For examples/probe_check.rs, which checks the sprite sampling against Java
#[doc(hidden)]
pub fn sample_columns(at: i32, size: i32, src: i32, screen: i32) -> Option<(i32, i32, Vec<Option<i32>>)> {
    gfx::sample_map(at, size, src, screen)
}
