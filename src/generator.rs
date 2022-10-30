const LCG_A: u64 = 1_664_525;
const LCG_C: u64 = 1_013_904_223;
const LCG_M: u64 = 4_294_967_296;

/// A Linear Congruential Generator (LCG)
pub struct Generator {
    seed: u64,
}

impl Generator {
    /// Creates a new instance of the generator using the provided seed.
    pub fn new_with_seed(seed: u64) -> Self {
        Generator { seed }
    }

    /// Returns the next pseudorandom number, the number if betweem 0.0 and 1.0
    pub fn next_random(&mut self) -> f64 {
        // Advance to the value in the rotation
        self.next_number();

        // Return the current pseudorandom number
        self.seed as f64 / LCG_M as f64
    }

    /// Generates the next number using LCG and sets it as the seed
    fn next_number(&mut self) {
        // Apply (a * seed + c) mod m
        self.seed = (LCG_A * self.seed + LCG_C) % LCG_M;
    }
}
