//! 🧮️ Replication binary primitives use the shared native wire layout.

/** 🎞️ Writes an unsigned LEB128 varint (minimal length) — a byte-for-byte TS twin of
 * `protocol_core`'s `write_varint_u64` (`protocol/core/rs/lib.rs` `🔖️WireCodec`). */
export function writeVarintU64(out: number[], value: number): void {
  let remaining = value;
  for (;;) {
    const byte = remaining & 0x7f;
    remaining = Math.floor(remaining / 128);
    if (remaining === 0) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 🎞️ Reads an unsigned LEB128 varint starting at `pos[0]`, advancing it past it — the TS twin of
 * `protocol_core`'s `read_varint_u64`. */
export function readVarintU64(bytes: Uint8Array, pos: [number]): number {
  let result = 0;
  let shift = 1;
  for (let i = 0; i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result += (byte & 0x7f) * shift;
    if ((byte & 0x80) === 0) return result;
    shift *= 128;
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}

/** 🎞️ Exact `u64` twin of {@link writeVarintU64} for values that may exceed 2^53 (hash-derived revisions). */
export function writeVarintU64Exact(out: number[], value: bigint): void {
  if (value < 0n || value > 0xffff_ffff_ffff_ffffn) throw new Error("wire frame varint: u64 range");
  let remaining = value;
  for (;;) {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    if (remaining === 0n) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 🎞️ Exact `u64` twin of {@link readVarintU64}. */
export function readVarintU64Exact(bytes: Uint8Array, pos: [number]): bigint {
  let result = 0n;
  for (let i = 0; i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result |= BigInt(byte & 0x7f) << BigInt(7 * i);
    if ((byte & 0x80) === 0) {
      if (result > 0xffff_ffff_ffff_ffffn) throw new Error("wire frame varint: u64 range");
      return result;
    }
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}

/** 🎞️ `varint-u64 len | utf8 bytes` — the TS twin of `protocol_core::write_str`. */
export function writeStr(out: number[], value: string): void {
  const bytes = new TextEncoder().encode(value);
  writeVarintU64(out, bytes.length);
  for (const byte of bytes) out.push(byte);
}

/** 🎞️ The inverse of {@link writeStr} — the TS twin of `protocol_core::read_str`. */
export function readStr(bytes: Uint8Array, pos: [number]): string {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire str: truncated");
  pos[0] += len;
  return new TextDecoder().decode(slice);
}

/** 🎞️ `varint-u64 len | raw bytes` — the TS twin of `protocol_core::write_bytes`. */
export function writeBytes(out: number[], value: readonly number[] | Uint8Array): void {
  writeVarintU64(out, value.length);
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeBytes} — the TS twin of `protocol_core::read_bytes`. */
export function readBytes(bytes: Uint8Array, pos: [number]): number[] {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire bytes: truncated");
  pos[0] += len;
  return Array.from(slice);
}

/** 🎞️ 32 raw bytes, no length prefix — the TS twin of `protocol_core::write_hash32`. */
export function writeHash32(out: number[], value: readonly number[]): void {
  if (value.length !== 32) throw new Error("wire hash32: expected 32 bytes");
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeHash32} — the TS twin of `protocol_core::read_hash32`. */
export function readHash32(bytes: Uint8Array, pos: [number]): number[] {
  const slice = bytes.subarray(pos[0], pos[0] + 32);
  if (slice.length !== 32) throw new Error("wire hash32: truncated");
  pos[0] += 32;
  return Array.from(slice);
}

/** 🎞️ One byte, `0`/`1` — the TS twin of `protocol_core::write_bool`. */
export function writeBool(out: number[], value: boolean): void {
  out.push(value ? 1 : 0);
}

/** 🎞️ The inverse of {@link writeBool} — the TS twin of `protocol_core::read_bool`. */
export function readBool(bytes: Uint8Array, pos: [number]): boolean {
  const byte = bytes[pos[0]];
  if (byte === undefined) throw new Error("wire bool: truncated");
  pos[0] += 1;
  return byte !== 0;
}

/** 🎞️ 8 raw little-endian bytes — the TS twin of `protocol_core::write_f64`. */
export function writeF64(out: number[], value: number): void {
  const buffer = new ArrayBuffer(8);
  new DataView(buffer).setFloat64(0, value, true);
  for (const byte of new Uint8Array(buffer)) out.push(byte);
}

/** 🎞️ The inverse of {@link writeF64} — the TS twin of `protocol_core::read_f64`. */
export function readF64(bytes: Uint8Array, pos: [number]): number {
  const slice = bytes.subarray(pos[0], pos[0] + 8);
  if (slice.length !== 8) throw new Error("wire f64: truncated");
  pos[0] += 8;
  return new DataView(slice.buffer, slice.byteOffset, 8).getFloat64(0, true);
}

/** 🎞️ `varint-u64 len | raw bytes` per entry — the TS twin of `protocol_wire::write_vec_bytes`. */
export function writeVecBytes(out: number[], values: readonly (readonly number[])[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeBytes(out, value);
}

/** 🎞️ The inverse of {@link writeVecBytes} — the TS twin of `protocol_wire::read_vec_bytes`. */
export function readVecBytes(bytes: Uint8Array, pos: [number]): number[][] {
  const count = readVarintU64(bytes, pos);
  const result: number[][] = [];
  for (let i = 0; i < count; i++) result.push(readBytes(bytes, pos));
  return result;
}

