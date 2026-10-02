//! 🎲️ Counter-based randomness of the pets simulation: a draw is a pure function of a key of unsigned 32-bit integers (`[seed, stream, counter]`), so the order in which actors are processed never changes what they draw.
//!
//! The words are those of `numpy.random.SeedSequence(key).generate_state(count)`: a pool of four words is filled by
//! hashing the key (words the key lacks count as 0), every pool word is mixed into every other one, key words beyond
//! the pool are mixed into all four, and the output hashes the pool cyclically. All arithmetic wraps at 32 bits
//! (`u32::wrapping_mul`, `wrapping_sub` ≙ the TypeScript twin's `Math.imul`, `>>> 0`). The floating-point draws use
//! the twin's expressions in the twin's order; "no weight is positive" is `None` here and `-1` there.
//!
//! @see <https://github.com/numpy/numpy/blob/main/numpy/random/bit_generator.pyx> — `SeedSequence`: `hashmix`, `mix`, `mix_entropy`, `generate_state`
//! @see <https://www.pcg-random.org/posts/developing-a-seed_seq-alternative.html> — the design of the hash-mix
//! @see ../🎲️randomness/🟦️.ts — the TypeScript twin

//#region 🔖️Constants
const POOL_SIZE: usize = 4;
const INIT_A: u32 = 0x43b0_d7e5;
const MULT_A: u32 = 0x931e_8875;
const INIT_B: u32 = 0x8b51_f9dd;
const MULT_B: u32 = 0x58f3_8ded;
const MIX_MULT_L: u32 = 0xca01_f9dd;
const MIX_MULT_R: u32 = 0x4973_f715;
const TWO_POW_32: f64 = 4_294_967_296.0;

/// 🎪️ The stream of the stage itself (`[seed, STAGE_STREAM, stage.draws]`): arrivals, pairings, encounters. Actors draw from the streams 0, 1, 2 … of their species, far below the reserved ones.
pub const STAGE_STREAM: u32 = 0xffff_ffff;

/// 🎟️ The stream of the casting (`[seed, CAST_STREAM, 0]`): word 0 is where the core of a cast begins its turns, word 1 where its rotation does.
pub const CAST_STREAM: u32 = 0xffff_fffe;

/// 🔁️ The stream of a render target's rotation clock (`[seed, ROTATION_STREAM, epoch]`): how long an epoch of the cast lasts.
pub const ROTATION_STREAM: u32 = 0xffff_fffd;
//#endregion 🔖️Constants

//#region 🔖️Words
/// 🥣️ `hashmix` with its multiplier made explicit: `value` is xored with the multiplier `before` this step, multiplied by the one `after` it and folded onto its upper half.
fn stir(value: u32, before: u32, after: u32) -> u32 {
    let product = (value ^ before).wrapping_mul(after);
    product ^ (product >> 16)
}

/// 🧪️ `mix`: `MIX_MULT_L × into − MIX_MULT_R × from`, folded onto its upper half.
fn blend(into: u32, from: u32) -> u32 {
    let difference = MIX_MULT_L.wrapping_mul(into).wrapping_sub(MIX_MULT_R.wrapping_mul(from));
    difference ^ (difference >> 16)
}

/// 🫙️ The entropy pool of a key (`mix_entropy`): four words that depend on every bit of every key word.
fn pool_of(key: &[u32]) -> [u32; POOL_SIZE] {
    let mut pool = [0u32; POOL_SIZE];
    let mut multiplier = INIT_A;
    for (index, slot) in pool.iter_mut().enumerate() {
        let next = multiplier.wrapping_mul(MULT_A);
        *slot = stir(key.get(index).copied().unwrap_or(0), multiplier, next);
        multiplier = next;
    }
    for source in 0..POOL_SIZE {
        for target in 0..POOL_SIZE {
            if source == target {
                continue;
            }
            let next = multiplier.wrapping_mul(MULT_A);
            pool[target] = blend(pool[target], stir(pool[source], multiplier, next));
            multiplier = next;
        }
    }
    for &word in key.iter().skip(POOL_SIZE) {
        for slot in &mut pool {
            let next = multiplier.wrapping_mul(MULT_A);
            *slot = blend(*slot, stir(word, multiplier, next));
            multiplier = next;
        }
    }
    pool
}

/// 🔢️ The first `count` unsigned 32-bit words of `numpy.random.SeedSequence(key).generate_state(count)` for a key of unsigned 32-bit integers.
pub fn random_words(key: &[u32], count: usize) -> Vec<u32> {
    let pool = pool_of(key);
    let mut words = Vec::with_capacity(count);
    let mut multiplier = INIT_B;
    for index in 0..count {
        let next = multiplier.wrapping_mul(MULT_B);
        words.push(stir(pool[index % POOL_SIZE], multiplier, next));
        multiplier = next;
    }
    words
}
//#endregion 🔖️Words

//#region 🔖️Draws
/// 🪙️ A word as a number in [0, 1): the word divided by 2³² (exact, a multiple of 2⁻³²).
pub fn unit_of(word: u32) -> f64 {
    f64::from(word) / TWO_POW_32
}

/// 🎯️ A number in [0, 1): [`unit_of`] the first word of the key.
pub fn random_unit(key: &[u32]) -> f64 {
    unit_of(random_words(key, 1)[0])
}

/// 📏️ A number between `low` and `high`: `low + (high − low) × random_unit(key)`.
pub fn random_between(key: &[u32], low: f64, high: f64) -> f64 {
    low + (high - low) * random_unit(key)
}

/// ⚖️ The index a unit draw picks in proportion to `weights`: the first index whose running sum of positive weights exceeds `unit × total`; weights that are not positive are never picked, and `None` says no weight is positive (the twin's −1).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn weighted_index(weights: &[f64], unit: f64) -> Option<usize> {
    let mut total = 0.0;
    for &weight in weights {
        if weight > 0.0 {
            total += weight;
        }
    }
    if !(total > 0.0) {
        return None;
    }
    let mark = unit * total;
    let mut running = 0.0;
    let mut last = None;
    for (index, &weight) in weights.iter().enumerate() {
        if !(weight > 0.0) {
            continue;
        }
        running += weight;
        last = Some(index);
        if mark < running {
            return Some(index);
        }
    }
    last
}

/// 🎰️ An index drawn in proportion to `weights`: [`weighted_index`] at `random_unit(key)`.
pub fn random_pick(key: &[u32], weights: &[f64]) -> Option<usize> {
    weighted_index(weights, random_unit(key))
}
//#endregion 🔖️Draws

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
