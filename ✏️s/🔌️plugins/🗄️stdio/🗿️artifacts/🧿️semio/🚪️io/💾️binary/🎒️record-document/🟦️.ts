// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🎒️ Independent TypeScript implementation of the `stdio.semio` record-document pack: the semio binary envelope around a pack container (`SPK` header, CRC-32C framed segments, fixed-Huffman deflate, manifest, footer with the BLAKE3 content hash) whose document segment is a derived record.
 *
 * The codec is schema-first. It knows no subset: it is driven by the exported record spec of a subset
 * (`<subset>/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`, the language-neutral `PackSchemaGraph` the Rust derive
 * exports and a Rust law keeps equal to the derive). Every byte is derived from the committed wire contract
 * (`🧰️framework/🔨️modules/🎒️pack`), never from the Rust code it is measured against, so a disagreement with the
 * Rust record pack is a finding. `🐍️.py` beside this file is the same codec in Python.
 *
 * Two views of one document meet here: the **record** (`Map<field id, FieldValue>`, the exact tree the wire carries)
 * and the **document** (the camelCase JSON snapshot every mutation oracle already works on).
 * `encodePack`/`decodePack` cross both, `encodeRecord`/`decodeRecord` stay on the wire view.
 */

import { inflateRawSync } from "node:zlib";

// #region 🔖️Types
/** 🧱️ One shape of the exported record graph. */
export type Shape =
  | { kind: "bool" | "int" | "uint" | "float" | "text" | "bytes64" | "value" | "wire" | "dir" | "range" | "count" | "expr" }
  | { kind: "enum"; variants: [number, string][] }
  | { kind: "tuple"; item: Shape; len: number | null }
  | { kind: "list" | "block" | "map"; item: Shape }
  | { kind: "record" | "table"; record: number }
  | { kind: "statements"; variants: [string, number][] }
  | { kind: "quantity" | "angle"; unit: string }
  | { kind: "ref"; entity: string }
  | { kind: "coord" | "dim"; dims: number }
  | { kind: "embed"; lang: string }
  | { kind: "embedFrom"; key: string };

/** 🪪️ One field of a graph record. */
export type Field = { id: number; key: string; optional: boolean; flatten: boolean; shape: Shape };

/** 🗺️ The committed `semio.pack.record-spec/v1` document. */
export type RecordSpecDocument = { schema: string; envelope: string; schemaHash: string; graph: Field[][]; annotations?: Annotations };

/** 🪧️ The optional annotations that settle what the record graph alone cannot: which statements fields hold exactly one variant, and which float fields are single precision, which statements fields tag their variants under another key than `kind`, which statements fields write their variant payload keys verbatim, and which records keep some fields in a JSON sub-object. */
export type Annotations = { single?: readonly (readonly [number, number])[]; float32?: readonly (readonly [number, number])[]; tags?: readonly (readonly [number, number, string])[]; verbatim?: readonly (readonly [number, number])[]; nested?: readonly (readonly [number, string, readonly string[]])[] };

/** 🌱️ A dynamic value: JSON plus the explicit float (`1.0` keeps its `f64` tag) and byte string. */
export type DslValue = null | boolean | number | bigint | DslFloat | string | Uint8Array | DslValue[] | { [key: string]: DslValue };

/** 🪙️ A dynamic-value number that is a float even when it is whole. */
export class DslFloat {
  constructor(readonly value: number) {}
}

/** 🌳️ One field value, tagged by the wire kind the shape decides. */
export type FieldValue =
  | { kind: "bool"; value: boolean }
  | { kind: "int" | "uint"; value: number | bigint }
  | { kind: "float" | "enum"; value: number }
  | { kind: "text"; value: string }
  | { kind: "bytes"; value: Uint8Array }
  | { kind: "tuple" | "list"; value: FieldValue[] }
  | { kind: "record"; value: RecordFields }
  | { kind: "block"; value: FieldValue }
  | { kind: "statements"; value: [string, RecordFields][] }
  | { kind: "map"; value: [string, FieldValue][] }
  | { kind: "value"; value: DslValue };

/** 🗃️ A record: field id to value, absent fields simply missing. */
export type RecordFields = Map<number, FieldValue>;
// #endregion 🔖️Types

// #region 🔖️Constants
const BINARY_MAGIC = Uint8Array.of(0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a);
const PACK_MAGIC = Uint8Array.of(0x89, 0x53, 0x50, 0x4b, 0x0d, 0x0a, 0x1a, 0x0a);
const FOOTER_MAGIC = new TextEncoder().encode("SPKFOOT1");
const HEADER_SIZE = 32;
const FOOTER_SIZE = 84;
const REQUIRED_COMPRESSED = 1;
const REQUIRED_CHUNKED = 2;
const OPTIONAL_CANONICAL = 1;
const KIND_END = 0;
const KIND_MANIFEST = 1;
const KIND_SYMBOLS = 3;
const KIND_DOCUMENT = 4;
const KIND_CHUNK = 5;
const KIND_CHUNK_TABLE = 6;
const CODEC_DEFLATE = 1;
const CHUNK_THRESHOLD = 256 * 1024;
const CHUNK_SIZE = 1024 * 1024;
const FRAME_SIZE = 1024 * 1024;
const MAX_DEPTH = 64;
/** 📏️ The largest raw size a single segment may declare; a writer frames documents and chunks at 1 MiB, so this bounds only the symbol table. */
const MAX_SEGMENT_RAW = 1 << 26;
const I64_MAX = (1n << 63n) - 1n;
const U64_MASK = (1n << 64n) - 1n;

const TAG = { ABSENT: 0x00, FALSE: 0x01, TRUE: 0x02, INT: 0x03, UINT: 0x04, F64: 0x05, STR: 0x06, STR_INLINE: 0x07, BYTES: 0x08, BYTES_CHUNKED: 0x09, ENUM: 0x0a, TUPLE: 0x0b, LIST: 0x0c, RECORD: 0x0d, BLOCK: 0x0e, STATEMENTS: 0x0f, MAP: 0x10, VALUE: 0x11, NULL: 0x12, TABLE_SOA: 0x14, PACKED_F64: 0x15, PACKED_VARINT: 0x16 } as const;
const ELEM = { FALLBACK: 0, BOOL: 1, INT: 2, UINT: 3, F64: 4, STR: 5, ENUM: 6 } as const;
const SHAPE_TAGS: Record<string, number> = { bool: 1, int: 2, uint: 3, float: 4, text: 5, bytes64: 6, enum: 7, tuple: 8, list: 9, record: 10, block: 11, statements: 12, map: 13, value: 14, table: 15, wire: 16, quantity: 17, angle: 18, ref: 19, coord: 20, dir: 21, dim: 22, range: 23, count: 24, expr: 25, embed: 26, embedFrom: 27 };
const TUPLE_SHAPES = new Set(["tuple", "coord", "dir", "dim", "range"]);
const FLOAT_SHAPES = new Set(["float", "quantity", "angle"]);
const TEXT_SHAPES = new Set(["text", "ref"]);
const UINT_SHAPES = new Set(["uint", "count"]);
const ENCODER = new TextEncoder();
const DECODER = new TextDecoder("utf-8", { fatal: true });
// #endregion 🔖️Constants

// #region 🔖️Bytes
/** 🪢️ Concatenates byte strings. */
export function concat(...parts: Uint8Array[]): Uint8Array {
  const out = new Uint8Array(parts.reduce((sum, part) => sum + part.length, 0));
  let at = 0;
  for (const part of parts) {
    out.set(part, at);
    at += part.length;
  }
  return out;
}

/** 🔢️ Unsigned LEB128, the `varint` every length, count, id and ordinal of the pack uses. */
export function writeVarint(value: number | bigint): Uint8Array {
  let remaining = BigInt(value);
  if (remaining < 0n) throw new RangeError(`a varint cannot hold the negative value ${remaining}`);
  const out: number[] = [];
  for (;;) {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    if (remaining === 0n) {
      out.push(byte);
      return Uint8Array.from(out);
    }
    out.push(byte | 0x80);
  }
}

/** ➡️ Signed varint: the zig-zag mapped magnitude as an unsigned LEB128. */
export function writeZigzag(value: number | bigint): Uint8Array {
  const wide = BigInt(value);
  return writeVarint(((wide << 1n) ^ (wide >> 63n)) & U64_MASK);
}

/** 🔤️ The 32-bit little-endian word at a byte offset. */
function u32(bytes: Uint8Array, at: number): number {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getUint32(at, true);
}

/** 🧷️ A u32 as four little-endian bytes. */
function u32Bytes(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value >>> 0, true);
  return out;
}

/** 📎️ A u64 as eight little-endian bytes. */
function u64Bytes(value: number): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, BigInt(value), true);
  return out;
}

/** 🧵️ An f64 as eight little-endian bytes. */
function f64Bytes(value: number): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setFloat64(0, value, true);
  return out;
}

/** 📖️ Bounds-checked cursor over a byte string; every refusal names the offset it happened at. */
export class Reader {
  at: number;

  constructor(readonly data: Uint8Array, at = 0) {
    this.at = at;
  }

