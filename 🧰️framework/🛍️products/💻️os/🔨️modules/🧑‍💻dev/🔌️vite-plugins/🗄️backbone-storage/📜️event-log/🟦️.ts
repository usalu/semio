/** 📜️ The folder event log `.semio/events.semio` (`SEMIOEL1`), the one canonical on-disk store of folder documents shared with the native actor (`FolderEventLogStorage`). Records are `magic | length u64 | checksum u64 | payload`; an archive put carries kind 4, the document id as key and the schema as metadata. */
import { closeSync, fsyncSync, ftruncateSync, mkdirSync, openSync, readFileSync, writeSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

const MAGIC = new TextEncoder().encode("SEMIOEL1");
const ARCHIVE_PUT_EVENT = 4;
const FNV_OFFSET = 0xcbf29ce484222325n;
const FNV_PRIME = 0x100000001b3n;
const U64 = 0xffffffffffffffffn;
const HEADER = 24;

/** 📍️ The canonical event log path inside a document folder. */
export function eventLogPath(folder: string): string {
  return join(folder, ".semio", "events.semio");
}

function checksum(payload: Uint8Array): bigint {
  let hash = FNV_OFFSET;
  for (const byte of payload) hash = ((hash ^ BigInt(byte)) * FNV_PRIME) & U64;
  return hash;
}

type Record = { readonly start: number; readonly payload: Uint8Array };
type Scan = { readonly records: readonly Record[]; readonly validLength: number };

/** 🩹️ Walks complete records; an incomplete or unverifiable final record is excluded (`validLength` ends before it), corruption elsewhere throws. */
function scan(bytes: Uint8Array): Scan {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const records: Record[] = [];
  let cursor = 0;
  while (cursor < bytes.length) {
    const remaining = bytes.length - cursor;
    const magicBytes = Math.min(remaining, MAGIC.length);
    for (let index = 0; index < magicBytes; index += 1) if (bytes[cursor + index] !== MAGIC[index]) throw new Error("folder event log: invalid magic");
    if (remaining < HEADER) break;
    const length = view.getBigUint64(cursor + 8, true);
    if (length > 16n * 1024n * 1024n * 1024n) throw new Error("folder event log: record exceeds the boundary");
    if (BigInt(remaining - HEADER) < length) break;
    const payload = bytes.subarray(cursor + HEADER, cursor + HEADER + Number(length));
    const final = cursor + HEADER + Number(length) === bytes.length;
    if (checksum(payload) !== view.getBigUint64(cursor + 16, true)) {
      if (final) break;
      throw new Error("folder event log: checksum mismatch");
    }
    records.push({ start: cursor, payload });
    cursor += HEADER + Number(length);
  }
  return { records, validLength: cursor };
}

type Decoded = { readonly kind: number; readonly key: string; readonly primary: Uint8Array };

function decode(payload: Uint8Array): Decoded {
  const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  const decoder = new TextDecoder("utf-8", { fatal: true });
  let cursor = 9;
  const text = (): string => {
    const length = view.getUint32(cursor, true);
    cursor += 4;
    const value = decoder.decode(payload.subarray(cursor, cursor + length));
    cursor += length;
    return value;
  };
  const data = (): Uint8Array => {
    const length = Number(view.getBigUint64(cursor, true));
    cursor += 8;
    const value = payload.subarray(cursor, cursor + length);
    cursor += length;
    return value;
  };
  const key = text();
  text();
  return { kind: payload[0]!, key, primary: data() };
}

/** 📖️ The newest archive of `documentId` in the log, or `null` when the log or the document is absent. */
export function readLatestArchive(path: string, documentId: string): Uint8Array | null {
  if (!existsSync(path)) return null;
  const { records } = scan(new Uint8Array(readFileSync(path)));
  for (let index = records.length - 1; index >= 0; index -= 1) {
    const event = decode(records[index]!.payload);
    if (event.kind === ARCHIVE_PUT_EVENT && event.key === documentId) return Uint8Array.from(event.primary);
  }
  return null;
}

/** ✍️ Appends one archive put after truncating an interrupted final record, flushes the file and the directory entry. */
export function appendArchive(path: string, documentId: string, schema: string, archive: Uint8Array): void {
  const encoder = new TextEncoder();
  const key = encoder.encode(documentId);
  const metadata = encoder.encode(schema);
  const payload = new Uint8Array(9 + 4 + key.length + 4 + metadata.length + 8 + archive.length + 8);
  const view = new DataView(payload.buffer);
  let cursor = 0;
  payload[cursor] = ARCHIVE_PUT_EVENT;
  view.setBigUint64(1, BigInt(Date.now()), true);
  cursor = 9;
  for (const text of [key, metadata]) {
    view.setUint32(cursor, text.length, true);
    payload.set(text, cursor + 4);
    cursor += 4 + text.length;
  }
  view.setBigUint64(cursor, BigInt(archive.length), true);
  payload.set(archive, cursor + 8);
  const record = new Uint8Array(HEADER + payload.length);
  record.set(MAGIC, 0);
  const header = new DataView(record.buffer);
  header.setBigUint64(8, BigInt(payload.length), true);
  header.setBigUint64(16, checksum(payload), true);
  record.set(payload, HEADER);
  const created = !existsSync(path);
  mkdirSync(dirname(path), { recursive: true });
  const descriptor = openSync(path, "a+");
  try {
    const { validLength } = scan(new Uint8Array(readFileSync(path)));
    ftruncateSync(descriptor, validLength);
    writeSync(descriptor, record, 0, record.length, validLength);
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
