//#region 🔌️Adapters
import { deflateRawSync, inflateRawSync } from "node:zlib";
//#endregion 🔌️Adapters

//#region 🔖️OwnedZipContract
export type OwnedZipFiles = ReadonlyMap<string, Uint8Array>;

const MAX_ZIP_BYTES = 256 * 1024 * 1024;
const MAX_ENTRY_COUNT = 4096;
const MAX_ENTRY_NAME_BYTES = 4096;
const MAX_ENTRY_BYTES = 256 * 1024 * 1024;
const MAX_TOTAL_BYTES = 512 * 1024 * 1024;
const LOCAL_HEADER = 0x04034b50;
const CENTRAL_HEADER = 0x02014b50;
const END_HEADER = 0x06054b50;
const UTF8_FLAG = 1 << 11;
const DEFLATE_METHOD = 8;
const DOS_DATE_1980_01_01 = 1 << 21;
//#endregion 🔖️OwnedZipContract

//#region 🧮️Primitives
const crcTable = Uint32Array.from({ length: 256 }, (_, value) => {
  let crc = value;
  for (let bit = 0; bit < 8; bit++) crc = crc & 1 ? 0xedb88320 ^ (crc >>> 1) : crc >>> 1;
  return crc >>> 0;
});

function crc32(bytes: Uint8Array): number {
  let crc = 0xffffffff;
  for (const byte of bytes) crc = crcTable[(crc ^ byte) & 0xff]! ^ (crc >>> 8);
  return (crc ^ 0xffffffff) >>> 0;
}

function view(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function assertRange(bytes: Uint8Array, offset: number, length: number, context: string): void {
  if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(length) || offset < 0 || length < 0 || offset + length > bytes.length) throw new Error(`invalid extension zip ${context}`);
}

function u16(bytes: Uint8Array, offset: number): number {
  assertRange(bytes, offset, 2, "u16 range");
  return view(bytes).getUint16(offset, true);
}

function u32(bytes: Uint8Array, offset: number): number {
  assertRange(bytes, offset, 4, "u32 range");
  return view(bytes).getUint32(offset, true);
}

function set16(bytes: Uint8Array, offset: number, value: number): void {
  view(bytes).setUint16(offset, value, true);
}

function set32(bytes: Uint8Array, offset: number, value: number): void {
  view(bytes).setUint32(offset, value >>> 0, true);
}

function decodeName(bytes: Uint8Array): string {
  if (bytes.length === 0 || bytes.length > MAX_ENTRY_NAME_BYTES) throw new Error("invalid extension zip entry name length");
  let name: string;
  try {
    name = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    throw new Error("invalid extension zip UTF-8 entry name");
  }
  validateName(name);
  return name;
}

function validateName(name: string): void {
  const segments = name.split("/");
  if (!name || name.startsWith("/") || name.includes("\\") || name.includes("\0") || segments.some((segment) => !segment || segment === "." || segment === "..")) {
    throw new Error(`invalid extension zip entry name ${JSON.stringify(name)}`);
  }
}

function findEndHeader(bytes: Uint8Array): number {
  const minimum = Math.max(0, bytes.length - 22 - 0xffff);
  for (let offset = bytes.length - 22; offset >= minimum; offset--) {
    if (u32(bytes, offset) === END_HEADER && offset + 22 + u16(bytes, offset + 20) === bytes.length) return offset;
  }
  throw new Error("invalid extension zip end record");
}
//#endregion 🧮️Primitives

