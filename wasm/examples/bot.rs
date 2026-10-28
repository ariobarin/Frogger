// Plays the game with a short look-ahead and writes what it did as a script for frames.rs and the Java
// frame renderer, so the frame by frame comparison also covers the river, the goals and the levels.
// Usage: cargo run --release --example bot -- <seed> <steps> <script out>

use std::fmt::Write;

use frogger::Panel;

const UP: usize = 38;
const LEFT: usize = 37;
const RIGHT: usize = 39;
const DOWN: usize = 40;
const ACTIONS: [Option<usize>; 5] = [Some(UP), Some(LEFT), Some(RIGHT), None, Some(DOWN)];

// (screen, level, score, dead, idle, y)
fn state(p: &Panel) -> (i32, i32, i32, bool, bool, i32) {
    p.debug_state()
}

fn act(p: &mut Panel, key: Option<usize>) {
    if let Some(k) = key {
        p.key_pressed(k);
    }
    p.step();
    if let Some(k) = key {
        p.key_released(k);
    }
}

// How good a plan of hops is: None if the frog dies, else progress (score, then height)
fn rate(p: &Panel, plan: &[Option<usize>]) -> Option<i64> {
    let mut q = p.clone();
    let start_score = state(&q).2;
    for &key in plan {
        act(&mut q, key);
        for _ in 0..5 {
            act(&mut q, None);
            if state(&q).3 {
                return None;
            }
        }
    }
    for _ in 0..14 {
        act(&mut q, None);
        if state(&q).3 {
            return None;
        }
    }
    let (_, level, score, _, _, y) = state(&q);
    Some((level as i64) * 1_000_000 + (score - start_score) as i64 * 1000 - y as i64)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: u64 = args[1].parse().unwrap();
    let steps: usize = args[2].parse().unwrap();
    let images = frogger::asset_pngs().iter().map(|_| frogger::GameImage::new(0, 0)).collect();
    let (mut panel, _art, _g) = frogger::new_game(images, seed, "");
    let mut script = format!("seed {seed}\n");

    let mut t = 0;
    while t < steps {
        let (screen, ..) = state(&panel);
        if screen != 1 {
            // intro: press Play; leaderboard: click to go back to the intro
            let (x, y) = if screen == 0 { (400, 280) } else { (5, 5) };
            panel.mouse_moved(x, y);
            panel.mouse_pressed(x, y);
            writeln!(script, "move {x} {y}\npress {x} {y}").unwrap();
        }
        if t % 60 == 0 {
            panel.reset_lives();
            script.push_str("lives\n");
        }
        let (_, _, _, dead, idle, _) = state(&panel);
        let mut key = None;
        if !dead && idle && screen == 1 {
            let mut best = (i64::MIN, None);
            for a in ACTIONS {
                for b in ACTIONS {
                    for c in ACTIONS {
                        if let Some(r) = rate(&panel, &[a, b, c]) {
                            if r > best.0 {
                                best = (r, a);
                            }
                        }
                    }
                }
            }
            key = best.1;
        }
        act(&mut panel, key);
        match key {
            Some(k) => writeln!(script, "key {k} down\nstep 1\nkey {k} up").unwrap(),
            None => script.push_str("step 1\n"),
        }
        if t % 20 == 0 {
            writeln!(script, "shot bot{seed}_{t:05}").unwrap();
        }
        t += 1;
    }
    let (_, level, score, ..) = state(&panel);
    eprintln!("seed {seed}: level {level}, score {score}");
    std::fs::write(&args[3], script).unwrap();
}
