/** 🗜️ Owned zlib (RFC 1950) deflate and inflate: LZ77 with hash chains, dynamic or fixed Huffman blocks, whichever is smaller; deterministic across hosts and runtimes.
 *
 * FBX arrays are zlib streams; owning the compressor keeps the bytes of an export identical on every machine instead of depending on a platform zlib build.
 *
 * @see https://www.rfc-editor.org/rfc/rfc1951
 * @see https://www.rfc-editor.org/rfc/rfc1950
 */
import type { Operation } from "../../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../../⏱️operation/🟦️.ts";

/** 🔌️ The interface FBX encoding compresses arrays through; any zlib-stream producer may stand behind it. The optional `operation` is the encoder's own; a producer that ignores it is merely not cancellable inside one array. */
export interface Compressor {
  zlib(bytes: Uint8Array, operation?: Operation): Uint8Array;
}

const LENGTH_BASE = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const CODE_LENGTH_ORDER = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
const WINDOW = 32768;
const MAX_MATCH = 258;
const MAX_CHAIN = 48;
const NICE_LENGTH = 128;
const HASH_BITS = 15;

const LENGTH_SYMBOL = (() => {
  const table = new Uint8Array(MAX_MATCH + 1);
  for (let symbol = 0; symbol < 29; symbol++) {
    const end = symbol === 28 ? 258 : LENGTH_BASE[symbol + 1] - 1;
    for (let length = LENGTH_BASE[symbol]; length <= end; length++) table[length] = symbol;
  }
  table[258] = 28;
  return table;
})();

function distanceSymbol(distance: number): number {
  if (distance <= 4) return distance - 1;
  const log = 31 - Math.clz32(distance - 1);
  return 2 * log + (((distance - 1) >> (log - 1)) & 1);
}

/** 🔢️ The Adler-32 checksum that closes a zlib stream; the optional `operation` is polled every few thousand bytes. */
export function adler32(bytes: Uint8Array, operation: Operation = silentOperation): number {
  const pace = new Pace(operation);
  let a = 1;
  let b = 0;
  for (let start = 0; start < bytes.length; ) {
    pace.tick(5552);
    const end = Math.min(bytes.length, start + 5552);
    for (let index = start; index < end; index++) {
      a += bytes[index];
      b += a;
    }
    a %= 65521;
    b %= 65521;
    start = end;
  }
  return ((b << 16) | a) >>> 0;
}

class BitWriter {
  private bytes = new Uint8Array(1024);
  private length = 0;
  private bits = 0;
  private count = 0;

  private push(byte: number): void {
    if (this.length === this.bytes.length) {
      const grown = new Uint8Array(this.bytes.length * 2);
      grown.set(this.bytes);
      this.bytes = grown;
    }
    this.bytes[this.length++] = byte;
  }

  write(value: number, width: number): void {
    this.bits |= value << this.count;
    this.count += width;
    while (this.count >= 8) {
      this.push(this.bits & 255);
      this.bits >>>= 8;
      this.count -= 8;
    }
  }

  writeCode(code: number, width: number): void {
    let reversed = 0;
    for (let bit = 0; bit < width; bit++) reversed |= ((code >> bit) & 1) << (width - 1 - bit);
    this.write(reversed, width);
  }

  finish(): Uint8Array {
    if (this.count > 0) this.push(this.bits & 255);
    return this.bytes.subarray(0, this.length);
  }
}

