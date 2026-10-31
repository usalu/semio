/** 🎲️ Deterministic counter-based noise: a 32-bit mix of integer keys gives uniforms and Gaussian pairs, so a random-walk increment is a pure function of (seed, parcel key, step index) and identical for play and scrub.
 *
 * @see https://github.com/aappleby/smhasher — MurmurHash3 finaliser
 */

/** 🎲️ A 32-bit hash of four integers (murmur3 mixing). */
export function hash32(a: number, b: number, c: number, d: number): number {
  let h = 0x9e3779b9 ^ a;
  for (const k of [b, c, d]) {
    let x = Math.imul(k | 0, 0xcc9e2d51);
    x = (x << 15) | (x >>> 17);
    x = Math.imul(x, 0x1b873593);
    h ^= x;
    h = (h << 13) | (h >>> 19);
    h = (Math.imul(h, 5) + 0xe6546b64) | 0;
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return h >>> 0;
}

/** 🎲️ A uniform in (0, 1) from a hash. */
export const uniform = (hash: number): number => (hash + 0.5) / 4294967296;

/** 🎲️ Two independent standard normals (Box–Muller) keyed by `(seed, key, step)`. */
export function gaussianPair(seed: number, key: number, step: number): readonly [number, number] {
  const u1 = uniform(hash32(seed, key, step, 1));
  const u2 = uniform(hash32(seed, key, step, 2));
  const r = Math.sqrt(-2 * Math.log(u1));
  return [r * Math.cos(2 * Math.PI * u2), r * Math.sin(2 * Math.PI * u2)];
}