//#region 📦️Codec
/** @emoji 📦️ Encodes deterministic UTF-8 ZIP/DEFLATE bytes for extension packages. */
export function encodeOwnedZip(files: OwnedZipFiles): Uint8Array {
  if (files.size === 0 || files.size > MAX_ENTRY_COUNT) throw new Error("invalid extension zip entry count");
  const rows: {
    readonly name: Uint8Array;
    readonly payload: Uint8Array;
    readonly compressed: Uint8Array;
    readonly crc: number;
    readonly localOffset: number;
  }[] = [];
  let localLength = 0;
  let totalBytes = 0;
  for (const [name, payload] of files) {
    validateName(name);
    const nameBytes = new TextEncoder().encode(name);
    if (nameBytes.length > MAX_ENTRY_NAME_BYTES) throw new Error("invalid extension zip entry name length");
    if (payload.length > MAX_ENTRY_BYTES || totalBytes + payload.length > MAX_TOTAL_BYTES) throw new Error("extension zip decoded size limit exceeded");
    const compressed = new Uint8Array(deflateRawSync(payload, { level: 6 }));
    const row = { name: nameBytes, payload, compressed, crc: crc32(payload), localOffset: localLength };
    rows.push(row);
    localLength += 30 + nameBytes.length + compressed.length;
    totalBytes += payload.length;
  }
  const centralLength = rows.reduce((sum, row) => sum + 46 + row.name.length, 0);
  const outputLength = localLength + centralLength + 22;
  if (outputLength > MAX_ZIP_BYTES) throw new Error("extension zip encoded size limit exceeded");
  const output = new Uint8Array(outputLength);
  let localOffset = 0;
  for (const row of rows) {
    set32(output, localOffset, LOCAL_HEADER);
    set16(output, localOffset + 4, 20);
    set16(output, localOffset + 6, UTF8_FLAG);
    set16(output, localOffset + 8, DEFLATE_METHOD);
    set32(output, localOffset + 10, DOS_DATE_1980_01_01);
    set32(output, localOffset + 14, row.crc);
    set32(output, localOffset + 18, row.compressed.length);
    set32(output, localOffset + 22, row.payload.length);
    set16(output, localOffset + 26, row.name.length);
    output.set(row.name, localOffset + 30);
    output.set(row.compressed, localOffset + 30 + row.name.length);
    localOffset += 30 + row.name.length + row.compressed.length;
  }
  let centralOffset = localLength;
  for (const row of rows) {
    set32(output, centralOffset, CENTRAL_HEADER);
    set16(output, centralOffset + 4, 20);
    set16(output, centralOffset + 6, 20);
    set16(output, centralOffset + 8, UTF8_FLAG);
    set16(output, centralOffset + 10, DEFLATE_METHOD);
    set32(output, centralOffset + 12, DOS_DATE_1980_01_01);
    set32(output, centralOffset + 16, row.crc);
    set32(output, centralOffset + 20, row.compressed.length);
    set32(output, centralOffset + 24, row.payload.length);
    set16(output, centralOffset + 28, row.name.length);
    set32(output, centralOffset + 42, row.localOffset);
    output.set(row.name, centralOffset + 46);
    centralOffset += 46 + row.name.length;
  }
  set32(output, centralOffset, END_HEADER);
  set16(output, centralOffset + 8, rows.length);
  set16(output, centralOffset + 10, rows.length);
  set32(output, centralOffset + 12, centralLength);
  set32(output, centralOffset + 16, localLength);
  return output;
}

