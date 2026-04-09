//! Pseudo-random number generation seeded from username and passphrase.

include!(concat!(env!("OUT_DIR"), "/salt.rs"));

/// Internal state for random number generation.
pub struct RandomState {
    seed: u128,
    cursor: u128,
}

/// Pseudo-random number generator.
pub struct Random {
    state: RandomState,
}

impl RandomState {
    /// Creates a new RandomState with the given salt and seed.
    pub fn new(salt: u128, seed: u128) -> Self {
        Self {
            seed,
            cursor: salt.wrapping_mul(COMPILE_SALT) ^ seed,
        }
    }

    /// Generates the next random value in the sequence.
    pub fn next(&mut self) -> u128 {
        let x = self.cursor;
        self.cursor = self
            .cursor
            .wrapping_mul(6364136223846793005_u128)
            .wrapping_add(1);
        x
    }
}

impl Random {
    /// Creates a new Random with the given salt and seed.
    pub fn new(salt: u128, seed: u128) -> Self {
        Self {
            state: RandomState::new(salt, seed),
        }
    }

    /// Creates a Random from username and passphrase inputs.
    pub fn from_inputs(username: &str, passphrase: &str) -> Self {
        let seed = username.bytes().fold(0x811C9DC5_u128, |h, b| {
            (h ^ b as u128).wrapping_mul(0x01000193)
        }) ^ passphrase.bytes().fold(0x811C9DC5_u128, |h, b| {
            (h ^ b as u128).wrapping_mul(0x01000193)
        });
        Self::new(COMPILE_SALT, seed)
    }

    /// Returns an iterator that produces random values.
    pub fn iter(&self) -> RandomIter {
        RandomIter {
            state: RandomState::new(COMPILE_SALT, self.state.seed),
        }
    }

    /// Returns the seed used by this generator.
    pub fn get_seed(&self) -> u128 {
        self.state.seed
    }
}

/// Iterator that yields random u128 values.
pub struct RandomIter {
    state: RandomState,
}

impl Iterator for RandomIter {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.state.next())
    }
}

impl ExactSizeIterator for RandomIter {
    fn len(&self) -> usize {
        usize::MAX
    }
}

impl<'a> IntoIterator for &'a Random {
    type Item = u128;
    type IntoIter = RandomIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