function codeLengths(frequencies: readonly number[], limit: number): number[] {
  const symbols = frequencies.length;
  const working = frequencies.slice();
  let used = working.filter((value) => value > 0).length;
  for (let symbol = 0; used < 2 && symbol < symbols; symbol++)
    if (working[symbol] === 0) {
      working[symbol] = 1;
      used++;
    }
  for (;;) {
    type Tree = { weight: number; order: number; symbol: number; left?: Tree; right?: Tree };
    let queue: Tree[] = [];
    working.forEach((weight, symbol) => {
      if (weight > 0) queue.push({ weight, order: symbol, symbol });
    });
    let order = symbols;
    while (queue.length > 1) {
      queue.sort((a, b) => a.weight - b.weight || a.order - b.order);
      const [left, right] = queue;
      queue = [...queue.slice(2), { weight: left.weight + right.weight, order: order++, symbol: -1, left, right }];
    }
    const lengths = new Array<number>(symbols).fill(0);
    let deepest = 0;
    const walk = (tree: Tree, depth: number): void => {
      if (tree.left && tree.right) {
        walk(tree.left, depth + 1);
        walk(tree.right, depth + 1);
      } else {
        lengths[tree.symbol] = depth;
        deepest = Math.max(deepest, depth);
      }
    };
    walk(queue[0], 0);
    if (deepest <= limit) return lengths;
    for (let symbol = 0; symbol < symbols; symbol++) if (working[symbol] > 0) working[symbol] = (working[symbol] + 1) >> 1;
  }
}

function canonicalCodes(lengths: readonly number[]): number[] {
  const counts = new Array<number>(16).fill(0);
  for (const length of lengths) counts[length]++;
  counts[0] = 0;
  const next = new Array<number>(16).fill(0);
  let code = 0;
  for (let length = 1; length < 16; length++) {
    code = (code + counts[length - 1]) << 1;
    next[length] = code;
  }
  return lengths.map((length) => (length === 0 ? 0 : next[length]++));
}

const FIXED_LITERAL_LENGTHS = Array.from({ length: 288 }, (_, symbol) => (symbol < 144 ? 8 : symbol < 256 ? 9 : symbol < 280 ? 7 : 8));
const FIXED_DISTANCE_LENGTHS = new Array<number>(30).fill(5);

type Tokens = { readonly kinds: Uint16Array; readonly values: Uint16Array; count: number };

function tokenize(bytes: Uint8Array, pace: Pace): Tokens {
  const size = bytes.length;
  const tokens: Tokens = { kinds: new Uint16Array(size + 1), values: new Uint16Array(size + 1), count: 0 };
  const head = new Int32Array(1 << HASH_BITS).fill(-1);
  const previous = new Int32Array(size).fill(-1);
  const hashAt = (position: number): number => Math.imul((bytes[position] << 16) | (bytes[position + 1] << 8) | bytes[position + 2], 2654435761) >>> (32 - HASH_BITS);
  const insert = (position: number): void => {
    if (position + 2 >= size) return;
    const hash = hashAt(position);
    previous[position] = head[hash];
    head[hash] = position;
  };
  let position = 0;
  while (position < size) {
    pace.tick(8);
    let bestLength = 0;
    let bestDistance = 0;
    if (position + 2 < size) {
      let candidate = head[hashAt(position)];
      const limit = Math.min(MAX_MATCH, size - position);
      for (let chain = 0; candidate >= 0 && position - candidate <= WINDOW && chain < MAX_CHAIN; chain++, candidate = previous[candidate]) {
        if (bytes[candidate + bestLength] !== bytes[position + bestLength] && bestLength > 0) continue;
        let length = 0;
        while (length < limit && bytes[candidate + length] === bytes[position + length]) length++;
        if (length > bestLength) {
          bestLength = length;
          bestDistance = position - candidate;
          if (length >= NICE_LENGTH || length === limit) break;
        }
      }
    }
    if (bestLength >= 3) {
      tokens.kinds[tokens.count] = bestLength;
      tokens.values[tokens.count++] = bestDistance;
      pace.tick(bestLength);
      for (let step = 0; step < bestLength; step++) insert(position + step);
      position += bestLength;
    } else {
      tokens.kinds[tokens.count] = 0;
      tokens.values[tokens.count++] = bytes[position];
      insert(position);
      position++;
    }
  }
  return tokens;
}

