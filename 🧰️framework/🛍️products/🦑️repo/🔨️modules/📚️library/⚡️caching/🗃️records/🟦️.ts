import { closeSync, mkdirSync, openSync, readdirSync, readFileSync, readSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import { createHash, randomBytes } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";

/** 🗃️ The on-disk index format version; see `./🧬️schema/🔣️.json`. */
export const RECORD_STORE_FORMAT = 1;

const RECORD_KEY = /^[0-9a-f]{64}$/u;
const PACK_NAME = /^[0-9a-z]+-[0-9]+-[0-9a-f]{16}$/u;
const ORPHAN_MS = 60 * 60 * 1000;
const PENDING_FLUSH_BYTES = 64 * 1024 * 1024;

/** 📇️ One pack index as persisted next to its pack: every record's key, byte range and value digest prefix. */
export interface RecordStoreIndex {
  readonly format: typeof RECORD_STORE_FORMAT;
  readonly pack: string;
  readonly packBytes: number;
  readonly records: readonly (readonly [key: string, offset: number, length: number, digest: string])[];
}

/** 📊️ Counters a caller reports to prove the store's effect. */
export interface RecordStoreStats { readonly hits: number; readonly misses: number; readonly puts: number; readonly packs: number; readonly records: number; readonly accessed: number }

/**
 * 🗄️ A content-addressed, append-only record store: values are opaque bytes under 64-hex keys that the caller derives from every input
 * of the computation that produced them, so a hit is indistinguishable from recomputing. Records are written in immutable packs whose
 * index is published by rename after the pack, so concurrent processes only ever see complete packs; a damaged or vanished record reads
 * as a miss, never as wrong bytes (each value is verified against its recorded digest).
 */
export interface RecordStore {
  has(key: string): boolean;
  get(key: string): Uint8Array | undefined;
  put(key: string, value: Uint8Array): void;
  flush(): void;
  retainAccessed(): void;
  close(): void;
  readonly stats: RecordStoreStats;
}

interface Location { readonly pack: string; readonly offset: number; readonly length: number; readonly digest: string }

/** 🧾️ The first 32 hex digits of a value's SHA-256, enough to reject damaged bytes. */
function valueDigest(value: Uint8Array): string {
  return createHash("sha256").update(value).digest("hex").slice(0, 32);
}

/** ✅️ Parses one persisted index, or `null` when it is not exactly a well-formed index of a complete pack named like it. */
export function parseRecordStoreIndex(text: string, name: string, packBytes: number | null): RecordStoreIndex | null {
  let value: unknown;
  try { value = JSON.parse(text); } catch { return null; }
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const index = value as Record<string, unknown>;
  if (Object.keys(index).sort().join() !== "format,pack,packBytes,records" || index.format !== RECORD_STORE_FORMAT || index.pack !== name || !Number.isSafeInteger(index.packBytes) || index.packBytes !== packBytes || !Array.isArray(index.records)) return null;
  let end = 0;
  for (const record of index.records as unknown[]) {
    if (!Array.isArray(record) || record.length !== 4) return null;
    const [key, offset, length, digest] = record as unknown[];
    if (typeof key !== "string" || !RECORD_KEY.test(key) || offset !== end || !Number.isSafeInteger(length) || (length as number) < 0 || typeof digest !== "string" || !/^[0-9a-f]{32}$/u.test(digest)) return null;
    end += length as number;
  }
  return end === packBytes ? index as unknown as RecordStoreIndex : null;
}

/** 🏷️ A fresh pack name: creation time, process and randomness, so concurrent writers never collide. */
function packName(): string {
  return `${Date.now().toString(36)}-${process.pid}-${randomBytes(8).toString("hex")}`;
}

/** ✍️ Publishes one pack: the bytes first, then the index by rename, so a reader that sees the index sees the whole pack. */
function writePack(directory: string, records: Iterable<readonly [string, Uint8Array]>): RecordStoreIndex {
  const name = packName(), packPath = join(directory, `${name}.pack`), temporary = `${packPath}.${randomBytes(4).toString("hex")}.tmp`;
  const rows: [string, number, number, string][] = [];
  const fd = openSync(temporary, "w");
  let offset = 0;
  try {
    for (const [key, value] of records) {
      for (let written = 0; written < value.byteLength;) written += writeSync(fd, value, written, value.byteLength - written);
      rows.push([key, offset, value.byteLength, valueDigest(value)]);
      offset += value.byteLength;
    }
  } finally { closeSync(fd); }
  renameSync(temporary, packPath);
  const index: RecordStoreIndex = { format: RECORD_STORE_FORMAT, pack: name, packBytes: offset, records: rows };
  const indexPath = join(directory, `${name}.idx`), indexTemporary = `${indexPath}.${randomBytes(4).toString("hex")}.tmp`;
  writeFileSync(indexTemporary, JSON.stringify(index));
  renameSync(indexTemporary, indexPath);
  return index;
}

/** 🧹️ Removes one file, tolerating it being gone or held open (a later compaction retries). */
function removeQuietly(path: string): void {
  try { rmSync(path, { force: true }); } catch {}
}

/** 📂️ Opens the store rooted at `directory` (created on demand), loading every complete pack index present. */
export function openRecordStore(directory: string): RecordStore {
  const root = resolve(directory);
  mkdirSync(root, { recursive: true });
  const locations = new Map<string, Location>(), packs = new Set<string>(), descriptors = new Map<string, number | null>();
  const pending = new Map<string, Uint8Array>(), accessed = new Set<string>(), damaged = new Set<string>();
  let hits = 0, misses = 0, puts = 0, pendingBytes = 0;
  const now = Date.now();
  for (const entry of readdirSync(root).sort()) {
    if (entry.endsWith(".tmp")) {
      try { if (now - statSync(join(root, entry)).mtimeMs > ORPHAN_MS) removeQuietly(join(root, entry)); } catch {}
      continue;
    }
    if (!entry.endsWith(".idx")) continue;
    const name = entry.slice(0, -4);
    if (!PACK_NAME.test(name)) continue;
    let packBytes: number | null = null;
    try { packBytes = statSync(join(root, `${name}.pack`)).size; } catch {}
    let text: string;
    try { text = readFileSync(join(root, entry), "utf8"); } catch { continue; }
    const index = parseRecordStoreIndex(text, name, packBytes);
    if (!index) continue;
    packs.add(name);
    for (const [key, offset, length, digest] of index.records) locations.set(key, { pack: name, offset, length, digest });
  }
  const descriptor = (pack: string): number | null => {
    if (!descriptors.has(pack)) { try { descriptors.set(pack, openSync(join(root, `${pack}.pack`), "r")); } catch { descriptors.set(pack, null); } }
    return descriptors.get(pack)!;
  };
  const read = (key: string): Uint8Array | undefined => {
    const staged = pending.get(key);
    if (staged) return staged;
    const location = locations.get(key);
    if (!location) return undefined;
    const fd = descriptor(location.pack), bytes = Buffer.allocUnsafe(location.length);
    let filled = 0;
    try { while (fd !== null && filled < location.length) { const count = readSync(fd, bytes, filled, location.length - filled, location.offset + filled); if (count === 0) break; filled += count; } } catch {}
    if (filled === location.length && valueDigest(bytes) === location.digest) return bytes;
    damaged.add(key);
    return undefined;
  };
  const store: RecordStore = {
    has(key) { return pending.has(key) || locations.has(key) && !damaged.has(key); },
    get(key) {
      const value = read(key);
      if (value === undefined) misses++;
      else { hits++; accessed.add(key); }
      return value;
    },
    put(key, value) {
      if (!RECORD_KEY.test(key)) throw new Error(`Record key must be 64 lowercase hex digits: ${key}`);
      accessed.add(key);
      if (locations.has(key) && !damaged.has(key) || pending.has(key)) return;
      pending.set(key, value);
      puts++;
      pendingBytes += value.byteLength;
      if (pendingBytes >= PENDING_FLUSH_BYTES) store.flush();
    },
    flush() {
      if (pending.size === 0) return;
      const index = writePack(root, [...pending]);
      packs.add(index.pack);
      for (const [key, offset, length, digest] of index.records) { locations.set(key, { pack: index.pack, offset, length, digest }); damaged.delete(key); }
      pending.clear();
      pendingBytes = 0;
    },
    retainAccessed() {
      store.flush();
      const live = function* (): Generator<readonly [string, Uint8Array]> { for (const key of [...accessed].sort()) { const value = read(key); if (value !== undefined) yield [key, value]; } };
      const index = writePack(root, live()), retired = [...packs].filter((pack) => pack !== index.pack);
      for (const pack of retired) {
        const fd = descriptors.get(pack);
        if (typeof fd === "number") closeSync(fd);
        descriptors.delete(pack);
        removeQuietly(join(root, `${pack}.idx`));
        removeQuietly(join(root, `${pack}.pack`));
        packs.delete(pack);
      }
      locations.clear();
      for (const [key, offset, length, digest] of index.records) locations.set(key, { pack: index.pack, offset, length, digest });
      packs.add(index.pack);
    },
    close() {
      store.flush();
      for (const fd of descriptors.values()) if (typeof fd === "number") closeSync(fd);
      descriptors.clear();
    },
    get stats() { return { hits, misses, puts, packs: packs.size, records: locations.size + pending.size, accessed: accessed.size }; },
  };
  return store;
}

/** 🔑️ The record key of one computation: SHA-256 over its namespace and every input, each length-prefixed so no two input lists collide. */
export function recordKey(namespace: string, ...inputs: readonly (string | Uint8Array)[]): string {
  const hash = createHash("sha256");
  for (const part of [namespace, ...inputs]) {
    const bytes = typeof part === "string" ? Buffer.from(part, "utf8") : part;
    hash.update(`${bytes.byteLength}:`).update(bytes);
  }
  return hash.digest("hex");
}

const STATIC_IMPORT = /(?:^|[\s;])(?:import|export)\s[^"'`;]*?\bfrom\s*["']([^"']+)["']|(?:^|[\s;])import\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']\s*\)/gu;

/**
 * 🧬️ A digest of one module's source closure: its own bytes plus, recursively, every relative module it imports statically or through a
 * literal dynamic `import()`, in path order. Records keyed by it are invalidated by any edit to the code that could have produced them.
 */
export function moduleSourceClosureDigest(entry: string): string {
  const seen = new Map<string, Buffer>(), pending = [resolve(entry)];
  while (pending.length > 0) {
    const file = pending.pop()!;
    if (seen.has(file)) continue;
    const bytes = readFileSync(file);
    seen.set(file, bytes);
    for (const match of bytes.toString("utf8").matchAll(STATIC_IMPORT)) {
      const specifier = match[1] ?? match[2] ?? match[3];
      if (specifier?.startsWith(".")) pending.push(resolve(dirname(file), specifier));
    }
  }
  const base = dirname(resolve(entry));
  return recordKey("module-source-closure-v1", ...[...seen].map(([file, bytes]) => [relative(base, file).replaceAll("\\", "/"), bytes] as const).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).flatMap(([file, bytes]) => [file, bytes]));
}

/** 🧺️ Removes sibling store namespaces of `root` other than `keep` that nobody touched for an hour (older code revisions). */
export function pruneRecordStoreNamespaces(root: string, keep: string): void {
  let names: string[];
  try { names = readdirSync(root); } catch { return; }
  const now = Date.now();
  for (const name of names) {
    if (name === keep) continue;
    const path = join(root, name);
    try { const stat = statSync(path); if (stat.isDirectory() && now - stat.mtimeMs > ORPHAN_MS) rmSync(path, { recursive: true, force: true }); } catch {}
  }
}