  take(count: number): Uint8Array {
    if (count < 0 || this.at + count > this.data.length) throw new RangeError(`the pack ends ${this.at + count - this.data.length} byte(s) short at offset ${this.at}`);
    const chunk = this.data.subarray(this.at, this.at + count);
    this.at += count;
    return chunk;
  }

  byte(): number {
    return this.take(1)[0]!;
  }

  varintBig(): bigint {
    let value = 0n;
    let shift = 0n;
    for (let count = 0; count < 10; count += 1) {
      const byte = this.byte();
      if (count === 9 && byte > 1) throw new RangeError(`varint exceeds 64 bits at offset ${this.at}`);
      value |= BigInt(byte & 0x7f) << shift;
      if ((byte & 0x80) === 0) return value;
      shift += 7n;
    }
    throw new RangeError(`overlong varint ending at offset ${this.at}`);
  }

  varint(): number {
    return Number(this.varintBig());
  }

  zigzagBig(): bigint {
    const raw = this.varintBig();
    return (raw >> 1n) ^ -(raw & 1n);
  }

  float64(): number {
    const bytes = this.take(8);
    return new DataView(bytes.buffer, bytes.byteOffset, 8).getFloat64(0, true);
  }

  remaining(): number {
    return this.data.length - this.at;
  }

  /** ⛔️ Reads an item count and refuses one that `unit` wire bytes per item cannot fit in what is left, so a count never sizes an allocation the bytes cannot back. */
  count(unit = 1): number {
    const count = this.varint();
    if (count * unit > this.remaining()) throw new RangeError(`a count of ${count} item(s) overruns the ${this.remaining()} byte(s) left at offset ${this.at}`);
    return count;
  }
}
// #endregion 🔖️Bytes

// #region 🔖️Crc32c
const CRC32C = (() => {
  const table = new Uint32Array(256);
  for (let index = 0; index < 256; index += 1) {
    let crc = index;
    for (let bit = 0; bit < 8; bit += 1) crc = crc & 1 ? (crc >>> 1) ^ 0x82f63b78 : crc >>> 1;
    table[index] = crc >>> 0;
  }
  return table;
})();

/** 🧮️ CRC-32C (Castagnoli, reflected) as every header, footer and segment frame stores it. */
export function crc32c(data: Uint8Array): number {
  let crc = 0xffffffff;
  for (const byte of data) crc = (crc >>> 8) ^ CRC32C[(crc ^ byte) & 0xff]!;
  return (crc ^ 0xffffffff) >>> 0;
}
// #endregion 🔖️Crc32c

// #region 🔖️Blake3
const B3_IV = Uint32Array.of(0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19);
const B3_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const B3_CHUNK_START = 1;
const B3_CHUNK_END = 2;
const B3_PARENT = 4;
const B3_ROOT = 8;

type B3Output = { chaining: Uint32Array; words: Uint32Array; counter: number; length: number; flags: number };

const rotr = (value: number, bits: number): number => ((value >>> bits) | (value << (32 - bits))) >>> 0;

function b3Compress(output: B3Output, extraFlags = 0): Uint32Array {
  const state = new Uint32Array(16);
  state.set(output.chaining.subarray(0, 8), 0);
  state.set(B3_IV.subarray(0, 4), 8);
  state[12] = output.counter >>> 0;
  state[13] = Math.floor(output.counter / 2 ** 32) >>> 0;
  state[14] = output.length;
  state[15] = (output.flags | extraFlags) >>> 0;
  const mix = (a: number, b: number, c: number, d: number, mx: number, my: number): void => {
    state[a] = (state[a]! + state[b]! + mx) >>> 0;
    state[d] = rotr(state[d]! ^ state[a]!, 16);
    state[c] = (state[c]! + state[d]!) >>> 0;
    state[b] = rotr(state[b]! ^ state[c]!, 12);
    state[a] = (state[a]! + state[b]! + my) >>> 0;
    state[d] = rotr(state[d]! ^ state[a]!, 8);
    state[c] = (state[c]! + state[d]!) >>> 0;
    state[b] = rotr(state[b]! ^ state[c]!, 7);
  };
  let block = Uint32Array.from(output.words);
  for (let round = 0; round < 7; round += 1) {
    mix(0, 4, 8, 12, block[0]!, block[1]!);
    mix(1, 5, 9, 13, block[2]!, block[3]!);
    mix(2, 6, 10, 14, block[4]!, block[5]!);
    mix(3, 7, 11, 15, block[6]!, block[7]!);
    mix(0, 5, 10, 15, block[8]!, block[9]!);
    mix(1, 6, 11, 12, block[10]!, block[11]!);
    mix(2, 7, 8, 13, block[12]!, block[13]!);
    mix(3, 4, 9, 14, block[14]!, block[15]!);
    if (round < 6) block = Uint32Array.from(B3_PERMUTATION, (index) => block[index]!);
  }
  for (let index = 0; index < 8; index += 1) {
    state[index] = (state[index]! ^ state[index + 8]!) >>> 0;
    state[index + 8] = (state[index + 8]! ^ output.chaining[index]!) >>> 0;
  }
  return state;
}

function b3Words(block: Uint8Array): Uint32Array {
  const padded = new Uint8Array(64);
  padded.set(block);
  const view = new DataView(padded.buffer);
  return Uint32Array.from({ length: 16 }, (_, index) => view.getUint32(index * 4, true));
}

function b3ChunkOutput(chunk: Uint8Array, counter: number): B3Output {
  let chaining: Uint32Array = B3_IV;
  const blocks: Uint8Array[] = [];
  for (let at = 0; at < chunk.length; at += 64) blocks.push(chunk.subarray(at, at + 64));
  if (blocks.length === 0) blocks.push(new Uint8Array(0));
  for (let index = 0; index < blocks.length - 1; index += 1) {
    chaining = b3Compress({ chaining, words: b3Words(blocks[index]!), counter, length: 64, flags: index === 0 ? B3_CHUNK_START : 0 }).subarray(0, 8);
  }
  const last = blocks[blocks.length - 1]!;
  return { chaining, words: b3Words(last), counter, length: last.length, flags: (blocks.length === 1 ? B3_CHUNK_START : 0) | B3_CHUNK_END };
}

const b3ChainingValue = (output: B3Output): Uint32Array => b3Compress(output).subarray(0, 8);

function b3ParentOutput(left: Uint32Array, right: Uint32Array): B3Output {
  const words = new Uint32Array(16);
  words.set(left, 0);
  words.set(right, 8);
  return { chaining: B3_IV, words, counter: 0, length: 64, flags: B3_PARENT };
}

/** #️⃣ BLAKE3 with the default 32-byte output, the digest of the document content and of the schema graph. */
export function blake3(data: Uint8Array): Uint8Array {
  const chunks: Uint8Array[] = [];
  for (let at = 0; at < data.length; at += 1024) chunks.push(data.subarray(at, at + 1024));
  if (chunks.length === 0) chunks.push(new Uint8Array(0));
  const stack: Uint32Array[] = [];
  for (let index = 0; index < chunks.length - 1; index += 1) {
    let value = b3ChainingValue(b3ChunkOutput(chunks[index]!, index));
    let completed = index + 1;
    while ((completed & 1) === 0) {
      value = b3ChainingValue(b3ParentOutput(stack.pop()!, value));
      completed >>>= 1;
    }
    stack.push(value);
  }
  let output = b3ChunkOutput(chunks[chunks.length - 1]!, chunks.length - 1);
  while (stack.length > 0) output = b3ParentOutput(stack.pop()!, b3ChainingValue(output));
  const words = b3Compress(output, B3_ROOT).subarray(0, 8);
  const out = new Uint8Array(32);
  words.forEach((word, index) => new DataView(out.buffer).setUint32(index * 4, word, true));
  return out;
}
// #endregion 🔖️Blake3

// #region 🔖️Deflate
const LENGTH_BASE = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const WINDOW = 32768;
const MIN_MATCH = 3;
const MAX_MATCH = 258;
const MAX_CHAIN = 128;
const HASH_BITS = 15;

function canonicalCodes(lengths: number[]): number[] {
  const counts = new Array<number>(Math.max(...lengths) + 1).fill(0);
  for (const length of lengths) if (length > 0) counts[length]! += 1;
  const next = new Array<number>(counts.length + 1).fill(0);
  let code = 0;
  for (let bits = 1; bits < counts.length; bits += 1) {
    code = (code + counts[bits - 1]!) << 1;
    next[bits] = code;
  }
  return lengths.map((length) => (length > 0 ? next[length]!++ : 0));
}

const LITERAL_LENGTHS = [...new Array<number>(144).fill(8), ...new Array<number>(112).fill(9), ...new Array<number>(24).fill(7), ...new Array<number>(8).fill(8)];
const LITERAL_CODES = canonicalCodes(LITERAL_LENGTHS);
const DISTANCE_CODES = canonicalCodes(new Array<number>(30).fill(5));

function tableIndex(bases: number[], value: number): number {
  for (let index = bases.length - 1; index >= 0; index -= 1) if (value >= bases[index]!) return index;
  return 0;
}

