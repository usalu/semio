/** 🔏️ First-party BLAKE3 (default 32-byte hash mode, no key/context) — the TypeScript twin of this
 * module's `🦀️.rs` `hash_bytes`. Runtime module, not a script: it runs unchanged in a browser
 * Worker, in Bun and in Node because it touches nothing but `Uint8Array`/`Uint32Array`. Web Crypto
 * provides SHA-256 but has no BLAKE3, so verified execution-target components are hashed here.
 * Ported from the BLAKE3 reference tree-hash construction
 * (https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs). */

//#region 🔏️Blake3
const BLAKE3_IV = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
const BLAKE3_MSG_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const BLAKE3_CHUNK_START = 1;
const BLAKE3_CHUNK_END = 2;
const BLAKE3_PARENT = 4;
const BLAKE3_ROOT = 8;
const BLAKE3_CHUNK_LEN = 1024;
const BLAKE3_BLOCK_LEN = 64;
const BLAKE3_OUT_LEN = 32;

function blake3Rotr(x: number, n: number): number {
  return ((x >>> n) | (x << (32 - n))) >>> 0;
}

function blake3G(state: Uint32Array, a: number, b: number, c: number, d: number, mx: number, my: number): void {
  state[a] = (state[a]! + state[b]! + mx) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 16);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 12);
  state[a] = (state[a]! + state[b]! + my) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 8);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 7);
}

function blake3RoundFn(state: Uint32Array, m: Uint32Array): void {
  blake3G(state, 0, 4, 8, 12, m[0]!, m[1]!);
  blake3G(state, 1, 5, 9, 13, m[2]!, m[3]!);
  blake3G(state, 2, 6, 10, 14, m[4]!, m[5]!);
  blake3G(state, 3, 7, 11, 15, m[6]!, m[7]!);
  blake3G(state, 0, 5, 10, 15, m[8]!, m[9]!);
  blake3G(state, 1, 6, 11, 12, m[10]!, m[11]!);
  blake3G(state, 2, 7, 8, 13, m[12]!, m[13]!);
  blake3G(state, 3, 4, 9, 14, m[14]!, m[15]!);
}

function blake3Permute(m: Uint32Array): Uint32Array {
  const out = new Uint32Array(16);
  for (let i = 0; i < 16; i++) out[i] = m[BLAKE3_MSG_PERMUTATION[i]!]!;
  return out;
}

function blake3Compress(cv: Uint32Array, block: Uint32Array, counter: number, blockLen: number, flags: number): Uint32Array {
  const state = new Uint32Array(16);
  state.set(cv, 0);
  state.set(BLAKE3_IV.subarray(0, 4), 8);
  state[12] = counter >>> 0;
  state[13] = Math.floor(counter / 2 ** 32) >>> 0;
  state[14] = blockLen;
  state[15] = flags;
  let m = block;
  for (let r = 0; r < 7; r++) {
    blake3RoundFn(state, m);
    if (r < 6) m = blake3Permute(m);
  }
  for (let i = 0; i < 8; i++) {
    state[i] = (state[i]! ^ state[i + 8]!) >>> 0;
    state[i + 8] = (state[i + 8]! ^ cv[i]!) >>> 0;
  }
  return state;
}

function blake3WordsFromBytes(bytes: Uint8Array, offset: number): Uint32Array {
  const words = new Uint32Array(16);
  for (let i = 0; i < 16; i++) {
    const o = offset + i * 4;
    words[i] = (bytes[o]! | (bytes[o + 1]! << 8) | (bytes[o + 2]! << 16) | (bytes[o + 3]! << 24)) >>> 0;
  }
  return words;
}

type Blake3ChunkOutput = { inputCv: Uint32Array; block: Uint32Array; counter: number; blockLen: number; flags: number };

function blake3OutputChainingValue(output: Blake3ChunkOutput): Uint32Array {
  return blake3Compress(output.inputCv, output.block, output.counter, output.blockLen, output.flags).subarray(0, 8);
}

function blake3RootOutputBytes(output: Blake3ChunkOutput, outLen: number): Uint8Array {
  const out = new Uint8Array(outLen);
  let outputBlockCounter = 0;
  let written = 0;
  while (written < outLen) {
    const words = blake3Compress(output.inputCv, output.block, outputBlockCounter, output.blockLen, output.flags | BLAKE3_ROOT);
    for (let i = 0; i < 16 && written < outLen; i++) {
      const w = words[i]!;
      out[written++] = w & 0xff;
      if (written < outLen) out[written++] = (w >>> 8) & 0xff;
      if (written < outLen) out[written++] = (w >>> 16) & 0xff;
      if (written < outLen) out[written++] = (w >>> 24) & 0xff;
    }
    outputBlockCounter++;
  }
  return out;
}