function tokenCost(tokens: Tokens, literalLengths: readonly number[], distanceLengths: readonly number[], pace: Pace): number {
  let bits = literalLengths[256];
  for (let index = 0; index < tokens.count; index++) {
    pace.tick();
    const length = tokens.kinds[index];
    if (length === 0) bits += literalLengths[tokens.values[index]];
    else {
      const lengthSymbol = LENGTH_SYMBOL[length];
      const distanceSym = distanceSymbol(tokens.values[index]);
      bits += literalLengths[257 + lengthSymbol] + LENGTH_EXTRA[lengthSymbol] + distanceLengths[distanceSym] + DIST_EXTRA[distanceSym];
    }
  }
  return bits;
}

function runLengthEncode(lengths: readonly number[]): { symbol: number; extra: number; bits: number }[] {
  const out: { symbol: number; extra: number; bits: number }[] = [];
  for (let index = 0; index < lengths.length; ) {
    const value = lengths[index];
    let run = 1;
    while (index + run < lengths.length && lengths[index + run] === value) run++;
    if (value === 0 && run >= 3) {
      const take = Math.min(run, 138);
      out.push(take >= 11 ? { symbol: 18, extra: take - 11, bits: 7 } : { symbol: 17, extra: take - 3, bits: 3 });
      index += take;
    } else if (value !== 0 && run >= 4) {
      out.push({ symbol: value, extra: 0, bits: 0 });
      const take = Math.min(run - 1, 6);
      out.push({ symbol: 16, extra: take - 3, bits: 2 });
      index += 1 + take;
    } else {
      out.push({ symbol: value, extra: 0, bits: 0 });
      index++;
    }
  }
  return out;
}

function emitTokens(writer: BitWriter, tokens: Tokens, literalLengths: readonly number[], distanceLengths: readonly number[], pace: Pace): void {
  const literalCodes = canonicalCodes(literalLengths);
  const distanceCodes = canonicalCodes(distanceLengths);
  for (let index = 0; index < tokens.count; index++) {
    pace.tick();
    const length = tokens.kinds[index];
    if (length === 0) writer.writeCode(literalCodes[tokens.values[index]], literalLengths[tokens.values[index]]);
    else {
      const lengthSymbol = LENGTH_SYMBOL[length];
      writer.writeCode(literalCodes[257 + lengthSymbol], literalLengths[257 + lengthSymbol]);
      writer.write(length - LENGTH_BASE[lengthSymbol], LENGTH_EXTRA[lengthSymbol]);
      const distance = tokens.values[index];
      const distanceSym = distanceSymbol(distance);
      writer.writeCode(distanceCodes[distanceSym], distanceLengths[distanceSym]);
      writer.write(distance - DIST_BASE[distanceSym], DIST_EXTRA[distanceSym]);
    }
  }
  writer.writeCode(literalCodes[256], literalLengths[256]);
}