/** 🗜️ The pack's one deflate: a single fixed-Huffman block over a greedy hash-chain LZ77 (3..258 byte matches, 32 KiB window, 128 probes). Pinned by its own specification so equal input is equal output in every language. */
export function deflate(raw: Uint8Array): Uint8Array {
  const out: number[] = [];
  let buffer = 0;
  let bits = 0;
  const write = (value: number, count: number): void => {
    buffer |= (value & ((1 << count) - 1)) << bits;
    bits += count;
    while (bits >= 8) {
      out.push(buffer & 0xff);
      buffer >>>= 8;
      bits -= 8;
    }
  };
  const huffman = (code: number, length: number): void => {
    for (let shift = length - 1; shift >= 0; shift -= 1) write((code >>> shift) & 1, 1);
  };
  write(1, 1);
  write(1, 2);
  const size = raw.length;
  const head = new Int32Array(1 << HASH_BITS).fill(-1);
  const prev = new Int32Array(size).fill(-1);
  const slot = (at: number): number => Math.imul(raw[at]! | (raw[at + 1]! << 8) | (raw[at + 2]! << 16), 2654435761 | 0) >>> (32 - HASH_BITS);
  const insert = (at: number): void => {
    if (at + MIN_MATCH <= size) {
      const hash = slot(at);
      prev[at] = head[hash]!;
      head[hash] = at;
    }
  };
  let i = 0;
  while (i < size) {
    let bestLength = 0;
    let bestDistance = 0;
    if (i + MIN_MATCH <= size) {
      let candidate = head[slot(i)]!;
      let chain = 0;
      while (candidate !== -1 && chain < MAX_CHAIN) {
        if (i - candidate > WINDOW) break;
        const limit = Math.min(size - i, MAX_MATCH);
        let length = 0;
        while (length < limit && raw[candidate + length] === raw[i + length]) length += 1;
        if (length > bestLength) {
          bestLength = length;
          bestDistance = i - candidate;
        }
        candidate = prev[candidate]!;
        chain += 1;
      }
    }
    if (bestLength >= MIN_MATCH) {
      let index = tableIndex(LENGTH_BASE, bestLength);
      huffman(LITERAL_CODES[257 + index]!, LITERAL_LENGTHS[257 + index]!);
      if (LENGTH_EXTRA[index]! > 0) write(bestLength - LENGTH_BASE[index]!, LENGTH_EXTRA[index]!);
      index = tableIndex(DIST_BASE, bestDistance);
      huffman(DISTANCE_CODES[index]!, 5);
      if (DIST_EXTRA[index]! > 0) write(bestDistance - DIST_BASE[index]!, DIST_EXTRA[index]!);
      for (let at = i; at < i + bestLength; at += 1) insert(at);
      i += bestLength;
    } else {
      huffman(LITERAL_CODES[raw[i]!]!, LITERAL_LENGTHS[raw[i]!]!);
      insert(i);
      i += 1;
    }
  }
  huffman(LITERAL_CODES[256]!, LITERAL_LENGTHS[256]!);
  if (bits > 0) out.push(buffer & 0xff);
  return Uint8Array.from(out);
}

/** 📤️ Raw RFC 1951 inflate through the system zlib, the third-party reader the deflate above is measured against. */
export function inflate(stored: Uint8Array, rawLength: number): Uint8Array {
  if (rawLength > MAX_SEGMENT_RAW) throw new RangeError(`a deflate segment declares ${rawLength} byte(s), more than the ${MAX_SEGMENT_RAW} allowed`);
  const raw = new Uint8Array(inflateRawSync(stored, { maxOutputLength: Math.max(1, rawLength) }));
  if (raw.length !== rawLength) throw new RangeError(`a deflate segment declares ${rawLength} byte(s) but inflates to ${raw.length}`);
  return raw;
}
// #endregion 🔖️Deflate

// #region 🔖️Container
/** 🖇️ One framed segment: `kind, flags, stored length, [raw length], stored bytes, CRC-32C of everything before the CRC`. */
export function encodeSegment(kind: number, payload: Uint8Array, compressed = true): Uint8Array {
  const frame = compressed
    ? (() => {
        const stored = deflate(payload);
        return concat(Uint8Array.of(kind, 1 | (CODEC_DEFLATE << 1)), writeVarint(stored.length), writeVarint(payload.length), stored);
      })()
    : concat(Uint8Array.of(kind, 0), writeVarint(payload.length), payload);
  return concat(frame, u32Bytes(crc32c(frame)));
}

/** 🚪️ Reads one segment, checks its CRC and returns `{ kind, payload }`. */
export function decodeSegment(reader: Reader): { kind: number; payload: Uint8Array } {
  const start = reader.at;
  const kind = reader.byte();
  const flags = reader.byte();
  const storedLength = reader.varint();
  const compressed = (flags & 1) !== 0;
  const rawLength = compressed ? reader.varint() : storedLength;
  const stored = reader.take(storedLength);
  const expected = u32(reader.take(4), 0);
  if (crc32c(reader.data.subarray(start, reader.at - 4)) !== expected) throw new RangeError(`the segment at offset ${start} fails its CRC-32C`);
  if (!compressed) return { kind, payload: stored };
  if (((flags >> 1) & 7) !== CODEC_DEFLATE) throw new RangeError(`the segment at offset ${start} names the unsupported codec ${(flags >> 1) & 7}`);
  return { kind, payload: inflate(stored, rawLength) };
}

/** 📑️ The manifest span and count names, in wire order. */
const MANIFEST_FIELDS = ["docOffset", "docLength", "docFrames", "symbolsOffset", "symbolsLength", "tableOffset", "tableLength", "indexOffset", "indexLength", "bodyLength", "fieldCount", "chunkCount", "symbolCount"] as const;

/** 📋️ The decoded manifest. */
export type Manifest = Record<(typeof MANIFEST_FIELDS)[number], number> & { schemaHash: Uint8Array };

function encodeManifest(schemaHash: Uint8Array, values: Record<(typeof MANIFEST_FIELDS)[number], number>): Uint8Array {
  return concat(writeVarint(0), schemaHash, ...MANIFEST_FIELDS.map((name) => writeVarint(values[name])));
}

function decodeManifest(payload: Uint8Array): Manifest {
  const reader = new Reader(payload);
  reader.varint();
  const schemaHash = reader.take(32).slice();
  const values = Object.fromEntries(MANIFEST_FIELDS.map((name) => [name, reader.varint()])) as Record<(typeof MANIFEST_FIELDS)[number], number>;
  return { ...values, schemaHash };
}

/** 🏗️ Assembles the container in the order the writer lays it out: header, symbols, raw chunks, document frames, chunk table, manifest, end segment, footer. */
export function buildPack(schemaHash: Uint8Array, symbolsPayload: Uint8Array, body: Uint8Array, chunks: Uint8Array[]): Uint8Array {
  const required = REQUIRED_COMPRESSED | (chunks.length > 0 ? REQUIRED_CHUNKED : 0);
  const head = new Uint8Array(20);
  head.set(PACK_MAGIC, 0);
  const view = new DataView(head.buffer);
  view.setUint16(8, 1, true);
  view.setUint16(10, 0, true);
  view.setUint32(12, required, true);
  view.setUint32(16, OPTIONAL_CANONICAL, true);
  const header = concat(head, u32Bytes(crc32c(head)), new Uint8Array(HEADER_SIZE - 24));
  const parts: Uint8Array[] = [header];
  let length = header.length;
  const push = (part: Uint8Array): void => {
    parts.push(part);
    length += part.length;
  };
  const symbolsStart = length;
  push(encodeSegment(KIND_SYMBOLS, symbolsPayload));
  const symbolsLength = length - symbolsStart;
  const entries: { offset: number; size: number; crc: number; digest: Uint8Array }[] = [];
  for (const chunk of chunks) {
    const frameHead = concat(Uint8Array.of(KIND_CHUNK, 0), writeVarint(chunk.length));
    const offset = length + frameHead.length;
    push(concat(frameHead, chunk, u32Bytes(crc32c(concat(frameHead, chunk)))));
    entries.push({ offset, size: chunk.length, crc: crc32c(chunk), digest: blake3(chunk) });
  }
  const docStart = length;
  let frames = 0;
  for (let at = 0; at < body.length; at += FRAME_SIZE) {
    push(encodeSegment(KIND_DOCUMENT, body.subarray(at, at + FRAME_SIZE)));
    frames += 1;
  }
  const docLength = length - docStart;
  let tableOffset = 0;
  let tableLength = 0;
  if (entries.length > 0) {
    tableOffset = length;
    push(encodeSegment(KIND_CHUNK_TABLE, concat(writeVarint(entries.length), ...entries.map((entry) => concat(writeVarint(entry.offset), writeVarint(entry.size), writeVarint(entry.size), u32Bytes(entry.crc), entry.digest))), false));
    tableLength = length - tableOffset;
  }
  const manifestStart = length;
  push(
    encodeSegment(
      KIND_MANIFEST,
      encodeManifest(schemaHash, { docOffset: docStart, docLength, docFrames: frames, symbolsOffset: symbolsStart, symbolsLength, tableOffset, tableLength, indexOffset: 0, indexLength: 0, bodyLength: body.length, fieldCount: new Reader(body).varint(), chunkCount: entries.length, symbolCount: new Reader(symbolsPayload).varint() }),
    ),
  );
  const manifestLength = length - manifestStart;
  push(encodeSegment(KIND_END, new Uint8Array(0), false));
  const footerHead = new Uint8Array(40);
  footerHead.set(FOOTER_MAGIC, 0);
  const footerView = new DataView(footerHead.buffer);
  footerView.setUint16(8, 1, true);
  footerView.setUint16(10, 0, true);
  footerView.setUint32(12, required, true);
  footerView.setBigUint64(16, BigInt(manifestStart), true);
  footerView.setBigUint64(24, BigInt(manifestLength), true);
  footerView.setBigUint64(32, BigInt(length + FOOTER_SIZE), true);
  const footer = concat(footerHead, blake3(body), u64Bytes(0));
  push(concat(footer, u32Bytes(crc32c(footer))));
  return concat(...parts);
}

