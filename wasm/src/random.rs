// java.util.Random, so a seed gives the same game here as in the Java version

#[derive(Clone)]
pub struct Random {
    seed: u64,
}

const MULTIPLIER: u64 = 0x5DEECE66D;
const MASK: u64 = (1 << 48) - 1;

impl Random {
    pub fn new(seed: u64) -> Random {
        Random { seed: (seed ^ MULTIPLIER) & MASK }
    }

    fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(MULTIPLIER).wrapping_add(0xB)) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    pub fn next_double(&mut self) -> f64 {
        let high = (self.next(26) as i64) << 27;
        (high + self.next(27) as i64) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_java() {
        // new java.util.Random(42).nextDouble() x3
        let mut random = Random::new(42);
        assert_eq!(random.next_double(), 0.7275636800328681);
        assert_eq!(random.next_double(), 0.6832234717598454);
        assert_eq!(random.next_double(), 0.30871945533265976);
    }
}