/** 📉️ Compresses `bytes` into a raw deflate stream (one block); the optional `operation` is polled every few thousand bytes or tokens and a cancellation throws `CancelledError`. */
export function deflateRaw(bytes: Uint8Array, operation: Operation = silentOperation): Uint8Array {
  const pace = new Pace(operation);
  const tokens = tokenize(bytes, pace);
  const literalFrequencies = new Array<number>(286).fill(0);
  const distanceFrequencies = new Array<number>(30).fill(0);
  literalFrequencies[256] = 1;
  for (let index = 0; index < tokens.count; index++) {
    pace.tick();
    const length = tokens.kinds[index];
    if (length === 0) literalFrequencies[tokens.values[index]]++;
    else {
      literalFrequencies[257 + LENGTH_SYMBOL[length]]++;
      distanceFrequencies[distanceSymbol(tokens.values[index])]++;
    }
  }
  const literalLengths = codeLengths(literalFrequencies, 15);
  const distanceLengths = codeLengths(distanceFrequencies, 15);
  let literalCount = 286;
  while (literalCount > 257 && literalLengths[literalCount - 1] === 0) literalCount--;
  let distanceCount = 30;
  while (distanceCount > 1 && distanceLengths[distanceCount - 1] === 0) distanceCount--;
  const sequence = runLengthEncode([...literalLengths.slice(0, literalCount), ...distanceLengths.slice(0, distanceCount)]);
  const clFrequencies = new Array<number>(19).fill(0);
  for (const item of sequence) clFrequencies[item.symbol]++;
  const clLengths = codeLengths(clFrequencies, 7);
  let clCount = 19;
  while (clCount > 4 && clLengths[CODE_LENGTH_ORDER[clCount - 1]] === 0) clCount--;
  const headerBits = 14 + clCount * 3 + sequence.reduce((sum, item) => sum + clLengths[item.symbol] + item.bits, 0);
  const dynamicBits = headerBits + tokenCost(tokens, literalLengths, distanceLengths, pace);
  const fixedBits = tokenCost(tokens, FIXED_LITERAL_LENGTHS, FIXED_DISTANCE_LENGTHS, pace);
  const writer = new BitWriter();
  writer.write(1, 1);
  if (fixedBits <= dynamicBits) {
    writer.write(1, 2);
    emitTokens(writer, tokens, FIXED_LITERAL_LENGTHS, FIXED_DISTANCE_LENGTHS, pace);
  } else {
    writer.write(2, 2);
    writer.write(literalCount - 257, 5);
    writer.write(distanceCount - 1, 5);
    writer.write(clCount - 4, 4);
    for (let index = 0; index < clCount; index++) writer.write(clLengths[CODE_LENGTH_ORDER[index]], 3);
    const clCodes = canonicalCodes(clLengths);
    for (const item of sequence) {
      writer.writeCode(clCodes[item.symbol], clLengths[item.symbol]);
      if (item.bits > 0) writer.write(item.extra, item.bits);
    }
    emitTokens(writer, tokens, literalLengths, distanceLengths, pace);
  }
  return writer.finish();
}

/** 🧶️ Compresses `bytes` into a zlib stream (RFC 1950, default level header, Adler-32 trailer); cancellable through the optional `operation` like {@link deflateRaw}. */
export function deflateZlib(bytes: Uint8Array, operation: Operation = silentOperation): Uint8Array {
  const body = deflateRaw(bytes, operation);
  const out = new Uint8Array(body.length + 6);
  out[0] = 0x78;
  out[1] = 0x9c;
  out.set(body, 2);
  new DataView(out.buffer).setUint32(body.length + 2, adler32(bytes, operation), false);
  return out;
}

/** 🏠️ The owned compressor, the default behind every FBX encoding. */
export const ownedCompressor: Compressor = { zlib: deflateZlib };

type Huffman = { readonly counts: Uint16Array; readonly symbols: Uint16Array };

function huffman(lengths: readonly number[]): Huffman {
  const counts = new Uint16Array(16);
  for (const length of lengths) counts[length]++;
  const offsets = new Uint16Array(16);
  for (let length = 1; length < 15; length++) offsets[length + 1] = offsets[length] + counts[length];
  const symbols = new Uint16Array(lengths.length);
  lengths.forEach((length, symbol) => {
    if (length !== 0) symbols[offsets[length]++] = symbol;
  });
  return { counts, symbols };
}