/** 🗄️ The verified parts of a container. */
export type OpenedPack = { manifest: Manifest; symbols: string[]; body: Uint8Array; chunks: Uint8Array[] };

/** 📂️ Verifies the container end to end — header CRC, footer CRC and span, every segment CRC, the document hash — and returns its symbol table and document body. */
export function openPack(inner: Uint8Array): OpenedPack {
  if (inner.length < HEADER_SIZE + FOOTER_SIZE) throw new RangeError(`the pack is ${inner.length} byte(s), shorter than its header and footer`);
  if (!PACK_MAGIC.every((byte, index) => inner[index] === byte)) throw new RangeError("the pack does not start with the SPK magic");
  const view = new DataView(inner.buffer, inner.byteOffset, inner.byteLength);
  if (crc32c(inner.subarray(0, 20)) !== view.getUint32(20, true) || view.getUint16(8, true) !== 1 || (view.getUint32(12, true) & ~0xf) !== 0) throw new RangeError("the pack header is corrupt or of an unsupported version");
  const footer = inner.subarray(inner.length - FOOTER_SIZE);
  if (!FOOTER_MAGIC.every((byte, index) => footer[index] === byte) || crc32c(footer.subarray(0, 80)) !== u32(footer, 80)) throw new RangeError("the pack footer is corrupt");
  const footerView = new DataView(footer.buffer, footer.byteOffset, footer.byteLength);
  const manifestOffset = Number(footerView.getBigUint64(16, true));
  const manifestLength = Number(footerView.getBigUint64(24, true));
  if (Number(footerView.getBigUint64(32, true)) !== inner.length) throw new RangeError(`the footer declares ${Number(footerView.getBigUint64(32, true))} byte(s) for a ${inner.length} byte pack`);
  const reader = new Reader(inner, HEADER_SIZE);
  let symbolsPayload: Uint8Array = new Uint8Array(0);
  const frames: Uint8Array[] = [];
  const chunks: Uint8Array[] = [];
  let manifest: Manifest | undefined;
  for (;;) {
    const at = reader.at;
    const { kind, payload } = decodeSegment(reader);
    if (kind === KIND_SYMBOLS) symbolsPayload = payload;
    else if (kind === KIND_DOCUMENT) frames.push(payload);
    else if (kind === KIND_CHUNK) chunks.push(payload);
    else if (kind === KIND_MANIFEST) {
      if (at !== manifestOffset || reader.at - at !== manifestLength) throw new RangeError("the manifest is not where the footer says");
      manifest = decodeManifest(payload);
    } else if (kind === KIND_END) break;
  }
  const body = concat(...frames);
  if (reader.at !== inner.length - FOOTER_SIZE || manifest === undefined) throw new RangeError("the pack carries bytes or lacks a manifest between its end segment and its footer");
  if (!blake3(body).every((byte, index) => byte === footer[40 + index])) throw new RangeError("the document body does not hash to the footer's content hash");
  if (manifest.bodyLength !== body.length) throw new RangeError("the manifest body length disagrees with the document");
  return { manifest, symbols: decodeSymbols(symbolsPayload), body, chunks };
}

function decodeSymbols(payload: Uint8Array): string[] {
  const reader = new Reader(payload);
  const symbols = Array.from({ length: reader.count() }, () => DECODER.decode(reader.take(reader.varint())));
  if (reader.remaining() > 0) throw new RangeError("the symbol table carries trailing bytes");
  return symbols;
}

/** 📦️ The semio binary envelope: magic, little-endian u32 token length, the `<envelope>.pack v1` token. */
export function wrapBinary(token: string, inner: Uint8Array): Uint8Array {
  const raw = ENCODER.encode(token);
  return concat(BINARY_MAGIC, u32Bytes(raw.length), raw, inner);
}

/** 🪆️ Splits the semio binary envelope into its token and the container. */
export function unwrapBinary(data: Uint8Array): { token: string; inner: Uint8Array } {
  if (data.length < 12 || !BINARY_MAGIC.every((byte, index) => data[index] === byte)) throw new RangeError("the file does not start with the semio binary magic");
  const length = u32(data, 8);
  if (12 + length > data.length) throw new RangeError("the semio binary envelope ends inside its token");
  return { token: DECODER.decode(data.subarray(12, 12 + length)), inner: data.subarray(12 + length) };
}
// #endregion 🔖️Container

// #region 🔖️Spec
const textBytes = (value: string): Uint8Array => {
  const raw = ENCODER.encode(value);
  return concat(writeVarint(raw.length), raw);
};

/** 🧬️ One shape in the canonical bytes the schema hash covers. */
function canonicalShape(node: Shape): Uint8Array {
  const tag = Uint8Array.of(SHAPE_TAGS[node.kind]!);
  switch (node.kind) {
    case "enum":
      return concat(tag, writeVarint(node.variants.length), ...node.variants.map(([ordinal, name]) => concat(writeVarint(ordinal), textBytes(name))));
    case "tuple":
      return concat(tag, canonicalShape(node.item), node.len === null ? Uint8Array.of(0) : concat(Uint8Array.of(1), writeVarint(node.len)));
    case "list":
    case "block":
    case "map":
      return concat(tag, canonicalShape(node.item));
    case "record":
    case "table":
      return concat(tag, writeVarint(node.record));
    case "statements":
      return concat(tag, writeVarint(node.variants.length), ...node.variants.map(([keyword, index]) => concat(textBytes(keyword), writeVarint(index))));
    case "quantity":
    case "angle":
      return concat(tag, textBytes(node.unit));
    case "ref":
      return concat(tag, textBytes(node.entity));
    case "coord":
    case "dim":
      return concat(tag, Uint8Array.of(node.dims));
    case "embed":
      return concat(tag, textBytes(node.lang));
    case "embedFrom":
      return concat(tag, textBytes(node.key));
    default:
      return tag;
  }
}

/** 🧭️ The exported record spec: the canonical record graph of a subset snapshot. Record `0` is the root; every `record`, `table` and `statements` shape names another record by index. */
export class RecordSpec {
  readonly envelope: string;
  readonly expectedHash: string;
  readonly graph: Field[][];
  readonly fields: Map<number, Field>[];
  readonly single: Set<string>;
  readonly float32: Set<string>;
  readonly tags: Map<string, string>;
  readonly verbatim: Set<string>;
  readonly nested: Map<number, readonly [string, readonly string[]]>;

  constructor(document: RecordSpecDocument) {
    if (document.schema !== "semio.pack.record-spec/v1") throw new RangeError("not a semio.pack.record-spec/v1 document");
    this.envelope = document.envelope;
    this.expectedHash = document.schemaHash;
    this.graph = document.graph;
    this.fields = document.graph.map((record) => new Map(record.map((field) => [field.id, field])));
    this.single = new Set((document.annotations?.single ?? []).map(([record, id]) => `${record}.${id}`));
    this.float32 = new Set((document.annotations?.float32 ?? []).map(([record, id]) => `${record}.${id}`));
    this.tags = new Map((document.annotations?.tags ?? []).map(([record, id, name]) => [`${record}.${id}`, name]));
    this.verbatim = new Set((document.annotations?.verbatim ?? []).map(([record, id]) => `${record}.${id}`));
    this.nested = new Map((document.annotations?.nested ?? []).map(([record, name, members]) => [record, [name, members] as const]));
  }

  get token(): string {
    return `${this.envelope}.pack v1`;
  }

  /** 🧾️ `varint(records)` then per record `varint(fields)` and per field `varint(id) text(key) flags shape` — the bytes the schema hash covers. */
  canonicalBytes(): Uint8Array {
    return concat(writeVarint(this.graph.length), ...this.graph.map((record) => concat(writeVarint(record.length), ...record.map((field) => concat(writeVarint(field.id), textBytes(field.key), Uint8Array.of((field.optional ? 1 : 0) | (field.flatten ? 2 : 0)), canonicalShape(field.shape))))));
  }