class Blake3ChunkState {
  cv: Uint32Array;
  chunkCounter: number;
  block = new Uint8Array(BLAKE3_BLOCK_LEN);
  blockLen = 0;
  blocksCompressed = 0;
  flags: number;

  constructor(key: Uint32Array, chunkCounter: number, flags: number) {
    this.cv = key.slice();
    this.chunkCounter = chunkCounter;
    this.flags = flags;
  }

  len(): number {
    return this.blocksCompressed * BLAKE3_BLOCK_LEN + this.blockLen;
  }

  startFlag(): number {
    return this.blocksCompressed === 0 ? BLAKE3_CHUNK_START : 0;
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.blockLen === BLAKE3_BLOCK_LEN) {
        const words = blake3WordsFromBytes(this.block, 0);
        this.cv = blake3Compress(this.cv, words, this.chunkCounter, BLAKE3_BLOCK_LEN, this.flags | this.startFlag()).subarray(0, 8);
        this.blocksCompressed++;
        this.block = new Uint8Array(BLAKE3_BLOCK_LEN);
        this.blockLen = 0;
      }
      const take = Math.min(BLAKE3_BLOCK_LEN - this.blockLen, input.length - offset);
      this.block.set(input.subarray(offset, offset + take), this.blockLen);
      this.blockLen += take;
      offset += take;
    }
  }

  output(): Blake3ChunkOutput {
    const words = blake3WordsFromBytes(this.block, 0);
    return { inputCv: this.cv, block: words, counter: this.chunkCounter, blockLen: this.blockLen, flags: this.flags | this.startFlag() | BLAKE3_CHUNK_END };
  }
}

function blake3ParentOutput(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Blake3ChunkOutput {
  const block = new Uint32Array(16);
  block.set(leftCv, 0);
  block.set(rightCv, 8);
  return { inputCv: key, block, counter: 0, blockLen: BLAKE3_BLOCK_LEN, flags: flags | BLAKE3_PARENT };
}

function blake3ParentCv(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Uint32Array {
  return blake3OutputChainingValue(blake3ParentOutput(leftCv, rightCv, key, flags));
}

/** 🧮️ Streaming hasher: chunk (1024B) → 16 blocks (64B) chained, chunks merged pairwise into a binary
 * Merkle tree via a "trailing-zero-bits" stack (a Merkle-mountain-range), root-finalized on `digest`. */
export class Blake3Hasher {
  private key = BLAKE3_IV;
  private chunkState = new Blake3ChunkState(BLAKE3_IV, 0, 0);
  private cvStack: Uint32Array[] = [];
  private flags = 0;

  private addChunkChainingValue(newCvIn: Uint32Array, totalChunksIn: number): void {
    let newCv = newCvIn;
    let totalChunks = totalChunksIn;
    while ((totalChunks & 1) === 0) {
      const left = this.cvStack.pop()!;
      newCv = blake3ParentCv(left, newCv, this.key, this.flags);
      totalChunks >>>= 1;
    }
    this.cvStack.push(newCv);
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.chunkState.len() === BLAKE3_CHUNK_LEN) {
        const chunkCv = blake3OutputChainingValue(this.chunkState.output());
        const totalChunks = this.chunkState.chunkCounter + 1;
        this.addChunkChainingValue(chunkCv, totalChunks);
        this.chunkState = new Blake3ChunkState(this.key, totalChunks, this.flags);
      }
      const take = Math.min(BLAKE3_CHUNK_LEN - this.chunkState.len(), input.length - offset);
      this.chunkState.update(input.subarray(offset, offset + take));
      offset += take;
    }
  }

  digest(outLen = BLAKE3_OUT_LEN): Uint8Array {
    let output = this.chunkState.output();
    let parentNodesRemaining = this.cvStack.length;
    while (parentNodesRemaining > 0) {
      parentNodesRemaining--;
      output = blake3ParentOutput(this.cvStack[parentNodesRemaining]!, blake3OutputChainingValue(output), this.key, this.flags);
    }
    return blake3RootOutputBytes(output, outLen);
  }
}

/** 🔗️ Hex-encoded BLAKE3 hash of `bytes`, matching `semio_framework_hash::hash_bytes`'s format. */
export function blake3Hex(bytes: Uint8Array): string {
  const hasher = new Blake3Hasher();
  hasher.update(bytes);
  return Array.from(hasher.digest(), (byte) => byte.toString(16).padStart(2, "0")).join("");
}
//#endregion 🔏️Blake3
