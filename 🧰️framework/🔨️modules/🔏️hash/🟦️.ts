/** 🔏️ First-party BLAKE3 (default 32-byte hash mode, no key/context) — the TypeScript twin of this
 * module's `🦀️.rs` `hash_bytes`. Runtime module, not a script: it runs unchanged in a browser
 * Worker, in Bun and in Node because it touches nothing but `Uint8Array`/`Uint32Array`. Web Crypto
 * provides SHA-256 but has no BLAKE3, so verified execution-target components are hashed here, on
 * every document open (ticket 26/09/23 F3: the allocating reference port hashed 8–35 MB/s, 0.6 s of a
 * 2.8 s warm writer open and 1.5 s of a stdio open). The compression function keeps its 16 state words
 * in locals and reads the message through a precomputed round schedule; blocks are compressed straight
 * from the input, and nothing is allocated per block. Tree construction per the BLAKE3 reference
 * (https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs). */

//#region 🔏️Blake3
const IV0 = 0x6a09e667;
const IV1 = 0xbb67ae85;
const IV2 = 0x3c6ef372;
const IV3 = 0xa54ff53a;
const BLAKE3_IV = new Uint32Array([IV0, IV1, IV2, IV3, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
const BLAKE3_CHUNK_START = 1;
const BLAKE3_CHUNK_END = 2;
const BLAKE3_PARENT = 4;
const BLAKE3_ROOT = 8;
const BLAKE3_CHUNK_LEN = 1024;
const BLAKE3_BLOCK_LEN = 64;
const BLAKE3_OUT_LEN = 32;
const BLAKE3_MAX_DEPTH = 54;

/** 🔀️ The message word each of the 7 rounds reads at each position: round 0 is the identity, every later
 * round the previous one permuted by BLAKE3's fixed message permutation. */
const BLAKE3_SCHEDULE = (() => {
  const permutation = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
  const schedule = new Uint8Array(7 * 16);
  let order = Array.from({ length: 16 }, (_, index) => index);
  for (let round = 0; round < 7; round++) {
    schedule.set(order, round * 16);
    order = permutation.map((index) => order[index]!);
  }
  return schedule;
})();

const LITTLE_ENDIAN = new Uint8Array(new Uint32Array([1]).buffer)[0] === 1;

/** 🌀️ One compression of the 16 words `m[mo…mo+15]` into `cv[co…co+7]`: writes the 8-word chaining value to
 * `out[oo…]`, or the full 16-word output block when `full` (the root output's extension words). */
function blake3Compress(cv: Uint32Array, co: number, m: Uint32Array, mo: number, counter: number, blockLen: number, flags: number, out: Uint32Array, oo: number, full: boolean): void {
  const c0 = cv[co]!, c1 = cv[co + 1]!, c2 = cv[co + 2]!, c3 = cv[co + 3]!, c4 = cv[co + 4]!, c5 = cv[co + 5]!, c6 = cv[co + 6]!, c7 = cv[co + 7]!;
  let s0 = c0, s1 = c1, s2 = c2, s3 = c3, s4 = c4, s5 = c5, s6 = c6, s7 = c7;
  let s8 = IV0, s9 = IV1, s10 = IV2, s11 = IV3;
  let s12 = counter | 0, s13 = Math.floor(counter / 4294967296) | 0, s14 = blockLen, s15 = flags;
  const S = BLAKE3_SCHEDULE;
  for (let r = 0; r < 112; r += 16) {
    s0 = (s0 + s4 + m[mo + S[r]!]!) | 0; s12 ^= s0; s12 = (s12 >>> 16) | (s12 << 16);
    s8 = (s8 + s12) | 0; s4 ^= s8; s4 = (s4 >>> 12) | (s4 << 20);
    s0 = (s0 + s4 + m[mo + S[r + 1]!]!) | 0; s12 ^= s0; s12 = (s12 >>> 8) | (s12 << 24);
    s8 = (s8 + s12) | 0; s4 ^= s8; s4 = (s4 >>> 7) | (s4 << 25);
    s1 = (s1 + s5 + m[mo + S[r + 2]!]!) | 0; s13 ^= s1; s13 = (s13 >>> 16) | (s13 << 16);
    s9 = (s9 + s13) | 0; s5 ^= s9; s5 = (s5 >>> 12) | (s5 << 20);
    s1 = (s1 + s5 + m[mo + S[r + 3]!]!) | 0; s13 ^= s1; s13 = (s13 >>> 8) | (s13 << 24);
    s9 = (s9 + s13) | 0; s5 ^= s9; s5 = (s5 >>> 7) | (s5 << 25);
    s2 = (s2 + s6 + m[mo + S[r + 4]!]!) | 0; s14 ^= s2; s14 = (s14 >>> 16) | (s14 << 16);
    s10 = (s10 + s14) | 0; s6 ^= s10; s6 = (s6 >>> 12) | (s6 << 20);
    s2 = (s2 + s6 + m[mo + S[r + 5]!]!) | 0; s14 ^= s2; s14 = (s14 >>> 8) | (s14 << 24);
    s10 = (s10 + s14) | 0; s6 ^= s10; s6 = (s6 >>> 7) | (s6 << 25);
    s3 = (s3 + s7 + m[mo + S[r + 6]!]!) | 0; s15 ^= s3; s15 = (s15 >>> 16) | (s15 << 16);
    s11 = (s11 + s15) | 0; s7 ^= s11; s7 = (s7 >>> 12) | (s7 << 20);
    s3 = (s3 + s7 + m[mo + S[r + 7]!]!) | 0; s15 ^= s3; s15 = (s15 >>> 8) | (s15 << 24);
    s11 = (s11 + s15) | 0; s7 ^= s11; s7 = (s7 >>> 7) | (s7 << 25);
    s0 = (s0 + s5 + m[mo + S[r + 8]!]!) | 0; s15 ^= s0; s15 = (s15 >>> 16) | (s15 << 16);
    s10 = (s10 + s15) | 0; s5 ^= s10; s5 = (s5 >>> 12) | (s5 << 20);
    s0 = (s0 + s5 + m[mo + S[r + 9]!]!) | 0; s15 ^= s0; s15 = (s15 >>> 8) | (s15 << 24);
    s10 = (s10 + s15) | 0; s5 ^= s10; s5 = (s5 >>> 7) | (s5 << 25);
    s1 = (s1 + s6 + m[mo + S[r + 10]!]!) | 0; s12 ^= s1; s12 = (s12 >>> 16) | (s12 << 16);
    s11 = (s11 + s12) | 0; s6 ^= s11; s6 = (s6 >>> 12) | (s6 << 20);
    s1 = (s1 + s6 + m[mo + S[r + 11]!]!) | 0; s12 ^= s1; s12 = (s12 >>> 8) | (s12 << 24);
    s11 = (s11 + s12) | 0; s6 ^= s11; s6 = (s6 >>> 7) | (s6 << 25);
    s2 = (s2 + s7 + m[mo + S[r + 12]!]!) | 0; s13 ^= s2; s13 = (s13 >>> 16) | (s13 << 16);
    s8 = (s8 + s13) | 0; s7 ^= s8; s7 = (s7 >>> 12) | (s7 << 20);
    s2 = (s2 + s7 + m[mo + S[r + 13]!]!) | 0; s13 ^= s2; s13 = (s13 >>> 8) | (s13 << 24);
    s8 = (s8 + s13) | 0; s7 ^= s8; s7 = (s7 >>> 7) | (s7 << 25);
    s3 = (s3 + s4 + m[mo + S[r + 14]!]!) | 0; s14 ^= s3; s14 = (s14 >>> 16) | (s14 << 16);
    s9 = (s9 + s14) | 0; s4 ^= s9; s4 = (s4 >>> 12) | (s4 << 20);
    s3 = (s3 + s4 + m[mo + S[r + 15]!]!) | 0; s14 ^= s3; s14 = (s14 >>> 8) | (s14 << 24);
    s9 = (s9 + s14) | 0; s4 ^= s9; s4 = (s4 >>> 7) | (s4 << 25);
  }
  out[oo] = s0 ^ s8; out[oo + 1] = s1 ^ s9; out[oo + 2] = s2 ^ s10; out[oo + 3] = s3 ^ s11;
  out[oo + 4] = s4 ^ s12; out[oo + 5] = s5 ^ s13; out[oo + 6] = s6 ^ s14; out[oo + 7] = s7 ^ s15;
  if (!full) return;
  out[oo + 8] = s8 ^ c0; out[oo + 9] = s9 ^ c1; out[oo + 10] = s10 ^ c2; out[oo + 11] = s11 ^ c3;
  out[oo + 12] = s12 ^ c4; out[oo + 13] = s13 ^ c5; out[oo + 14] = s14 ^ c6; out[oo + 15] = s15 ^ c7;
}

/** 🧮️ Streaming hasher: chunks (1024 B) of 16 chained blocks (64 B), chunk chaining values merged pairwise into
 * a binary Merkle tree through a "trailing-zero-bits" stack, root-finalized on `digest` (which leaves the state
 * as it was, so hashing may continue). */
export class Blake3Hasher {
  private readonly cv = BLAKE3_IV.slice();
  private chunkCounter = 0;
  private blocksCompressed = 0;
  private readonly block = new Uint8Array(BLAKE3_BLOCK_LEN);
  private readonly blockWords = new Uint32Array(this.block.buffer);
  private blockLen = 0;
  private readonly stack = new Uint32Array(BLAKE3_MAX_DEPTH * 8);
  private stackLen = 0;
  private readonly aligned = new Uint32Array(BLAKE3_CHUNK_LEN / 4);
  private readonly parent = new Uint32Array(16);

  private startFlag(): number {
    return this.blocksCompressed === 0 ? BLAKE3_CHUNK_START : 0;
  }

  private loadBlockWords(): Uint32Array {
    if (!LITTLE_ENDIAN) {
      const bytes = this.block;
      for (let i = 0; i < 16; i++) this.blockWords[i] = bytes[i * 4]! | (bytes[i * 4 + 1]! << 8) | (bytes[i * 4 + 2]! << 16) | (bytes[i * 4 + 3]! << 24);
    }
    return this.blockWords;
  }

  private pushChunk(): void {
    let totalChunks = this.chunkCounter + 1;
    const stack = this.stack;
    const parent = this.parent;
    const cv = this.cv;
    while ((totalChunks & 1) === 0) {
      this.stackLen--;
      parent.set(stack.subarray(this.stackLen * 8, this.stackLen * 8 + 8), 0);
      parent.set(cv, 8);
      blake3Compress(BLAKE3_IV, 0, parent, 0, 0, BLAKE3_BLOCK_LEN, BLAKE3_PARENT, cv, 0, false);
      totalChunks = Math.floor(totalChunks / 2);
    }
    stack.set(cv, this.stackLen * 8);
    this.stackLen++;
    cv.set(BLAKE3_IV);
    this.chunkCounter++;
    this.blocksCompressed = 0;
  }

  update(input: Uint8Array): void {
    let offset = 0;
    const length = input.length;
    const aligned = LITTLE_ENDIAN && (input.byteOffset & 3) === 0 ? new Uint32Array(input.buffer, input.byteOffset, length >>> 2) : null;
    while (offset < length) {
      if (this.blockLen === BLAKE3_BLOCK_LEN) {
        const flags = this.startFlag() | (this.blocksCompressed === 15 ? BLAKE3_CHUNK_END : 0);
        blake3Compress(this.cv, 0, this.loadBlockWords(), 0, this.chunkCounter, BLAKE3_BLOCK_LEN, flags, this.cv, 0, false);
        this.blockLen = 0;
        if (++this.blocksCompressed === 16) this.pushChunk();
      }
      if (this.blockLen === 0 && this.blocksCompressed < 15) {
        const direct = Math.min(15 - this.blocksCompressed, Math.floor((length - offset - 1) / BLAKE3_BLOCK_LEN));
        if (direct > 0) {
          let words: Uint32Array;
          let wordOffset: number;
          if (aligned !== null && (offset & 3) === 0) {
            words = aligned;
            wordOffset = offset >>> 2;
          } else {
            const bytes = direct * BLAKE3_BLOCK_LEN;
            const view = new Uint8Array(this.aligned.buffer, 0, bytes);
            view.set(input.subarray(offset, offset + bytes));
            if (!LITTLE_ENDIAN) for (let i = 0; i < bytes >>> 2; i++) this.aligned[i] = view[i * 4]! | (view[i * 4 + 1]! << 8) | (view[i * 4 + 2]! << 16) | (view[i * 4 + 3]! << 24);
            words = this.aligned;
            wordOffset = 0;
          }
          for (let block = 0; block < direct; block++) {
            blake3Compress(this.cv, 0, words, wordOffset + block * 16, this.chunkCounter, BLAKE3_BLOCK_LEN, this.startFlag(), this.cv, 0, false);
            this.blocksCompressed++;
          }
          offset += direct * BLAKE3_BLOCK_LEN;
          continue;
        }
      }
      const take = Math.min(BLAKE3_BLOCK_LEN - this.blockLen, length - offset);
      this.block.set(input.subarray(offset, offset + take), this.blockLen);
      this.blockLen += take;
      offset += take;
    }
  }

  digest(outLen = BLAKE3_OUT_LEN): Uint8Array {
    this.block.fill(0, this.blockLen);
    const words = new Uint32Array(this.loadBlockWords());
    let inputCv = this.cv.slice();
    let counter = this.chunkCounter;
    let blockLen = this.blockLen;
    let flags = this.startFlag() | BLAKE3_CHUNK_END;
    const chaining = new Uint32Array(8);
    for (let index = this.stackLen - 1; index >= 0; index--) {
      blake3Compress(inputCv, 0, words, 0, counter, blockLen, flags, chaining, 0, false);
      words.set(this.stack.subarray(index * 8, index * 8 + 8), 0);
      words.set(chaining, 8);
      inputCv = BLAKE3_IV.slice();
      counter = 0;
      blockLen = BLAKE3_BLOCK_LEN;
      flags = BLAKE3_PARENT;
    }
    const out = new Uint8Array(outLen);
    const output = new Uint32Array(16);
    for (let outputBlock = 0, written = 0; written < outLen; outputBlock++) {
      blake3Compress(inputCv, 0, words, 0, outputBlock, blockLen, flags | BLAKE3_ROOT, output, 0, true);
      for (let i = 0; i < 16 && written < outLen; i++) for (let shift = 0; shift < 32 && written < outLen; shift += 8) out[written++] = (output[i]! >>> shift) & 0xff;
    }
    return out;
  }
}

/** 🔗️ Hex-encoded BLAKE3 hash of `bytes`, matching `semio_framework_hash::hash_bytes`'s format. */
export function blake3Hex(bytes: Uint8Array): string {
  const hasher = new Blake3Hasher();
  hasher.update(bytes);
  return Array.from(hasher.digest(), (byte) => byte.toString(16).padStart(2, "0")).join("");
}
//#endregion 🔏️Blake3

//#region 🔐️Sha256
const SHA256_ROUNDS = new Uint32Array([
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);

function sha256Compress(state: Uint32Array, block: Uint8Array, offset: number, words: Uint32Array): void {
  for (let index = 0; index < 16; index += 1) words[index] = ((block[offset + index * 4]! << 24) | (block[offset + index * 4 + 1]! << 16) | (block[offset + index * 4 + 2]! << 8) | block[offset + index * 4 + 3]!) >>> 0;
  for (let index = 16; index < 64; index += 1) {
    const w15 = words[index - 15]!;
    const w2 = words[index - 2]!;
    const s0 = ((w15 >>> 7) | (w15 << 25)) ^ ((w15 >>> 18) | (w15 << 14)) ^ (w15 >>> 3);
    const s1 = ((w2 >>> 17) | (w2 << 15)) ^ ((w2 >>> 19) | (w2 << 13)) ^ (w2 >>> 10);
    words[index] = (words[index - 16]! + s0 + words[index - 7]! + s1) >>> 0;
  }
  let [a, b, c, d, e, f, g, h] = state as unknown as [number, number, number, number, number, number, number, number];
  for (let index = 0; index < 64; index += 1) {
    const sum1 = ((e >>> 6) | (e << 26)) ^ ((e >>> 11) | (e << 21)) ^ ((e >>> 25) | (e << 7));
    const choice = (e & f) ^ (~e & g);
    const temporary1 = (h + sum1 + choice + SHA256_ROUNDS[index]! + words[index]!) >>> 0;
    const sum0 = ((a >>> 2) | (a << 30)) ^ ((a >>> 13) | (a << 19)) ^ ((a >>> 22) | (a << 10));
    const majority = (a & b) ^ (a & c) ^ (b & c);
    const temporary2 = (sum0 + majority) >>> 0;
    h = g;
    g = f;
    f = e;
    e = (d + temporary1) >>> 0;
    d = c;
    c = b;
    b = a;
    a = (temporary1 + temporary2) >>> 0;
  }
  const next = [a, b, c, d, e, f, g, h];
  for (let index = 0; index < 8; index += 1) state[index] = (state[index]! + next[index]!) >>> 0;
}

/** 🔐️ First-party synchronous SHA-256 (FIPS 180-4) — the TypeScript twin of this module's `🦀️.rs` `Sha256::digest`, for
 * callers that cannot await Web Crypto (content ids minted inside a synchronous mutation twin). */
export function sha256(bytes: Uint8Array): Uint8Array {
  const state = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
  const words = new Uint32Array(64);
  const whole = bytes.length - (bytes.length % 64);
  for (let offset = 0; offset < whole; offset += 64) sha256Compress(state, bytes, offset, words);
  const tail = new Uint8Array(bytes.length % 64 < 56 ? 64 : 128);
  tail.set(bytes.subarray(whole));
  tail[bytes.length - whole] = 0x80;
  const bits = BigInt(bytes.length) * 8n;
  for (let index = 0; index < 8; index += 1) tail[tail.length - 1 - index] = Number((bits >> BigInt(index * 8)) & 0xffn);
  for (let offset = 0; offset < tail.length; offset += 64) sha256Compress(state, tail, offset, words);
  const digest = new Uint8Array(32);
  for (let index = 0; index < 8; index += 1) {
    digest[index * 4] = state[index]! >>> 24;
    digest[index * 4 + 1] = (state[index]! >>> 16) & 0xff;
    digest[index * 4 + 2] = (state[index]! >>> 8) & 0xff;
    digest[index * 4 + 3] = state[index]! & 0xff;
  }
  return digest;
}

/** 🔡 Lowercase hexadecimal of `bytes` — the TypeScript twin of this module's `🦀️.rs` `hex_lower`. */
export function hexLower(bytes: Uint8Array): string {
  let output = "";
  for (const byte of bytes) output += byte.toString(16).padStart(2, "0");
  return output;
}

/** #️⃣ Canonical lowercase SHA-256 hex of a complete byte slice — twin of `🦀️.rs` `sha256_hex`. */
export function sha256Hex(bytes: Uint8Array): string {
  return hexLower(sha256(bytes));
}
//#endregion 🔐️Sha256
