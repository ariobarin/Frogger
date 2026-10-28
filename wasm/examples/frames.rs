// Renders frames from a script without a browser, to compare with the Java version frame by frame.
// Usage: cargo run --example frames -- <script> <out dir> [scores file]
//
// Script lines: seed N | step N | key CODE down|up | move X Y | press X Y | lives | shot NAME

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

use frogger::GameImage;

fn decode(bytes: &[u8]) -> GameImage {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).unwrap();
    let (width, height) = (info.width as i32, info.height as i32);
    let channels = info.color_type.samples();
    let pixels = buffer[..info.buffer_size()]
        .chunks(channels)
        .map(|p| match channels {
            4 => u32::from_le_bytes([p[0], p[1], p[2], p[3]]),
            3 => u32::from_le_bytes([p[0], p[1], p[2], 255]),
            2 => u32::from_le_bytes([p[0], p[0], p[0], p[1]]),
            _ => u32::from_le_bytes([p[0], p[0], p[0], 255]),
        })
        .collect();
    GameImage { width, height, pixels }
}

fn save(pixels: &[u32], path: &Path) {
    let mut encoder = png::Encoder::new(BufWriter::new(File::create(path).unwrap()), 800, 800);
    encoder.set_color(png::ColorType::Rgb);
    let bytes: Vec<u8> = pixels.iter().flat_map(|p| p.to_le_bytes()[..3].to_vec()).collect();
    encoder.write_header().unwrap().write_image_data(&bytes).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let script = fs::read_to_string(&args[1]).unwrap();
    let out = Path::new(&args[2]);
    fs::create_dir_all(out).unwrap();
    let scores = args.get(3).map(|p| fs::read_to_string(p).unwrap()).unwrap_or_default();
    let seed = script.lines().find_map(|l| l.strip_prefix("seed ")).map_or(0, |s| s.trim().parse().unwrap());

    let images = frogger::asset_pngs().iter().map(|png| decode(png)).collect();
    let (mut panel, art, mut g) = frogger::new_game(images, seed, &scores);
    panel.paint(&mut g, &art);
    let mut steps = 0;

    for line in script.lines() {
        let w: Vec<&str> = line.split_whitespace().collect();
        let num = |i: usize| w[i].parse::<i32>().unwrap();
        match w.first().copied() {
            Some("step") => {
                for _ in 0..num(1) {
                    steps += 1;
                    if !panel.step() {
                        println!("game over dialog at step {steps}");
                        return;
                    }
                    panel.paint(&mut g, &art);
                }
            }
            Some("key") if w[2] == "down" => panel.key_pressed(num(1) as usize),
            Some("key") => panel.key_released(num(1) as usize),
            Some("move") => {
                panel.mouse_moved(num(1), num(2));
                panel.paint(&mut g, &art);
            }
            Some("press") => panel.mouse_pressed(num(1), num(2)),
            Some("lives") => panel.reset_lives(),
            Some("shot") => save(&g.pixels, &out.join(format!("{}.png", w[1]))),
            _ => {}
        }
    }
}
