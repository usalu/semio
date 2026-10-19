/** 🎲️ Bit-exact randomness shared by every quiz core: the FNV-1a run seed, the MT19937 generator, the rejection-sampled uniform index and the Fisher–Yates shuffle.
 *
 * @see http://www.isthe.com/chongo/tech/comp/fnv/ — FNV-1a
 * @see http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html — `init_genrand` / `genrand_int32`
 * @see ./🦀️.rs — the Rust twin
 */

const FNV_OFFSET_BASIS = 2166136261;
const FNV_PRIME = 16777619;
const STATE_SIZE = 624;
const SHIFT_SIZE = 397;
const MATRIX_A = 0x9908b0df;
const UPPER_MASK = 0x80000000;
const LOWER_MASK = 0x7fffffff;
const TWO_POW_32 = 4294967296;
const UTF8 = new TextEncoder();

/** #️⃣ FNV-1a 32-bit over the UTF-8 bytes of `text`. */
export function fnv1a32(text: string): number {
  let hash = FNV_OFFSET_BASIS;
  for (const byte of UTF8.encode(text)) hash = Math.imul(hash ^ byte, FNV_PRIME) >>> 0;
  return hash;
}

/** 🌱️ The sheet seed of a run: FNV-1a of its id. */
export function runSeed(run: string): number {
  return fnv1a32(run);
}

/** 🎰️ A source of uniformly distributed unsigned 32-bit integers. */
export interface RandomSource {
  next(): number;
}

/** 🌀️ The 32-bit Mersenne Twister MT19937, seeded with `init_genrand`, yielding tempered unsigned 32-bit outputs. */
export class Mt19937 implements RandomSource {
  private readonly state = new Uint32Array(STATE_SIZE);
  private index = STATE_SIZE;

  constructor(seed: number) {
    this.state[0] = seed >>> 0;
    for (let i = 1; i < STATE_SIZE; i++) {
      const previous = this.state[i - 1]!;
      this.state[i] = (Math.imul(1812433253, previous ^ (previous >>> 30)) + i) >>> 0;
    }
  }

  /** ➡️ `genrand_int32()`: the next tempered output. */
  next(): number {
    if (this.index >= STATE_SIZE) this.twist();
    let y = this.state[this.index++]!;
    y ^= y >>> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >>> 18;
    return y >>> 0;
  }

  private twist(): void {
    const state = this.state;
    for (let k = 0; k < STATE_SIZE; k++) {
      const y = (state[k]! & UPPER_MASK) | (state[(k + 1) % STATE_SIZE]! & LOWER_MASK);
      state[k] = state[(k + SHIFT_SIZE) % STATE_SIZE]! ^ (y >>> 1) ^ (y & 1 ? MATRIX_A : 0);
    }
    this.index = 0;
  }
}

/** 🎯️ An unbiased index in `0 … n−1` by rejection sampling: `n ≤ 1 → 0` without a draw, else draw until below `2³² − (2³² mod n)` and reduce modulo `n`. */
export function uniformIndex(random: RandomSource, n: number): number {
  if (n <= 1) return 0;
  const limit = TWO_POW_32 - (TWO_POW_32 % n);
  for (;;) {
    const x = random.next();
    if (x < limit) return x % n;
  }
}

/** 🔀️ Fisher–Yates from the end on a copy: for `i = len − 1 … 1` swap `i` with `uniformIndex(i + 1)`. */
export function shuffle<T>(random: RandomSource, items: readonly T[]): T[] {
  const copy = [...items];
  for (let i = copy.length - 1; i >= 1; i--) {
    const j = uniformIndex(random, i + 1);
    [copy[i], copy[j]] = [copy[j]!, copy[i]!];
  }
  return copy;
}