/** 📤️ Inflates a raw deflate stream; throws on malformed input, and `CancelledError` when the optional `operation` is cancelled (polled every few thousand symbols). */
export function inflateRaw(input: Uint8Array, operation: Operation = silentOperation): Uint8Array {
  const pace = new Pace(operation);
  let out = new Uint8Array(Math.max(1024, input.length * 4));
  let outLength = 0;
  let position = 0;
  let bitBuffer = 0;
  let bitCount = 0;
  const bits = (need: number): number => {
    while (bitCount < need) {
      if (position >= input.length) throw new Error("inflate: unexpected end of data");
      bitBuffer |= input[position++] << bitCount;
      bitCount += 8;
    }
    const value = bitBuffer & ((1 << need) - 1);
    bitBuffer >>>= need;
    bitCount -= need;
    return value;
  };
  const emit = (byte: number): void => {
    if (outLength === out.length) {
      const grown = new Uint8Array(out.length * 2);
      grown.set(out);
      out = grown;
    }
    out[outLength++] = byte;
  };
  const decode = (table: Huffman): number => {
    let code = 0;
    let first = 0;
    let index = 0;
    for (let length = 1; length < 16; length++) {
      code |= bits(1);
      const count = table.counts[length];
      if (code - count < first) return table.symbols[index + (code - first)];
      index += count;
      first += count;
      first <<= 1;
      code <<= 1;
    }
    throw new Error("inflate: invalid code");
  };
  const fixedLiteral = huffman(FIXED_LITERAL_LENGTHS);
  const fixedDistance = huffman(FIXED_DISTANCE_LENGTHS);
  for (let final = 0; !final; ) {
    final = bits(1);
    const type = bits(2);
    if (type === 0) {
      bitBuffer = 0;
      bitCount = 0;
      const length = input[position] | (input[position + 1] << 8);
      position += 4;
      for (let index = 0; index < length; index++) {
        pace.tick();
        emit(input[position++]);
      }
      continue;
    }
    if (type === 3) throw new Error("inflate: invalid block type");
    let literal = fixedLiteral;
    let distance = fixedDistance;
    if (type === 2) {
      const literalCount = bits(5) + 257;
      const distanceCount = bits(5) + 1;
      const clCount = bits(4) + 4;
      const clLengths = new Array<number>(19).fill(0);
      for (let index = 0; index < clCount; index++) clLengths[CODE_LENGTH_ORDER[index]] = bits(3);
      const clTable = huffman(clLengths);
      const lengths: number[] = [];
      while (lengths.length < literalCount + distanceCount) {
        const symbol = decode(clTable);
        if (symbol < 16) lengths.push(symbol);
        else {
          const previous = symbol === 16 ? lengths[lengths.length - 1] : 0;
          const repeat = symbol === 16 ? 3 + bits(2) : symbol === 17 ? 3 + bits(3) : 11 + bits(7);
          for (let index = 0; index < repeat; index++) lengths.push(previous);
        }
      }
      literal = huffman(lengths.slice(0, literalCount));
      distance = huffman(lengths.slice(literalCount));
    }
    for (;;) {
      pace.tick();
      const symbol = decode(literal);
      if (symbol < 256) emit(symbol);
      else if (symbol === 256) break;
      else {
        const lengthIndex = symbol - 257;
        const length = LENGTH_BASE[lengthIndex] + bits(LENGTH_EXTRA[lengthIndex]);
        const distanceIndex = decode(distance);
        const back = DIST_BASE[distanceIndex] + bits(DIST_EXTRA[distanceIndex]);
        if (back > outLength) throw new Error("inflate: distance too far back");
        pace.tick(length);
        for (let index = 0; index < length; index++) emit(out[outLength - back]);
      }
    }
  }
  return out.slice(0, outLength);
}

/** 📥️ Inflates a zlib stream and verifies its Adler-32 trailer; cancellable through the optional `operation` like {@link inflateRaw}. */
export function inflateZlib(input: Uint8Array, operation: Operation = silentOperation): Uint8Array {
  if (input.length < 6 || (input[0] & 15) !== 8 || ((input[0] << 8) | input[1]) % 31 !== 0) throw new Error("inflate: invalid zlib header");
  const out = inflateRaw(input.subarray(2, input.length - 4), operation);
  if (new DataView(input.buffer, input.byteOffset, input.byteLength).getUint32(input.length - 4, false) !== adler32(out, operation)) throw new Error("inflate: checksum mismatch");
  return out;
}