/** @emoji 🔓️ Decodes bounded UTF-8 ZIP entries using stored or raw-DEFLATE payloads. */
export function decodeOwnedZip(bytes: Uint8Array): Map<string, Uint8Array> {
  if (bytes.length < 22 || bytes.length > MAX_ZIP_BYTES) throw new Error("invalid extension zip encoded size");
  const endOffset = findEndHeader(bytes);
  if (u16(bytes, endOffset + 4) !== 0 || u16(bytes, endOffset + 6) !== 0) throw new Error("multi-disk extension zip is unsupported");
  const entryCount = u16(bytes, endOffset + 10);
  if (entryCount === 0 || entryCount !== u16(bytes, endOffset + 8) || entryCount > MAX_ENTRY_COUNT) throw new Error("invalid extension zip entry count");
  const centralLength = u32(bytes, endOffset + 12);
  const centralStart = u32(bytes, endOffset + 16);
  assertRange(bytes, centralStart, centralLength, "central directory range");
  if (centralStart + centralLength !== endOffset) throw new Error("invalid extension zip central directory boundary");
  const files = new Map<string, Uint8Array>();
  let centralOffset = centralStart;
  let totalBytes = 0;
  for (let index = 0; index < entryCount; index++) {
    if (u32(bytes, centralOffset) !== CENTRAL_HEADER) throw new Error("invalid extension zip central header");
    const flags = u16(bytes, centralOffset + 8);
    const method = u16(bytes, centralOffset + 10);
    if (flags & 1) throw new Error("encrypted extension zip entries are unsupported");
    if (method !== 0 && method !== DEFLATE_METHOD) throw new Error(`unsupported extension zip compression method ${method}`);
    const expectedCrc = u32(bytes, centralOffset + 16);
    const compressedLength = u32(bytes, centralOffset + 20);
    const payloadLength = u32(bytes, centralOffset + 24);
    const nameLength = u16(bytes, centralOffset + 28);
    const extraLength = u16(bytes, centralOffset + 30);
    const commentLength = u16(bytes, centralOffset + 32);
    const disk = u16(bytes, centralOffset + 34);
    const localOffset = u32(bytes, centralOffset + 42);
    if (disk !== 0 || compressedLength === 0xffffffff || payloadLength === 0xffffffff || localOffset === 0xffffffff) throw new Error("ZIP64 extension packages are unsupported");
    assertRange(bytes, centralOffset + 46, nameLength + extraLength + commentLength, "central entry range");
    const name = decodeName(bytes.subarray(centralOffset + 46, centralOffset + 46 + nameLength));
    if (files.has(name)) throw new Error(`duplicate extension zip entry ${name}`);
    if (payloadLength > MAX_ENTRY_BYTES || totalBytes + payloadLength > MAX_TOTAL_BYTES) throw new Error("extension zip decoded size limit exceeded");
    if (u32(bytes, localOffset) !== LOCAL_HEADER) throw new Error("invalid extension zip local header");
    const localFlags = u16(bytes, localOffset + 6);
    const localMethod = u16(bytes, localOffset + 8);
    if (localFlags & 1 || localMethod !== method) throw new Error("extension zip local header mismatch");
    const localNameLength = u16(bytes, localOffset + 26);
    const localExtraLength = u16(bytes, localOffset + 28);
    assertRange(bytes, localOffset + 30, localNameLength + localExtraLength, "local entry range");
    const localName = decodeName(bytes.subarray(localOffset + 30, localOffset + 30 + localNameLength));
    if (localName !== name) throw new Error("extension zip local header mismatch");
    const payloadOffset = localOffset + 30 + localNameLength + localExtraLength;
    assertRange(bytes, payloadOffset, compressedLength, "compressed payload range");
    if (payloadOffset + compressedLength > centralStart) throw new Error("invalid extension zip compressed payload boundary");
    const compressed = bytes.subarray(payloadOffset, payloadOffset + compressedLength);
    let payload: Uint8Array;
    try {
      payload = method === 0 ? new Uint8Array(compressed) : new Uint8Array(inflateRawSync(compressed, { maxOutputLength: Math.min(payloadLength + 1, MAX_ENTRY_BYTES + 1) }));
    } catch {
      throw new Error(`invalid extension zip compressed payload ${name}`);
    }
    if (payload.length !== payloadLength || crc32(payload) !== expectedCrc) throw new Error(`invalid extension zip checksum ${name}`);
    files.set(name, payload);
    totalBytes += payload.length;
    centralOffset += 46 + nameLength + extraLength + commentLength;
  }
  if (centralOffset !== endOffset) throw new Error("invalid extension zip central entry count");
  return files;
}
//#endregion 📦️Codec