  /** 🔑️ BLAKE3 over the canonical bytes, recomputed here and required to equal the hash the export carries. */
  schemaHash(): Uint8Array {
    return blake3(this.canonicalBytes());
  }
}

const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");

/** 📥️ Builds a spec from one committed `🗺️record-spec/🔣️.json` and refuses it when its carried hash is not the one its graph hashes to. */
export function loadRecordSpec(document: RecordSpecDocument): RecordSpec {
  const spec = new RecordSpec(document);
  if (hex(spec.schemaHash()) !== spec.expectedHash) throw new RangeError(`the record spec graph hashes to ${hex(spec.schemaHash())}, not the carried ${spec.expectedHash}`);
  return spec;
}
// #endregion 🔖️Spec

// #region 🔖️Encode
/** ➗️ Whether a sequence is the mandatory packed form: every element a float (`f64`) or every element an int, enum or `u64 <= i64::MAX` (`varint`); empty runs stay plain. */
function numericRun(items: FieldValue[]): "f64" | "varint" | undefined {
  if (items.length === 0) return undefined;
  const kinds = new Set(items.map((item) => item.kind));
  if (kinds.size !== 1) return undefined;
  const [kind] = kinds;
  if (kind === "float") return "f64";
  if (kind === "int" || kind === "enum") return "varint";
  if (kind === "uint" && items.every((item) => BigInt(item.value as number | bigint) <= I64_MAX)) return "varint";
  return undefined;
}

/** 🏷️ The column element tag of a table column from its declared shape. */
function elemTag(shape: Shape): number {
  if (FLOAT_SHAPES.has(shape.kind)) return ELEM.F64;
  if (shape.kind === "int") return ELEM.INT;
  if (UINT_SHAPES.has(shape.kind)) return ELEM.UINT;
  if (shape.kind === "enum") return ELEM.ENUM;
  if (shape.kind === "bool") return ELEM.BOOL;
  if (TEXT_SHAPES.has(shape.kind)) return ELEM.STR;
  return ELEM.FALLBACK;
}

const elementShape = (shape: Shape | undefined): Shape | undefined => (shape !== undefined && (shape.kind === "tuple" || shape.kind === "list") ? shape.item : undefined);
const utf8Compare = (left: string, right: string): number => {
  const a = ENCODER.encode(left);
  const b = ENCODER.encode(right);
  for (let index = 0; index < Math.min(a.length, b.length); index += 1) if (a[index] !== b[index]) return a[index]! - b[index]!;
  return a.length - b.length;
};

function dslStrings(value: DslValue, counts: Map<string, number>): void {
  if (typeof value === "string") counts.set(value, (counts.get(value) ?? 0) + 1);
  else if (Array.isArray(value)) for (const item of value) dslStrings(item, counts);
  else if (value !== null && typeof value === "object" && !(value instanceof Uint8Array) && !(value instanceof DslFloat)) for (const item of Object.values(value)) dslStrings(item, counts);
}

/** ✍️ One document encode: the deterministic interning pre-pass, then the tag-prefixed, canonical record body. */
class Encoder {
  symbols: string[] = [];
  index = new Map<string, number>();
  chunks: Uint8Array[] = [];

  constructor(readonly spec: RecordSpec) {}

  intern(record: RecordFields): void {
    const counts = new Map<string, number>();
    const forced = new Set<string>();
    const note = (text: string): void => void counts.set(text, (counts.get(text) ?? 0) + 1);
    const force = (text: string): void => {
      forced.add(text);
      note(text);
    };
    const walkRecord = (specIndex: number | undefined, fields: RecordFields): void => {
      for (const id of [...fields.keys()].sort((a, b) => a - b)) walk(specIndex === undefined ? undefined : this.spec.fields[specIndex]!.get(id)?.shape, fields.get(id)!);
    };
    const walk = (shape: Shape | undefined, value: FieldValue): void => {
      switch (value.kind) {
        case "text":
          note(value.value);
          break;
        case "tuple":
          for (const item of value.value) walk(elementShape(shape), item);
          break;
        case "list":
          if (shape?.kind === "table") {
            for (const row of value.value) {
              if (row.kind !== "record") continue;
              for (const column of this.spec.graph[shape.record]!) {
                const cell = row.value.get(column.id);
                if (cell === undefined) continue;
                if (TEXT_SHAPES.has(column.shape.kind)) {
                  if (cell.kind === "text") force(cell.value);
                } else walk(column.shape, cell);
              }
            }
          } else for (const item of value.value) walk(elementShape(shape), item);
          break;
        case "record":
          walkRecord(shape?.kind === "record" ? shape.record : undefined, value.value);
          break;
        case "block":
          walk(shape?.kind === "block" ? shape.item : undefined, value.value);
          break;
        case "statements": {
          const variants = new Map(shape?.kind === "statements" ? shape.variants : []);
          for (const [keyword, fields] of value.value) {
            force(keyword);
            walkRecord(variants.get(keyword), fields);
          }
          break;
        }
        case "map":
          for (const [key, item] of value.value) {
            note(key);
            walk(shape?.kind === "map" ? shape.item : undefined, item);
          }
          break;
        case "value":
          dslStrings(value.value, counts);
          break;
        default:
      }
    };
    walkRecord(0, record);
    const chosen = new Set(forced);
    for (const [text, count] of counts) if (ENCODER.encode(text).length <= 128 || count >= 2) chosen.add(text);
    this.symbols = [...chosen].sort(utf8Compare);
    this.index = new Map(this.symbols.map((text, position) => [text, position]));
  }

  string(text: string): Uint8Array {
    const position = this.index.get(text);
    return position === undefined ? this.inline(text) : concat(Uint8Array.of(TAG.STR), writeVarint(position));
  }

  inline(text: string): Uint8Array {
    const raw = ENCODER.encode(text);
    return concat(Uint8Array.of(TAG.STR_INLINE), writeVarint(raw.length), raw);
  }

  forcedSymbol(text: string): Uint8Array {
    const position = this.index.get(text);
    if (position === undefined) throw new RangeError(`symbol ${JSON.stringify(text)} is missing from the precomputed table`);
    return writeVarint(position);
  }

  recordFields(specIndex: number | undefined, fields: RecordFields, depth: number): Uint8Array {
    if (depth > MAX_DEPTH) throw new RangeError(`the record nests deeper than ${MAX_DEPTH}`);
    const known = specIndex === undefined ? undefined : this.spec.fields[specIndex]!;
    const ids = [...fields.keys()].filter((id) => known === undefined || known.has(id)).sort((a, b) => a - b);
    return concat(writeVarint(ids.length), ...ids.map((id) => concat(writeVarint(id), this.value(known?.get(id)?.shape, fields.get(id)!, depth + 1))));
  }

  value(shape: Shape | undefined, field: FieldValue, depth: number): Uint8Array {
    if (depth > MAX_DEPTH) throw new RangeError(`the record nests deeper than ${MAX_DEPTH}`);
    switch (field.kind) {
      case "bool":
        return Uint8Array.of(field.value ? TAG.TRUE : TAG.FALSE);
      case "int":
        return concat(Uint8Array.of(TAG.INT), writeZigzag(field.value));
      case "uint":
        return concat(Uint8Array.of(TAG.UINT), writeVarint(field.value));
      case "float":
        return concat(Uint8Array.of(TAG.F64), f64Bytes(field.value));
      case "text":
        return this.string(field.value);
      case "bytes":
        return this.bytesValue(field.value);
      case "enum":
        return concat(Uint8Array.of(TAG.ENUM), writeVarint(field.value));
      case "tuple":
        return this.sequence(shape, field.value, true, depth);
      case "list":
        return shape?.kind === "table" ? this.table(shape.record, field.value, depth) : this.sequence(shape, field.value, false, depth);
      case "record":
        return concat(Uint8Array.of(TAG.RECORD), this.recordFields(shape?.kind === "record" ? shape.record : undefined, field.value, depth + 1));
      case "block":
        return concat(Uint8Array.of(TAG.BLOCK), this.value(shape?.kind === "block" ? shape.item : undefined, field.value, depth + 1));
      case "statements": {
        const variants = new Map(shape?.kind === "statements" ? shape.variants : []);
        return concat(Uint8Array.of(TAG.STATEMENTS), writeVarint(field.value.length), ...field.value.map(([keyword, fields]) => concat(this.forcedSymbol(keyword), this.recordFields(variants.get(keyword), fields, depth + 1))));
      }
      case "map": {
        const inner = shape?.kind === "map" ? shape.item : undefined;
        const entries = [...field.value].sort((a, b) => utf8Compare(a[0], b[0]));
        return concat(Uint8Array.of(TAG.MAP), writeVarint(entries.length), ...entries.map(([key, item]) => concat(this.string(key), this.value(inner, item, depth + 1))));
      }
      case "value":
        return concat(Uint8Array.of(TAG.VALUE), this.dslValue(field.value, depth + 1));
    }
  }

