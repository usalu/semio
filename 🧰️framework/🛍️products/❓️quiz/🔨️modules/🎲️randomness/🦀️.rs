//! 🎲️ Bit-exact randomness shared by every quiz core: the FNV-1a run seed, the MT19937 generator,
//! the rejection-sampled uniform index and the Fisher–Yates shuffle (design §3).
//!
//! @see <http://www.isthe.com/chongo/tech/comp/fnv/> — FNV-1a
//! @see <http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html> — `init_genrand`/`genrand_int32`
//! @see ../🎲️randomness/🟦️.ts — the TypeScript twin

const FNV_OFFSET_BASIS: u32 = 2_166_136_261;
const FNV_PRIME: u32 = 16_777_619;
const STATE_SIZE: usize = 624;
const SHIFT_SIZE: usize = 397;
const MATRIX_A: u32 = 0x9908_b0df;
const UPPER_MASK: u32 = 0x8000_0000;
const LOWER_MASK: u32 = 0x7fff_ffff;

/// #️⃣ FNV-1a 32-bit over the UTF-8 bytes of `text`.
pub fn fnv1a32(text: &str) -> u32 {
    text.bytes().fold(FNV_OFFSET_BASIS, |hash, byte| (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME))
}

/// 🌱️ The sheet seed of a run: FNV-1a of its id.
pub fn run_seed(run: &str) -> u32 {
    fnv1a32(run)
}

/// 🌀️ The 32-bit Mersenne Twister MT19937, seeded with `init_genrand`, yielding tempered `u32`s.
#[derive(Clone, Debug)]
pub struct Mt19937 {
    state: [u32; STATE_SIZE],
    index: usize,
}

impl Mt19937 {
    /// 🥚️ `init_genrand(seed)`.
    pub fn new(seed: u32) -> Self {
        let mut state = [0u32; STATE_SIZE];
        state[0] = seed;
        for i in 1..STATE_SIZE {
            let previous = state[i - 1];
            state[i] = 1_812_433_253u32.wrapping_mul(previous ^ (previous >> 30)).wrapping_add(i as u32);
        }
        Self { state, index: STATE_SIZE }
    }

    /// ➡️ `genrand_int32()`: the next tempered output.
    pub fn next_u32(&mut self) -> u32 {
        if self.index >= STATE_SIZE {
            self.twist();
        }
        let mut y = self.state[self.index];
        self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    fn twist(&mut self) {
        for k in 0..STATE_SIZE {
            let y = (self.state[k] & UPPER_MASK) | (self.state[(k + 1) % STATE_SIZE] & LOWER_MASK);
            let magnitude = if y & 1 == 0 { 0 } else { MATRIX_A };
            self.state[k] = self.state[(k + SHIFT_SIZE) % STATE_SIZE] ^ (y >> 1) ^ magnitude;
        }
        self.index = 0;
    }
}

/// 🎯️ An unbiased index in `0..n` by rejection sampling: `n ≤ 1 → 0`, else draw until the output
/// falls below `2³² − (2³² mod n)` and reduce it modulo `n`.
pub fn uniform_index(random: &mut Mt19937, n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    let n = n as u64;
    let limit = (1u64 << 32) - (1u64 << 32) % n;
    loop {
        let x = u64::from(random.next_u32());
        if x < limit {
            return (x % n) as usize;
        }
    }
}

/// 🔀️ Fisher–Yates from the end on a copy: for `i = len − 1 … 1` swap `i` with `uniform_index(i + 1)`.
pub fn shuffle<T: Clone>(random: &mut Mt19937, items: &[T]) -> Vec<T> {
    let mut copy = items.to_vec();
    for i in (1..copy.len()).rev() {
        let j = uniform_index(random, i + 1);
        copy.swap(i, j);
    }
    copy
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
