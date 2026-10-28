// The few Java2D drawing calls the game makes (drawImage, fillRect, drawRect), done the way Java2D's
// software loops do them, on a picture the page puts on screen

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

// Pixels are RGBA bytes, read as little-endian u32s (0xAABBGGRR), the layout canvas ImageData uses
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u32>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        Image { width, height, pixels: vec![0; (width * height) as usize] }
    }

    // BufferedImage.getSubimage, as a copy
    pub fn sub_image(&self, x: i32, y: i32, width: i32, height: i32) -> Image {
        let mut sub = Image::new(width, height);
        for row in 0..height {
            let from = ((y + row) * self.width + x) as usize;
            let to = (row * width) as usize;
            sub.pixels[to..to + width as usize].copy_from_slice(&self.pixels[from..from + width as usize]);
        }
        sub
    }
}

// Java2D's mul8table: a * b / 255, rounded
fn mul8(a: u32, b: u32) -> u32 {
    ((a * 0x10101 * b + (1 << 23)) >> 24) & 0xff
}

pub struct Graphics {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u32>,
}

impl Graphics {
    // A Swing window starts out the panel background grey
    pub fn new(width: i32, height: i32) -> Graphics {
        Graphics { width, height, pixels: vec![0xffeeeeee; (width * height) as usize] }
    }

    // Util.drawImage: black pixels are transparent, scaled with nearest neighbor interpolation. Which
    // source pixel each screen pixel takes follows Java2D exactly (see sample_map).
    pub fn draw_image(&mut self, img: &Image, x: i32, y: i32, width: i32, height: i32) {
        if width <= 0 || height <= 0 {
            return;
        }
        let Some((x1, x2, columns)) = sample_map(x, width, img.width, self.width) else { return };
        let Some((y1, y2, rows)) = sample_map(y, height, img.height, self.height) else { return };
        for dy in y1..y2 {
            let Some(sy) = rows[(dy - y1) as usize] else { continue };
            let src_row = (sy * img.width) as usize;
            let dst_row = (dy * self.width) as usize;
            for dx in x1..x2 {
                let Some(sx) = columns[(dx - x1) as usize] else { continue };
                let pixel = img.pixels[src_row + sx as usize];
                if pixel & 0x00ff_ffff == 0 {
                    continue; // black, made transparent
                }
                match pixel >> 24 {
                    0 => {}
                    255 => self.pixels[dst_row + dx as usize] = pixel,
                    alpha => {
                        let dst = &mut self.pixels[dst_row + dx as usize];
                        *dst = blend(pixel, alpha, *dst);
                    }
                }
            }
        }
    }

    // Graphics.fillRect, alpha composited over what's there (SrcOver)
    pub fn fill_rect(&mut self, color: Color, x: i32, y: i32, width: i32, height: i32) {
        if color.a == 0 {
            return;
        }
        let x1 = x.max(0);
        let x2 = (x + width).min(self.width);
        let y1 = y.max(0);
        let y2 = (y + height).min(self.height);
        let pixel = 0xff00_0000 | (color.b as u32) << 16 | (color.g as u32) << 8 | color.r as u32;
        for dy in y1..y2 {
            let row = &mut self.pixels[(dy * self.width) as usize..((dy + 1) * self.width) as usize];
            for dst in &mut row[x1.max(0) as usize..x2.max(x1) as usize] {
                *dst = if color.a == 255 { pixel } else { blend(pixel, color.a as u32, *dst) };
            }
        }
    }

    // Graphics.drawRect: a one pixel outline covering x..x+width and y..y+height
    pub fn draw_rect(&mut self, color: Color, x: i32, y: i32, width: i32, height: i32) {
        if width < 0 || height < 0 {
            return;
        }
        self.fill_rect(color, x, y, width + 1, 1);
        self.fill_rect(color, x, y + height, width + 1, 1);
        self.fill_rect(color, x, y + 1, 1, height - 1);
        self.fill_rect(color, x + width, y + 1, 1, height - 1);
    }
}

// One axis of a drawImage(img, x, y, width, height): the screen range it touches and, for each pixel of
// it, the source pixel (None where that falls outside the image). Every sprite has transparent pixels,
// so Java2D draws it through TransformHelper (TransformHelper.c, DrawImage.renderImageXform): the
// inverse transform of the first visible pixel's centre in 32.32 fixed point, then a fixed point step
// per pixel. Its rounding differs from floor((i + 0.5) * size / width) wherever that lands exactly on a
// pixel edge, and it starts counting at the first on-screen pixel. An image drawn at its own size is
// a plain copy.
fn sample_map(at: i32, size: i32, src: i32, screen: i32) -> Option<(i32, i32, Vec<Option<i32>>)> {
    if size == src {
        let (start, end) = (at.max(0), (at + size).min(screen));
        return (start < end).then(|| (start, end, (start..end).map(|d| Some(d - at)).collect()));
    }
    const ONE: f64 = 4294967296.0; // 1 in 32.32 fixed point
    let scale = size as f64 / src as f64; // AffineTransform.scale
    let inverse = 1.0 / scale; // AffineTransform.createInverse
    let offset = -(at as f64) / scale;
    let start = at.max(0);
    let end = ((src as f64 * scale + at as f64).ceil() as i32).min(screen);
    if start >= end {
        return None;
    }
    let base = ((inverse * (start as f64 + 0.5) + offset) * ONE) as i64;
    let step = (inverse * ONE) as i64;
    let map = (0..end - start)
        .map(|i| {
            let s = ((base + i as i64 * step) >> 32) as i32;
            (0..src).contains(&s).then_some(s)
        })
        .collect();
    Some((start, end, map))
}

// SrcOver onto an opaque pixel: src * a + dst * (1 - a), with Java2D's rounding
fn blend(src: u32, alpha: u32, dst: u32) -> u32 {
    let dst_f = 255 - alpha;
    let channel = |shift: u32| {
        let s = (src >> shift) & 0xff;
        let d = (dst >> shift) & 0xff;
        (mul8(alpha, s) + mul8(dst_f, d)) << shift
    };
    0xff00_0000 | channel(16) | channel(8) | channel(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul8_rounds() {
        assert_eq!(mul8(255, 200), 200);
        assert_eq!(mul8(0, 200), 0);
        assert_eq!(mul8(59, 255), 59);
        assert_eq!(mul8(128, 255), 128);
    }
}