  bytesValue(payload: Uint8Array): Uint8Array {
    if (payload.length < CHUNK_THRESHOLD) return concat(Uint8Array.of(TAG.BYTES), writeVarint(payload.length), payload);
    const ids: number[] = [];
    for (let at = 0; at < payload.length; at += CHUNK_SIZE) {
      this.chunks.push(payload.subarray(at, at + CHUNK_SIZE));
      ids.push(this.chunks.length - 1);
    }
    return concat(Uint8Array.of(TAG.BYTES_CHUNKED), writeVarint(ids.length), ...ids.map((id) => writeVarint(id)));
  }

  sequence(shape: Shape | undefined, items: FieldValue[], isTuple: boolean, depth: number): Uint8Array {
    const run = numericRun(items);
    if (run === "f64") return concat(Uint8Array.of(TAG.PACKED_F64), writeVarint(items.length), ...items.map((item) => f64Bytes(item.value as number)));
    if (run === "varint") return concat(Uint8Array.of(TAG.PACKED_VARINT), writeVarint(items.length), ...items.map((item) => writeZigzag(item.value as number | bigint)));
    const inner = elementShape(shape);
    return concat(Uint8Array.of(isTuple ? TAG.TUPLE : TAG.LIST), writeVarint(items.length), ...items.map((item) => this.value(inner, item, depth + 1)));
  }

  table(specIndex: number, rows: FieldValue[], depth: number): Uint8Array {
    const columns = [...this.spec.graph[specIndex]!].sort((a, b) => a.id - b.id);
    const parts: Uint8Array[] = [Uint8Array.of(TAG.TABLE_SOA), writeVarint(rows.length), writeVarint(columns.length)];
    for (const column of columns) {
      const cells = rows.map((row) => (row.kind === "record" ? row.value.get(column.id) : undefined));
      const present = cells.map((cell) => cell !== undefined);
      parts.push(writeVarint(column.id));
      if (present.every(Boolean)) parts.push(Uint8Array.of(0));
      else {
        const bitmap = new Uint8Array(Math.ceil(rows.length / 8));
        present.forEach((flag, position) => {
          if (flag) bitmap[position >> 3]! |= 1 << (position & 7);
        });
        parts.push(Uint8Array.of(1), bitmap);
      }
      const tag = elemTag(column.shape);
      parts.push(Uint8Array.of(tag));
      if (tag === ELEM.BOOL) {
        const bits = new Uint8Array(Math.ceil(rows.length / 8));
        cells.forEach((cell, position) => {
          if (cell?.kind === "bool" && cell.value) bits[position >> 3]! |= 1 << (position & 7);
        });
        parts.push(bits);
        continue;
      }
      for (const cell of cells) {
        if (cell === undefined) continue;
        if (tag === ELEM.F64) parts.push(f64Bytes(cell.value as number));
        else if (tag === ELEM.INT) parts.push(writeZigzag(cell.value as number | bigint));
        else if (tag === ELEM.UINT || tag === ELEM.ENUM) parts.push(writeVarint(cell.value as number | bigint));
        else if (tag === ELEM.STR) parts.push(this.forcedSymbol(cell.value as string));
        else parts.push(this.value(column.shape, cell, depth + 1));
      }
    }
    return concat(...parts);
  }

  dslValue(value: DslValue, depth: number): Uint8Array {
    if (depth > MAX_DEPTH) throw new RangeError(`the value nests deeper than ${MAX_DEPTH}`);
    if (value === null) return Uint8Array.of(TAG.NULL);
    if (typeof value === "boolean") return Uint8Array.of(value ? TAG.TRUE : TAG.FALSE);
    if (value instanceof DslFloat) return concat(Uint8Array.of(TAG.F64), f64Bytes(value.value));
    if (typeof value === "bigint") return value >= 0n ? concat(Uint8Array.of(TAG.UINT), writeVarint(value)) : concat(Uint8Array.of(TAG.INT), writeZigzag(value));
    if (typeof value === "number") {
      if (!Number.isInteger(value)) return concat(Uint8Array.of(TAG.F64), f64Bytes(value));
      return value >= 0 ? concat(Uint8Array.of(TAG.UINT), writeVarint(value)) : concat(Uint8Array.of(TAG.INT), writeZigzag(value));
    }
    if (typeof value === "string") return this.string(value);
    if (value instanceof Uint8Array) return this.bytesValue(value);
    if (Array.isArray(value)) return concat(Uint8Array.of(TAG.LIST), writeVarint(value.length), ...value.map((item) => this.dslValue(item, depth + 1)));
    const entries = Object.entries(value);
    return concat(Uint8Array.of(TAG.MAP), writeVarint(entries.length), ...entries.map(([key, item]) => concat(this.inline(key), this.dslValue(item, depth + 1))));
  }
}

/** 🔐️ Encodes a record tree to the container (no envelope): symbols, document frames, chunks, manifest, footer. */
export function encodeRecord(spec: RecordSpec, record: RecordFields): Uint8Array {
  const encoder = new Encoder(spec);
  encoder.intern(record);
  const body = encoder.recordFields(0, record, 0);
  const symbols = concat(writeVarint(encoder.symbols.length), ...encoder.symbols.map(textBytes));
  return buildPack(spec.schemaHash(), symbols, body, encoder.chunks);
}
// #endregion 🔖️Encode

// #region 🔖️Decode
/** 🔬️ One document decode against a record spec: tags are self-describing, the spec only settles tuple versus list, packed element types and nested record shapes. */
class Decoder {
  constructor(
    readonly spec: RecordSpec,
    readonly symbols: string[],
    readonly chunks: Uint8Array[],
  ) {}

  symbol(index: number): string {
    const found = this.symbols[index];
    if (found === undefined) throw new RangeError(`symbol reference ${index} is outside the ${this.symbols.length} symbol table`);
    return found;
  }

  text(reader: Reader): string {
    const tag = reader.byte();
    if (tag === TAG.STR) return this.symbol(reader.varint());
    if (tag === TAG.STR_INLINE) return DECODER.decode(reader.take(reader.varint()));
    throw new RangeError(`expected a string tag, found 0x${tag.toString(16)} at offset ${reader.at}`);
  }

  recordFields(reader: Reader, specIndex: number | undefined, depth: number): RecordFields {
    if (depth > MAX_DEPTH) throw new RangeError(`the record nests deeper than ${MAX_DEPTH}`);
    const fields: RecordFields = new Map();
    const known = specIndex === undefined ? undefined : this.spec.fields[specIndex]!;
    for (let count = reader.count(); count > 0; count -= 1) {
      const id = reader.varint();
      const value = this.value(reader, known?.get(id)?.shape, depth + 1);
      if (value !== undefined) fields.set(id, value);
    }
    return fields;
  }

  value(reader: Reader, shape: Shape | undefined, depth: number): FieldValue | undefined {
    if (depth > MAX_DEPTH) throw new RangeError(`the record nests deeper than ${MAX_DEPTH}`);
    const tag = reader.byte();
    const kind = shape?.kind;
    switch (tag) {
      case TAG.ABSENT:
        return undefined;
      case TAG.FALSE:
      case TAG.TRUE:
        return { kind: "bool", value: tag === TAG.TRUE };
      case TAG.INT:
        return { kind: "int", value: narrow(reader.zigzagBig()) };
      case TAG.UINT:
        return { kind: "uint", value: narrow(reader.varintBig()) };
      case TAG.F64:
        return { kind: "float", value: reader.float64() };
      case TAG.STR:
        return { kind: "text", value: this.symbol(reader.varint()) };
      case TAG.STR_INLINE:
        return { kind: "text", value: DECODER.decode(reader.take(reader.varint())) };
      case TAG.BYTES:
        return { kind: "bytes", value: reader.take(reader.varint()).slice() };
      case TAG.BYTES_CHUNKED: {
        const ids = Array.from({ length: reader.count() }, () => reader.varint());
        return { kind: "bytes", value: concat(...ids.map((id) => this.chunk(id))) };
      }
      case TAG.ENUM:
        return { kind: "enum", value: reader.varint() };
      case TAG.TUPLE:
      case TAG.LIST: {
        const inner: Shape | undefined = tag === TAG.LIST && shape?.kind === "table" ? { kind: "record", record: shape.record } : elementShape(shape);
        return { kind: tag === TAG.TUPLE ? "tuple" : "list", value: Array.from({ length: reader.count() }, () => this.value(reader, inner, depth + 1)!) };
      }
      case TAG.RECORD:
        return { kind: "record", value: this.recordFields(reader, shape?.kind === "record" ? shape.record : undefined, depth + 1) };
      case TAG.BLOCK:
        return { kind: "block", value: this.value(reader, shape?.kind === "block" ? shape.item : undefined, depth + 1)! };
      case TAG.STATEMENTS: {
        const variants = new Map(shape?.kind === "statements" ? shape.variants : []);
        const items: [string, RecordFields][] = [];
        for (let count = reader.count(); count > 0; count -= 1) {
          const keyword = this.symbol(reader.varint());
          items.push([keyword, this.recordFields(reader, variants.get(keyword), depth + 1)]);
        }
        return { kind: "statements", value: items };
      }
      case TAG.MAP: {
        const entries: [string, FieldValue][] = [];
        for (let count = reader.count(); count > 0; count -= 1) {
          const key = this.text(reader);
          entries.push([key, this.value(reader, shape?.kind === "map" ? shape.item : undefined, depth + 1)!]);
        }
        return { kind: "map", value: entries };
      }
      case TAG.VALUE:
        return { kind: "value", value: this.dslValue(reader, depth + 1) };
      case TAG.TABLE_SOA:
        return { kind: "list", value: this.table(reader, shape?.kind === "table" ? shape.record : undefined, depth) };
      case TAG.PACKED_F64:
        return { kind: kind !== undefined && TUPLE_SHAPES.has(kind) ? "tuple" : "list", value: Array.from({ length: reader.count(8) }, () => ({ kind: "float", value: reader.float64() }) as FieldValue) };
      case TAG.PACKED_VARINT: {
        const inner = kind === "tuple" || kind === "list" ? elementShape(shape) : shape;
        const itemKind = inner?.kind === "uint" ? "uint" : inner?.kind === "enum" ? "enum" : "int";
        const items = Array.from({ length: reader.count() }, () => {
          const number = reader.zigzagBig();
          if (itemKind !== "int" && number < 0n) throw new RangeError(`a packed ${itemKind} run holds the negative value ${number}`);
          return itemKind === "enum" ? ({ kind: "enum", value: Number(number) } as FieldValue) : ({ kind: itemKind, value: narrow(number) } as FieldValue);
        });
        return { kind: kind !== undefined && TUPLE_SHAPES.has(kind) ? "tuple" : "list", value: items };
      }
      default:
        throw new RangeError(`unrecognized value tag 0x${tag.toString(16)} at offset ${reader.at}`);
    }
  }

