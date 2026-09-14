//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The coordinator event envelope as this package encodes it: one frozen field order, a payload
// re-encoded with sorted object keys, and a checksum over the NUL-separated preimage. The digest is
// this package's own SHA-256 — FIPS 180-4 written out here rather than borrowed from the platform —
// so that comparing this implementation against `node:crypto` compares two independent readings of
// the same standard instead of one library against itself.

//#endregion 🧲️Header

//#region 🔖️Envelope

/** ✉️ One committed coordinator event, with the seven fields in the order the log freezes. */
export type EventEnvelope = {
  readonly stream: string;
  readonly sequence: number;
  readonly id: string;
  readonly generation: number;
  readonly type: string;
  readonly payload: unknown;
  readonly checksum: string;
};

//#endregion 🔖️Envelope

//#region 🔏️Digest

const ROUND_CONSTANTS = [
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
] as const;

const rotate = (value: number, bits: number): number => ((value >>> bits) | (value << (32 - bits))) >>> 0;

/** 🔏️ SHA-256 over the given bytes, as the lowercase hexadecimal digest. */
export function sha256Hex(bytes: Uint8Array): string {
  const state = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
  const bitLength = BigInt(bytes.length) * 8n;
  const padded = new Uint8Array(((bytes.length + 9 + 63) >> 6) << 6);
  padded.set(bytes);
  padded[bytes.length] = 0x80;
  for (let offset = 0; offset < 8; offset += 1) padded[padded.length - 1 - offset] = Number((bitLength >> BigInt(8 * offset)) & 0xffn);
  const schedule = new Uint32Array(64);
  for (let block = 0; block < padded.length; block += 64) {
    for (let index = 0; index < 16; index += 1) schedule[index] = ((padded[block + index * 4] << 24) | (padded[block + index * 4 + 1] << 16) | (padded[block + index * 4 + 2] << 8) | padded[block + index * 4 + 3]) >>> 0;
    for (let index = 16; index < 64; index += 1) {
      const lower = rotate(schedule[index - 15], 7) ^ rotate(schedule[index - 15], 18) ^ (schedule[index - 15] >>> 3);
      const upper = rotate(schedule[index - 2], 17) ^ rotate(schedule[index - 2], 19) ^ (schedule[index - 2] >>> 10);
      schedule[index] = (schedule[index - 16] + lower + schedule[index - 7] + upper) >>> 0;
    }
    let [a, b, c, d, e, f, g, h] = state;
    for (let index = 0; index < 64; index += 1) {
      const sigmaOne = rotate(e, 6) ^ rotate(e, 11) ^ rotate(e, 25);
      const choose = (e & f) ^ (~e & g);
      const first = (h + sigmaOne + choose + ROUND_CONSTANTS[index] + schedule[index]) >>> 0;
      const sigmaZero = rotate(a, 2) ^ rotate(a, 13) ^ rotate(a, 22);
      const majority = (a & b) ^ (a & c) ^ (b & c);
      const second = (sigmaZero + majority) >>> 0;
      h = g;
      g = f;
      f = e;
      e = (d + first) >>> 0;
      d = c;
      c = b;
      b = a;
      a = (first + second) >>> 0;
    }
    const round = [a, b, c, d, e, f, g, h];
    for (let index = 0; index < 8; index += 1) state[index] = (state[index] + round[index]) >>> 0;
  }
  return state.map((word) => word.toString(16).padStart(8, "0")).join("");
}

//#endregion 🔏️Digest

//#region ♻️Canonical

/** ♻️ The value as the log persists it: object keys sorted, no insignificant whitespace. */
export function canonicalJson(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value) ?? "null";
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  const entries = Object.entries(value as Record<string, unknown>).sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0));
  return `{${entries.map(([key, item]) => `${JSON.stringify(key)}:${canonicalJson(item)}`).join(",")}}`;
}

/** 🧮️ The preimage the checksum covers: five header fields separated by NUL, then the canonical payload. */
export function checksumPreimage(event: EventEnvelope): Uint8Array {
  return new TextEncoder().encode(`${event.stream}\u0000${event.sequence}\u0000${event.id}\u0000${event.generation}\u0000${event.type}\u0000${canonicalJson(event.payload)}`);
}

/** 🔏️ The envelope's checksum, recomputed from its own fields. */
export function eventChecksum(event: EventEnvelope): string {
  return sha256Hex(checksumPreimage(event));
}

/** 📄️ The single canonical log line for one envelope, checksum included. */
export function canonicalEventLine(event: EventEnvelope): string {
  return `{"stream":${JSON.stringify(event.stream)},"sequence":${event.sequence},"id":${JSON.stringify(event.id)},"generation":${event.generation},"type":${JSON.stringify(event.type)},"payload":${canonicalJson(event.payload)},"checksum":${JSON.stringify(eventChecksum(event))}}`;
}

//#endregion ♻️Canonical
