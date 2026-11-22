/** 📜️ The folder event log `.semio/events.semio` (`SEMIOEL1`), the one canonical on-disk store of folder documents shared with the native actor (`FolderEventLogStorage`). Records are `magic | length u64 | checksum u64 | payload`; an archive put carries kind 4, the document id as key and the schema as metadata. */
import { closeSync, fsyncSync, ftruncateSync, fstatSync, mkdirSync, openSync, readSync, writeSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

const MAGIC = new TextEncoder().encode("SEMIOEL1");
const ARCHIVE_PUT_EVENT = 4;
const CHUNK = 64 * 1024;
const HEADER = 24;

/** 📍️ The canonical event log path inside a document folder. */
export function eventLogPath(folder: string): string {
  return join(folder, ".semio", "events.semio");
}

/** 🔢️ Streaming FNV-1a 64 over two 32-bit halves, so a record of any size is hashed in bounded chunks without per-byte `BigInt`. */
class Fnv64 {
  private hi = 0xcbf29ce4;
  private lo = 0x84222325;
  update(bytes: Uint8Array): this {
    let { hi, lo } = this;
    for (let index = 0; index < bytes.length; index += 1) {
      lo = (lo ^ bytes[index]!) >>> 0;
      const product = lo * 0x1b3;
      hi = (Math.imul(hi, 0x1b3) + Math.floor(product / 4294967296) + ((lo << 8) >>> 0)) >>> 0;
      lo = product >>> 0;
    }
    this.hi = hi;
    this.lo = lo;
    return this;
  }
  value(): bigint {
    return (BigInt(this.hi) << 32n) | BigInt(this.lo);
  }
}

export function eventLogChecksum(payload: Uint8Array): bigint {
  return new Fnv64().update(payload).value();
}

const readExact = (descriptor: number, target: Uint8Array, position: number): boolean => {
  let filled = 0;
  while (filled < target.length) {
    const count = readSync(descriptor, target, filled, target.length - filled, position + filled);
    if (count === 0) return false;
    filled += count;
  }
  return true;
};

type Record = { readonly start: number; readonly length: number };
type Scan = { readonly records: readonly Record[]; readonly validLength: number };

/** 🩹️ Walks complete records through bounded reads; an incomplete or unverifiable final record is excluded (`validLength` ends before it), corruption elsewhere throws. */
function scan(descriptor: number): Scan {
  const size = fstatSync(descriptor).size;
  const header = new Uint8Array(HEADER);
  const view = new DataView(header.buffer);
  const chunk = new Uint8Array(CHUNK);
  const records: Record[] = [];
  let cursor = 0;
  while (cursor < size) {
    const remaining = size - cursor;
    const magicBytes = Math.min(remaining, MAGIC.length);
    const prefix = header.subarray(0, magicBytes);
    if (!readExact(descriptor, prefix, cursor)) break;
    for (let index = 0; index < magicBytes; index += 1) if (header[index] !== MAGIC[index]) throw new Error("folder event log: invalid magic");
    if (remaining < HEADER) break;
    if (!readExact(descriptor, header, cursor)) break;
    const length = view.getBigUint64(8, true);
    if (length > 16n * 1024n * 1024n * 1024n) throw new Error("folder event log: record exceeds the boundary");
    if (BigInt(remaining - HEADER) < length) break;
    const bytes = Number(length);
    const expected = view.getBigUint64(16, true);
    const hash = new Fnv64();
    for (let done = 0; done < bytes; done += CHUNK) {
      const part = chunk.subarray(0, Math.min(CHUNK, bytes - done));
      if (!readExact(descriptor, part, cursor + HEADER + done)) throw new Error("folder event log: short read");
      hash.update(part);
    }
    const final = cursor + HEADER + bytes === size;
    if (hash.value() !== expected) {
      if (final) break;
      throw new Error("folder event log: checksum mismatch");
    }
    records.push({ start: cursor, length: bytes });
    cursor += HEADER + bytes;
  }
  return { records, validLength: cursor };
}

/** 📖️ The newest archive of `documentId` in the log, or `null` when the log or the document is absent; only that archive's bytes are held. */
export function readLatestArchive(path: string, documentId: string): Uint8Array | null {
  if (!existsSync(path)) return null;
  const descriptor = openSync(path, "r");
  try {
    const { records } = scan(descriptor);
    const decoder = new TextDecoder("utf-8", { fatal: true });
    for (let index = records.length - 1; index >= 0; index -= 1) {
      const { start, length } = records[index]!;
      const prefix = new Uint8Array(Math.min(length, 9 + 4 + 1024 + 4 + 4096 + 8));
      if (!readExact(descriptor, prefix, start + HEADER)) continue;
      if (prefix[0] !== ARCHIVE_PUT_EVENT) continue;
      const view = new DataView(prefix.buffer);
      const keyLength = view.getUint32(9, true);
      if (decoder.decode(prefix.subarray(13, 13 + keyLength)) !== documentId) continue;
      const metadataAt = 13 + keyLength;
      const dataAt = metadataAt + 4 + view.getUint32(metadataAt, true) + 8;
      const archive = new Uint8Array(length - dataAt);
      if (!readExact(descriptor, archive, start + HEADER + dataAt)) throw new Error("folder event log: short read");
      return archive;
    }
    return null;
  } finally {
    closeSync(descriptor);
  }
}

/** ✍️ Appends one archive put after truncating an interrupted final record: header and key are written first, the archive is hashed and written from the caller's buffer, then the file and directory entry are flushed. */
export function appendArchive(path: string, documentId: string, schema: string, archive: Uint8Array): void {
  const encoder = new TextEncoder();
  const key = encoder.encode(documentId);
  const metadata = encoder.encode(schema);
  const prefix = new Uint8Array(9 + 4 + key.length + 4 + metadata.length + 8);
  const prefixView = new DataView(prefix.buffer);
  prefix[0] = ARCHIVE_PUT_EVENT;
  prefixView.setBigUint64(1, BigInt(Date.now()), true);
  let cursor = 9;
  for (const text of [key, metadata]) {
    prefixView.setUint32(cursor, text.length, true);
    prefix.set(text, cursor + 4);
    cursor += 4 + text.length;
  }
  prefixView.setBigUint64(cursor, BigInt(archive.length), true);
  const payloadLength = prefix.length + archive.length;
  const header = new Uint8Array(HEADER);
  header.set(MAGIC, 0);
  const headerView = new DataView(header.buffer);
  headerView.setBigUint64(8, BigInt(payloadLength), true);
  headerView.setBigUint64(16, new Fnv64().update(prefix).update(archive).value(), true);
  const created = !existsSync(path);
  mkdirSync(dirname(path), { recursive: true });
  const descriptor = openSync(path, "a+");
  try {
    const { validLength } = scan(descriptor);
    ftruncateSync(descriptor, validLength);
    let position = validLength;
    for (const part of [header, prefix]) position += writeSync(descriptor, part, 0, part.length, position);
    for (let done = 0; done < archive.length; ) done += writeSync(descriptor, archive, done, Math.min(CHUNK * 16, archive.length - done), position + done);
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
  if (created && process.platform !== "win32") {
    const directory = openSync(dirname(path), "r");
    try {
      fsyncSync(directory);
    } finally {
      closeSync(directory);
    }
  }
}