  chunk(id: number): Uint8Array {
    const chunk = this.chunks[id];
    if (chunk === undefined) throw new RangeError(`the chunk ${id} is not among the ${this.chunks.length} the pack carries`);
    return chunk;
  }

  table(reader: Reader, specIndex: number | undefined, depth: number): FieldValue[] {
    const rowsCount = reader.varint();
    const columnsCount = reader.count();
    if (rowsCount > reader.remaining() * 8) throw new RangeError(`a table of ${rowsCount} row(s) overruns the ${reader.remaining()} byte(s) left at offset ${reader.at}`);
    const rows: RecordFields[] = Array.from({ length: rowsCount }, () => new Map());
    const known = specIndex === undefined ? undefined : this.spec.fields[specIndex]!;
    for (let column = 0; column < columnsCount; column += 1) {
      const id = reader.varint();
      const dense = reader.byte() === 0;
      const bitmap = dense ? undefined : reader.take(Math.ceil(rowsCount / 8));
      const tag = reader.byte();
      const present = (position: number): boolean => bitmap === undefined || (bitmap[position >> 3]! & (1 << (position & 7))) !== 0;
      if (tag === ELEM.BOOL) {
        const bits = reader.take(Math.ceil(rowsCount / 8));
        for (let position = 0; position < rowsCount; position += 1) if (present(position)) rows[position]!.set(id, { kind: "bool", value: (bits[position >> 3]! & (1 << (position & 7))) !== 0 });
        continue;
      }
      for (let position = 0; position < rowsCount; position += 1) {
        if (!present(position)) continue;
        const row = rows[position]!;
        if (tag === ELEM.F64) row.set(id, { kind: "float", value: reader.float64() });
        else if (tag === ELEM.INT) row.set(id, { kind: "int", value: narrow(reader.zigzagBig()) });
        else if (tag === ELEM.UINT) row.set(id, { kind: "uint", value: narrow(reader.varintBig()) });
        else if (tag === ELEM.ENUM) row.set(id, { kind: "enum", value: reader.varint() });
        else if (tag === ELEM.STR) row.set(id, { kind: "text", value: this.symbol(reader.varint()) });
        else {
          const value = this.value(reader, known?.get(id)?.shape, depth + 1);
          if (value !== undefined) row.set(id, value);
        }
      }
    }
    return rows.map((value) => ({ kind: "record", value }) as FieldValue);
  }

  dslValue(reader: Reader, depth: number): DslValue {
    if (depth > MAX_DEPTH) throw new RangeError(`the value nests deeper than ${MAX_DEPTH}`);
    const tag = reader.byte();
    switch (tag) {
      case TAG.NULL:
        return null;
      case TAG.FALSE:
      case TAG.TRUE:
        return tag === TAG.TRUE;
      case TAG.UINT:
        return narrow(reader.varintBig());
      case TAG.INT:
        return narrow(reader.zigzagBig());
      case TAG.F64:
        return new DslFloat(reader.float64());
      case TAG.STR:
        return this.symbol(reader.varint());
      case TAG.STR_INLINE:
        return DECODER.decode(reader.take(reader.varint()));
      case TAG.BYTES:
        return reader.take(reader.varint()).slice();
      case TAG.LIST:
        return Array.from({ length: reader.count() }, () => this.dslValue(reader, depth + 1));
      case TAG.MAP: {
        const entries: Record<string, DslValue> = Object.create(null);
        for (let count = reader.count(); count > 0; count -= 1) {
          const key = this.text(reader);
          entries[key] = this.dslValue(reader, depth + 1);
        }
        return entries;
      }
      default:
        throw new RangeError(`unexpected dynamic value tag 0x${tag.toString(16)} at offset ${reader.at}`);
    }
  }
}

/** 🔟️ A decoded 64-bit integer as a plain number while it is exact, a bigint beyond. */
function narrow(value: bigint): number | bigint {
  return value >= BigInt(Number.MIN_SAFE_INTEGER) && value <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(value) : value;
}

/** 🔓️ Decodes a container (no envelope) to its record tree, refusing a schema hash other than the spec's and any trailing document byte. */
export function decodeRecord(spec: RecordSpec, inner: Uint8Array): RecordFields {
  const pack = openPack(inner);
  if (!spec.schemaHash().every((byte, index) => byte === pack.manifest.schemaHash[index])) throw new RangeError("the pack was written for another record schema");
  const reader = new Reader(pack.body);
  const record = new Decoder(spec, pack.symbols, pack.chunks).recordFields(reader, 0, 0);
  if (reader.remaining() > 0) throw new RangeError(`${reader.remaining()} trailing byte(s) after the document record`);
  return record;
}
// #endregion 🔖️Decode

// #region 🔖️Document
/** 🐫️ `snake_case` and `kebab-case` spec keys and keywords as the camelCase JSON names the snapshot documents use. */
export function camel(name: string): string {
  const [head = "", ...rest] = name.replaceAll("-", "_").split("_");
  return head + rest.map((part) => part.slice(0, 1).toUpperCase() + part.slice(1)).join("");
}

/** 🧹️ A name with its case and separators dropped, the one comparison between a document tag and a spec tag. */
const normal = (name: string): string => name.replaceAll("-", "").replaceAll("_", "").toLowerCase();

type Json = unknown;
type JsonObject = { [key: string]: Json };

/** 🕳️ An absent `Option<enum>` is a present field holding no statement, wrapped in its block where the shape has one. */
const emptyStatements = (shape: Shape): FieldValue => (shape.kind === "block" ? { kind: "block", value: { kind: "statements", value: [] } } : { kind: "statements", value: [] });

const omittable = (shape: Shape): boolean => ["list", "table", "map", "statements", "block"].includes(shape.kind);
const isObject = (value: Json): value is JsonObject => typeof value === "object" && value !== null && !Array.isArray(value) && !(value instanceof Uint8Array);

/** 🪶️ The double whose shortest decimal still round-trips the single-precision `value`, the number a document spells for an `f32` field (`0.1`, not `0.10000000149011612`). */
export function shortestF32(value: number): number {
  for (let digits = 1; digits < 10; digits += 1) {
    const candidate = Number(value.toPrecision(digits));
    if (Math.fround(candidate) === value) return candidate;
  }
  return value;
}

/** ⚙️ How a decoded document spells what the record leaves open: an absent optional as no key instead of `null`, and `f32` fields as their shortest decimal instead of the widened double. */
export type DocumentOptions = { omitNulls?: boolean; narrowF32?: boolean };

/** 📄️ The two-way mapping between a camelCase JSON snapshot and the record tree, driven by the record spec alone.
 *
 * - a record field is the JSON key `camel(field key)`; an absent optional field is JSON `null`, or no key at all with `omitNulls` (and `null` or a missing key is absent either way);
 * - the payload keys of a statements variant are camelCase like every other record, except on the fields the spec's `verbatim` annotation lists
 *   (serde's `rename_all_fields` is absent there, so the Rust field name, `style_id`, is the JSON key);
 * - a record the spec's `nested` annotation lists keeps some of its fields in a JSON sub-object (`ArtifactRef` flattens its `dialect` on the wire);
 * - a unit-enum shape is its tag string; a statements shape is an object `{ kind: <tag>, ...variant fields }`, an array of them when the
 *   field holds many, and the single object when the `single` annotation says the field holds exactly one;
 * - every float shape is a JSON number; a field the spec's `float32` annotation lists is single precision, so the encoder narrows its number to `f32` first and the decoder
 *   returns the widened double, or with `narrowF32` the shortest decimal that round-trips the `f32` (`0.1`, as serde writes it). */
