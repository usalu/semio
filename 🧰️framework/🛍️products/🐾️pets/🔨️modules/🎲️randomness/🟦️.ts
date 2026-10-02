/** 🎲️ Counter-based randomness of the pets simulation: a draw is a pure function of a key of unsigned 32-bit integers (`[seed, stream, counter]`), so the order in which actors are processed never changes what they draw.
 *
 * The words are those of `numpy.random.SeedSequence(key).generate_state(count)`: a pool of four words is filled by
 * hashing the key (words the key lacks count as 0), every pool word is mixed into every other one, key words beyond
 * the pool are mixed into all four, and the output hashes the pool cyclically. All arithmetic wraps at 32 bits
 * (`Math.imul`, `>>> 0` ≙ `u32::wrapping_mul`, `wrapping_sub`).
 *
 * @see https://github.com/numpy/numpy/blob/main/numpy/random/bit_generator.pyx — `SeedSequence`: `hashmix`, `mix`, `mix_entropy`, `generate_state`
 * @see https://www.pcg-random.org/posts/developing-a-seed_seq-alternative.html — the design of the hash-mix
 * @see ./🦀️.rs — the Rust twin
 */

//#region 🔖️Constants
const POOL_SIZE = 4;
const INIT_A = 0x43b0d7e5;
const MULT_A = 0x931e8875;
const INIT_B = 0x8b51f9dd;
const MULT_B = 0x58f38ded;
const MIX_MULT_L = 0xca01f9dd;
const MIX_MULT_R = 0x4973f715;
const TWO_POW_32 = 4294967296;

/** 🎪️ The stream of the stage itself (`[seed, STAGE_STREAM, stage.draws]`): arrivals, pairings, encounters. Actors draw from the streams 0, 1, 2 … of their species, far below the reserved ones. */
export const STAGE_STREAM = 0xffffffff;

/** 🎟️ The stream of the casting (`[seed, CAST_STREAM, 0]`): word 0 is where the core of a cast begins its turns, word 1 where its rotation does. */
export const CAST_STREAM = 0xfffffffe;

/** 🔁️ The stream of a render target's rotation clock (`[seed, ROTATION_STREAM, epoch]`): how long an epoch of the cast lasts. */
export const ROTATION_STREAM = 0xfffffffd;
//#endregion 🔖️Constants

//#region 🔖️Words
/** 🥣️ `hashmix` with its multiplier made explicit: `value` is xored with the multiplier `before` this step, multiplied by the one `after` it and folded onto its upper half. */
function stir(value: number, before: number, after: number): number {
  const product = Math.imul(value ^ before, after);
  return (product ^ (product >>> 16)) >>> 0;
}

/** 🧪️ `mix`: `MIX_MULT_L × into − MIX_MULT_R × from`, folded onto its upper half. */
function blend(into: number, from: number): number {
  const difference = (Math.imul(MIX_MULT_L, into) - Math.imul(MIX_MULT_R, from)) >>> 0;
  return (difference ^ (difference >>> 16)) >>> 0;
}

/** 🫙️ The entropy pool of a key (`mix_entropy`): four words that depend on every bit of every key word. */
function poolOf(key: readonly number[]): number[] {
  const pool = [0, 0, 0, 0];
  let multiplier = INIT_A;
  for (let index = 0; index < POOL_SIZE; index++) {
    const next = Math.imul(multiplier, MULT_A) >>> 0;
    pool[index] = stir(index < key.length ? key[index]! : 0, multiplier, next);
    multiplier = next;
  }
  for (let source = 0; source < POOL_SIZE; source++) {
    for (let target = 0; target < POOL_SIZE; target++) {
      if (source === target) continue;
      const next = Math.imul(multiplier, MULT_A) >>> 0;
      pool[target] = blend(pool[target]!, stir(pool[source]!, multiplier, next));
      multiplier = next;
    }
  }
  for (let source = POOL_SIZE; source < key.length; source++) {
    for (let target = 0; target < POOL_SIZE; target++) {
      const next = Math.imul(multiplier, MULT_A) >>> 0;
      pool[target] = blend(pool[target]!, stir(key[source]!, multiplier, next));
      multiplier = next;
    }
  }
  return pool;
}

/** 🔢️ The first `count` unsigned 32-bit words of `numpy.random.SeedSequence(key).generate_state(count)` for a key of unsigned 32-bit integers. */
export function randomWords(key: readonly number[], count: number): number[] {
  const pool = poolOf(key);
  const words: number[] = [];
  let multiplier = INIT_B;
  for (let index = 0; index < count; index++) {
    const next = Math.imul(multiplier, MULT_B) >>> 0;
    words.push(stir(pool[index % POOL_SIZE]!, multiplier, next));
    multiplier = next;
  }
  return words;
}
//#endregion 🔖️Words

//#region 🔖️Draws
/** 🪙️ A word as a number in [0, 1): the word divided by 2³² (exact, a multiple of 2⁻³²). */
export function unitOf(word: number): number {
  return word / TWO_POW_32;
}

/** 🎯️ A number in [0, 1): {@link unitOf} the first word of the key. */
export function randomUnit(key: readonly number[]): number {
  return unitOf(randomWords(key, 1)[0]!);
}

/** 📏️ A number between `low` and `high`: `low + (high − low) × randomUnit(key)`. */
export function randomBetween(key: readonly number[], low: number, high: number): number {
  return low + (high - low) * randomUnit(key);
}

/** ⚖️ The index a unit draw picks in proportion to `weights`: the first index whose running sum of positive weights exceeds `unit × total`; weights that are not positive are never picked, and −1 says no weight is positive. */
export function weightedIndex(weights: readonly number[], unit: number): number {
  let total = 0;
  for (let index = 0; index < weights.length; index++) {
    const weight = weights[index]!;
    if (weight > 0) total = total + weight;
  }
  if (!(total > 0)) return -1;
  const mark = unit * total;
  let running = 0;
  let last = -1;
  for (let index = 0; index < weights.length; index++) {
    const weight = weights[index]!;
    if (!(weight > 0)) continue;
    running = running + weight;
    last = index;
    if (mark < running) return index;
  }
  return last;
}

/** 🎰️ An index drawn in proportion to `weights`: {@link weightedIndex} at `randomUnit(key)`. */
export function randomPick(key: readonly number[], weights: readonly number[]): number {
  return weightedIndex(weights, randomUnit(key));
}
//#endregion 🔖️Draws