export class Document {
  constructor(
    readonly spec: RecordSpec,
    readonly options: DocumentOptions = {},
  ) {}

  private unnest(specIndex: number, document: JsonObject): JsonObject {
    const nested = this.spec.nested.get(specIndex);
    const inner = nested === undefined ? undefined : document[nested[0]];
    if (nested === undefined || !isObject(inner)) return document;
    return { ...Object.fromEntries(Object.entries(document).filter(([name]) => name !== nested[0])), ...inner };
  }

  record(specIndex: number, input: Json, path = "$", verbatim = false): RecordFields {
    if (!isObject(input)) throw new RangeError(`${path}: expected an object for record ${specIndex}`);
    const document = this.unnest(specIndex, input);
    const fields: RecordFields = new Map();
    const known = new Map(this.spec.graph[specIndex]!.map((field) => [verbatim ? field.key.replaceAll("-", "_") : camel(field.key), field]));
    const extra = Object.keys(document).filter((name) => !known.has(name));
    if (extra.length > 0) throw new RangeError(`${path}: the document carries ${JSON.stringify(extra)}, which record ${specIndex} does not declare`);
    for (const [name, field] of known) {
      const raw = document[name];
      if (raw === null || raw === undefined) {
        if (!field.optional && !omittable(field.shape)) throw new RangeError(`${path}.${name}: the required field is missing`);
        if (this.spec.single.has(`${specIndex}.${field.id}`)) fields.set(field.id, emptyStatements(field.shape));
        continue;
      }
      fields.set(field.id, this.value(field.shape, raw, `${specIndex}.${field.id}`, `${path}.${name}`));
    }
    return fields;
  }

  value(shape: Shape, raw: Json, at: string, path: string): FieldValue {
    switch (shape.kind) {
      case "bool":
        return { kind: "bool", value: Boolean(raw) };
      case "int":
        return { kind: "int", value: Number(raw) };
      case "uint":
      case "count":
        return { kind: "uint", value: Number(raw) };
      case "float":
      case "quantity":
      case "angle":
        return { kind: "float", value: this.spec.float32.has(at) ? Math.fround(Number(raw)) : Number(raw) };
      case "text":
      case "ref":
        return { kind: "text", value: String(raw) };
      case "bytes64":
        return { kind: "bytes", value: Uint8Array.from(raw as number[]) };
      case "enum": {
        const found = shape.variants.find(([, tag]) => normal(tag) === normal(String(raw)));
        if (found === undefined) throw new RangeError(`${path}: ${JSON.stringify(raw)} is not one of ${JSON.stringify(shape.variants.map(([, tag]) => tag))}`);
        return { kind: "enum", value: found[0] };
      }
      case "tuple":
        return { kind: "tuple", value: (raw as Json[]).map((item, index) => this.value(shape.item, item, at, `${path}[${index}]`)) };
      case "coord":
      case "dir":
      case "dim":
      case "range":
        return { kind: "tuple", value: (raw as Json[]).map((item, index) => this.value({ kind: "float" }, item, at, `${path}[${index}]`)) };
      case "list":
        return { kind: "list", value: (raw as Json[]).map((item, index) => this.value(shape.item, item, at, `${path}[${index}]`)) };
      case "table":
        return { kind: "list", value: (raw as Json[]).map((item, index) => ({ kind: "record", value: this.record(shape.record, item, `${path}[${index}]`) })) };
      case "record":
        return { kind: "record", value: this.record(shape.record, raw, path) };
      case "block":
        return { kind: "block", value: this.value(shape.item, raw, at, path) };
      case "statements": {
        const items = Array.isArray(raw) ? raw : [raw];
        return { kind: "statements", value: items.map((item, index) => this.statement(shape, item, at, `${path}[${index}]`)) };
      }
      case "map":
        return { kind: "map", value: Object.entries(raw as JsonObject).map(([key, item]) => [key, this.value(shape.item, item, at, `${path}.${key}`)] as [string, FieldValue]) };
      case "value":
        return { kind: "value", value: raw as DslValue };
      default:
        throw new RangeError(`${path}: the ${shape.kind} shape has no document form`);
    }
  }

  private statement(shape: Extract<Shape, { kind: "statements" }>, raw: Json, at: string, path: string): [string, RecordFields] {
    const tagKey = this.spec.tags.get(at) ?? "kind";
    const tag = isObject(raw) ? raw[tagKey] : undefined;
    const found = shape.variants.find(([keyword]) => normal(keyword) === normal(String(tag)));
    if (found === undefined || !isObject(raw)) throw new RangeError(`${path}: ${JSON.stringify(tag)} is not one of ${JSON.stringify(shape.variants.map(([keyword]) => keyword))}`);
    const body = Object.fromEntries(Object.entries(raw).filter(([name]) => name !== tagKey));
    return [found[0], this.record(found[1], body, path, this.spec.verbatim.has(at))];
  }

  document(specIndex: number, fields: RecordFields, verbatim = false): JsonObject {
    const out: JsonObject = {};
    for (const field of this.spec.graph[specIndex]!) {
      const value = fields.get(field.id);
      const name = verbatim ? field.key.replaceAll("-", "_") : camel(field.key);
      const json = value === undefined ? (omittable(field.shape) && !field.optional ? [] : null) : this.json(field.shape, value, `${specIndex}.${field.id}`);
      if (json !== null || this.options.omitNulls !== true) out[name] = json;
    }
    const nested = this.spec.nested.get(specIndex);
    if (nested !== undefined) out[nested[0]] = Object.fromEntries(nested[1].map((member) => [camel(member), out[camel(member)]]));
    if (nested !== undefined) for (const member of nested[1]) delete out[camel(member)];
    return out;
  }

  json(shape: Shape, field: FieldValue, at: string): Json {
    switch (field.kind) {
      case "bool":
      case "text":
        return field.value;
      case "float":
        return this.options.narrowF32 === true && this.spec.float32.has(at) ? shortestF32(field.value) : field.value;
      case "int":
      case "uint":
        return Number(field.value);
      case "bytes":
        return [...field.value];
      case "enum": {
        const found = variantsOf(shape).find(([ordinal]) => ordinal === field.value);
        if (found === undefined) throw new RangeError(`the enum ordinal ${field.value} is not declared`);
        return camel(found[1]);
      }
      case "tuple":
      case "list":
        if (shape.kind === "table") return field.value.map((row) => this.document(shape.record, row.kind === "record" ? row.value : new Map()));
        return field.value.map((item) => this.json(shape.kind === "tuple" || shape.kind === "list" ? shape.item : { kind: "float" }, item, at));
      case "record":
        return this.document(recordOf(shape), field.value);
      case "block":
        return this.json(itemOf(shape), field.value, at);
      case "statements": {
        const variants = new Map(keywordsOf(shape));
        const tagKey = this.spec.tags.get(at) ?? "kind";
        const items = field.value.map(([keyword, fields]) => ({ [tagKey]: camel(keyword), ...this.document(variants.get(keyword)!, fields, this.spec.verbatim.has(at)) }));
        return this.spec.single.has(at) ? (items[0] ?? null) : items;
      }
      case "map":
        return Object.fromEntries(field.value.map(([key, item]) => [key, this.json(itemOf(shape), item, at)]));
      case "value":
        return field.value;
    }
  }
}

const shapeError = (shape: Shape, wanted: string): RangeError => new RangeError(`expected a ${wanted} shape, found ${shape.kind}`);
const variantsOf = (shape: Shape): [number, string][] => {
  if (shape.kind !== "enum") throw shapeError(shape, "enum");
  return shape.variants;
};
const keywordsOf = (shape: Shape): [string, number][] => {
  if (shape.kind !== "statements") throw shapeError(shape, "statements");
  return shape.variants;
};
const recordOf = (shape: Shape): number => {
  if (shape.kind !== "record" && shape.kind !== "table") throw shapeError(shape, "record");
  return shape.record;
};
const itemOf = (shape: Shape): Shape => {
  if (shape.kind !== "block" && shape.kind !== "map" && shape.kind !== "list" && shape.kind !== "tuple") throw shapeError(shape, "container");
  return shape.item;
};

/** 🚚️ A JSON snapshot to the `.pack.semio` bytes: the document mapped to its record tree, the record encoded, the semio envelope around it. */
export function encodePack(spec: RecordSpec, document: Json): Uint8Array {
  return wrapBinary(spec.token, encodeRecord(spec, new Document(spec).record(0, document)));
}

/** 📨️ The reading direction of `encodePack`, refusing any envelope token other than the spec's; `options` settle how the document spells an absent optional and an `f32`. */
export function decodePack(spec: RecordSpec, data: Uint8Array, options: DocumentOptions = {}): JsonObject {
  const { token, inner } = unwrapBinary(data);
  if (token !== spec.token) throw new RangeError(`expected the ${JSON.stringify(spec.token)} envelope token, got ${JSON.stringify(token)}`);
  return new Document(spec, options).document(0, decodeRecord(spec, inner));
}
// #endregion 🔖️Document
