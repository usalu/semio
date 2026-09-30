/** 🔖️ Owns bounded source walks and the exact metadata staged beside plugin artifacts. */
import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { readdir as readdirAsync, readFile as readFileAsync, stat as statAsync } from "node:fs/promises";
import { createHash, randomUUID } from "node:crypto";
import { join } from "node:path";

/** 🗑️ Directory names inside a component's owner tree that hold BUILD OUTPUT, never the sources whose
 * mtime decides whether the staged module is behind. Walking them would make every crate permanently
 * "stale" the moment its own `dist/component-dev/*.wasm` lands. */
export const UNWATCHED_COMPONENT_SOURCE_DIRECTORIES: readonly string[] = ["dist", "target", "node_modules", "pkg", ".git"];

/** 🛂️ File names at a component's OWNER ROOT that are build output too, even though they sit next to the
 * sources instead of inside a `dist`: the plugin descriptor pair `🔣️.json` + `🛂️.descriptor.semio`, which
 * the crate's `describe` target declares as its own `outputs` and rewrites in place (tracked in git, so
 * they look like authored files to everything that only reads mtimes). Counting them as sources made the
 * serve report EVERY component `source-newer` seconds after a describe pass — 30 `[stale]` lines on
 * 2026-09-22 15:57 naming nothing but `✏️s/🔌️plugins/<p>/🔣️.json` — while the staged descriptor was
 * current: `materialize-<profile>` does not copy this file, it re-emits its own from the built component
 * (`🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts`), so the two differ only in the `hashes.*` of the
 * build each was made from. A real source edit still reports, because the `.rs` it lives in is still
 * walked. ONLY the owner root is exempt: a nested `🔣️.json` is a fixture or a schema, i.e. a real source. */
export const GENERATED_COMPONENT_OWNER_FILES: readonly string[] = ["🔣️.json", "🛂️.descriptor.semio"];

/** 🔒️ Bound on one component's source walk so a serve-start freshness pass over ~20 crates stays a
 * few milliseconds and can never be turned into an unbounded repository scan by a stray symlink. */
export const COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20_000;

/** 🚦️ Source files one freshness walk stats or hashes at once, and components one freshness pass walks at once: the pass
 * runs beside a dev server, so it stays asynchronous end to end and bounded — 60 components × 54 131 files hashed
 * synchronously kept `serve s react dev` from spawning Vite for 49 s (ticket 26/09/23 F3). */
export const SOURCE_FRESHNESS_FILE_CONCURRENCY = 32;
export const SOURCE_FRESHNESS_COMPONENT_CONCURRENCY = 4;

/** 🚦️ Maps `items` through `map` with at most `limit` in flight, results in input order; a cancelled `signal` stops
 * taking new items and rejects with its reason. */
export async function mapBoundedV1<T, R>(items: readonly T[], limit: number, map: (item: T) => Promise<R>, signal?: AbortSignal): Promise<R[]> {
  const results = new Array<R>(items.length);
  let next = 0;
  const lane = async (): Promise<void> => {
    while (next < items.length) {
      signal?.throwIfAborted();
      const index = next++;
      results[index] = await map(items[index]!);
    }
  };
  await Promise.all(Array.from({ length: Math.max(1, Math.min(limit, items.length)) }, lane));
  signal?.throwIfAborted();
  return results;
}

/** 📂️ One directory's entries, or none when it cannot be read — structurally typed so this module keeps
 * its node-builtin-only import surface (`⚙️vite.config.ts` bundles it on every dev-server boot). */
function readableDirectoryEntries(directory: string): readonly { readonly name: string; isSymbolicLink(): boolean; isDirectory(): boolean; isFile(): boolean }[] {
  try { return readdirSync(directory, { withFileTypes: true }); } catch { return []; }
}

/** 🕰️ Newest regular-file mtime under one component's source tree, output directories excluded and the
 * walk bounded. `undefined` when the tree is absent or holds no readable source file. */
export function newestComponentSourceMtime(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): { readonly mtimeMs: number; readonly path: string } | undefined {
  if (!existsSync(sourceRoot)) return undefined;
  let newest: { mtimeMs: number; path: string } | undefined, visited = 0;
  const pending = [sourceRoot];
  while (pending.length > 0) {
    const directory = pending.pop()!;
    for (const entry of readableDirectoryEntries(directory)) {
      if (++visited > maximumEntries) return newest;
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!UNWATCHED_COMPONENT_SOURCE_DIRECTORIES.includes(entry.name)) pending.push(path);
        continue;
      }
      if (!entry.isFile()) continue;
      if (directory === sourceRoot && GENERATED_COMPONENT_OWNER_FILES.includes(entry.name)) continue;
      let mtimeMs: number;
      try { mtimeMs = statSync(path).mtimeMs; } catch { continue; }
      if (!newest || mtimeMs > newest.mtimeMs) newest = { mtimeMs, path };
    }
  }
  return newest;
}

/** 🕰️ Newest regular-file mtime among one staged module directory's own files. */
export function stagedModuleMtime(moduleDirectory: string): number | undefined {
  if (!existsSync(moduleDirectory)) return undefined;
  let newest: number | undefined;
  for (const entry of readableDirectoryEntries(moduleDirectory)) {
    if (!entry.isFile() || entry.name === ".nx-artifact.json") continue;
    try { const { mtimeMs } = statSync(join(moduleDirectory, entry.name)); if (newest === undefined || mtimeMs > newest) newest = mtimeMs; } catch { continue; }
  }
  return newest;
}

/** 🔖️ Content hash of one plugin-owner source tree — same walk bounds as {@link newestComponentSourceMtime},
 * keyed only on plugin sources (never framework). Empty trees yield the empty-input SHA-256. */
export async function componentSourceContentHash(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal?: AbortSignal): Promise<string> {
  return (await buildComponentSourceStatIndex(sourceRoot, maximumEntries, signal)).contentSha256;
}


/** 🔖 Sibling marker next to a staged module recording the plugin-source content hash used to build it. */
export const STAGED_SOURCE_CONTENT_HASH_FILE = ".source-content-sha256";

/** 🔖 Sibling marker next to a staged module recording the per-file source stat index used for boot freshness. */
export const STAGED_SOURCE_STAT_INDEX_FILE = ".source-stat-index.json";

/** 🧾️ The closed set admitted by both staging writers and artifact materialization. */
export const STAGED_SOURCE_FRESHNESS_FILES: readonly string[] = [STAGED_SOURCE_CONTENT_HASH_FILE, STAGED_SOURCE_STAT_INDEX_FILE];

export type SourceStatFileEntry = Readonly<{
  readonly path: string;
  readonly size: number;
  readonly mtimeNs: string;
  readonly sha256: string;
}>;

export type SourceStatIndex = Readonly<{
  readonly schema: "semio.dev.source-stat-index/v1";
  readonly contentSha256: string;
  readonly files: readonly SourceStatFileEntry[];
}>;

export type BootSourceHashResolution = Readonly<{
  readonly sourceContentSha256: string;
  readonly stagedSourceContentSha256?: string;
  readonly newestSourceMs?: number;
  readonly newestSourcePath?: string;
  readonly hashedFileCount: number;
  readonly reusedFileCount: number;
}>;

/** 📖 Reads the staged source-content hash marker, if present and well-formed. */
export function readStagedSourceContentHash(moduleDirectory: string): string | undefined {
  try {
    const value = readFileSync(join(moduleDirectory, STAGED_SOURCE_CONTENT_HASH_FILE), "utf8").trim();
    return /^[a-f0-9]{64}$/.test(value) ? value : undefined;
  } catch { return undefined; }
}

/** 🔖 Writes the staged source-content hash marker for one module directory. */
export function writeStagedSourceContentHash(moduleDirectory: string, sourceContentSha256: string): void {
  if (!/^[a-f0-9]{64}$/.test(sourceContentSha256)) throw new Error("Invalid source content hash");
  mkdirSync(moduleDirectory, { recursive: true });
  const temporary = join(moduleDirectory, `.source-content-${sourceContentSha256.slice(0, 8)}-${randomUUID()}.stage`);
  try {
    writeFileSync(temporary, sourceContentSha256 + "\n");
    renameSync(temporary, join(moduleDirectory, STAGED_SOURCE_CONTENT_HASH_FILE));
  } finally { rmSync(temporary, { force: true }); }
}

function fileMtimeNs(stats: { mtimeNs?: bigint; mtimeMs: number }): string {
  if (typeof stats.mtimeNs === "bigint") return stats.mtimeNs.toString();
  return String(BigInt(Math.round(stats.mtimeMs * 1_000_000)));
}

/** 🕰 Lists plugin-owner source files under one root (same bounds as content hashing), sorted. */
export async function listComponentSourceFiles(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): Promise<readonly string[]> {
  if (!existsSync(sourceRoot)) return [];
  let visited = 0;
  const pending = [sourceRoot];
  const files: string[] = [];
  while (pending.length > 0) {
    const directory = pending.pop()!;
    let entries: import("node:fs").Dirent<string>[];
    try {
      entries = await readdirAsync(directory, { withFileTypes: true, encoding: "utf8" });
    } catch {
      continue;
    }
    for (const entry of entries) {
      if (++visited > maximumEntries) return files.sort();
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!UNWATCHED_COMPONENT_SOURCE_DIRECTORIES.includes(entry.name)) pending.push(path);
        continue;
      }
      if (!entry.isFile()) continue;
      if (directory === sourceRoot && GENERATED_COMPONENT_OWNER_FILES.includes(entry.name)) continue;
      files.push(path);
    }
  }
  return files.sort();
}


function relativeSourcePath(sourceRoot: string, absolutePath: string): string {
  return absolutePath.slice(sourceRoot.length).replace(/^[/\\]+/, "").split(/[/\\]/).join("/");
}

function aggregateSourceContentHash(entries: readonly { readonly path: string; readonly sha256: string }[]): string {
  const hash = createHash("sha256");
  for (const entry of [...entries].sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0))) {
    hash.update(entry.path);
    hash.update("\0");
    hash.update(entry.sha256);
    hash.update("\0");
  }
  return hash.digest("hex");
}

/** 🧮️ One owner tree's walk: every source file stated with bounded parallelism, a prior digest reused when its size and
 * mtime match, every other file read and hashed — asynchronous end to end, so no event loop waits on a tree walk. */
type SourceStatWalk = Readonly<{ files: readonly SourceStatFileEntry[]; newestSourceMs?: number; newestSourcePath?: string; hashedFileCount: number; reusedFileCount: number }>;

async function walkSourceStats(sourceRoot: string, prior: ReadonlyMap<string, SourceStatFileEntry>, maximumEntries: number, signal?: AbortSignal): Promise<SourceStatWalk> {
  const absolutes = await listComponentSourceFiles(sourceRoot, maximumEntries);
  const parts = await mapBoundedV1(absolutes, SOURCE_FRESHNESS_FILE_CONCURRENCY, async (absolute) => {
    let stats: Awaited<ReturnType<typeof statAsync>>;
    try { stats = await statAsync(absolute); } catch { return undefined; }
    const path = relativeSourcePath(sourceRoot, absolute);
    const mtimeNs = fileMtimeNs(stats);
    const previous = prior.get(path);
    if (previous && previous.size === stats.size && previous.mtimeNs === mtimeNs) return { entry: previous, hashed: false, mtimeMs: stats.mtimeMs, absolute };
    let bytes: Buffer;
    try { bytes = await readFileAsync(absolute); } catch { return undefined; }
    return { entry: { path, size: stats.size, mtimeNs, sha256: createHash("sha256").update(bytes).digest("hex") }, hashed: true, mtimeMs: stats.mtimeMs, absolute };
  }, signal);
  const files: SourceStatFileEntry[] = [];
  let hashedFileCount = 0, reusedFileCount = 0, newestSourceMs: number | undefined, newestSourcePath: string | undefined;
  for (const part of parts) {
    if (!part) continue;
    files.push(part.entry);
    if (part.hashed) hashedFileCount += 1; else reusedFileCount += 1;
    if (newestSourceMs === undefined || part.mtimeMs > newestSourceMs) {
      newestSourceMs = part.mtimeMs;
      newestSourcePath = part.absolute;
    }
  }
  return { files, newestSourceMs, newestSourcePath, hashedFileCount, reusedFileCount };
}

/** 🔖 Builds the per-file source stat index and aggregate content hash for one plugin owner tree. */
export async function buildComponentSourceStatIndex(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal?: AbortSignal): Promise<SourceStatIndex> {
  const { files } = await walkSourceStats(sourceRoot, new Map(), maximumEntries, signal);
  return {
    schema: "semio.dev.source-stat-index/v1",
    contentSha256: aggregateSourceContentHash(files),
    files,
  };
}

/** 📖 Reads the staged source-stat index, if present and well-formed. */
export function readStagedSourceStatIndex(moduleDirectory: string): SourceStatIndex | undefined {
  try {
    const parsed: unknown = JSON.parse(readFileSync(join(moduleDirectory, STAGED_SOURCE_STAT_INDEX_FILE), "utf8"));
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return undefined;
    const row = parsed as Record<string, unknown>;
    if (row.schema !== "semio.dev.source-stat-index/v1") return undefined;
    if (typeof row.contentSha256 !== "string" || !/^[a-f0-9]{64}$/.test(row.contentSha256)) return undefined;
    if (!Array.isArray(row.files)) return undefined;
    const files: SourceStatFileEntry[] = [];
    for (const item of row.files) {
      if (!item || typeof item !== "object" || Array.isArray(item)) return undefined;
      const file = item as Record<string, unknown>;
      if (typeof file.path !== "string" || file.path.length === 0) return undefined;
      if (typeof file.size !== "number" || !Number.isInteger(file.size) || file.size < 0) return undefined;
      if (typeof file.mtimeNs !== "string" || !/^[0-9]+$/.test(file.mtimeNs)) return undefined;
      if (typeof file.sha256 !== "string" || !/^[a-f0-9]{64}$/.test(file.sha256)) return undefined;
      files.push({ path: file.path, size: file.size, mtimeNs: file.mtimeNs, sha256: file.sha256 });
    }
    return { schema: "semio.dev.source-stat-index/v1", contentSha256: row.contentSha256, files };
  } catch { return undefined; }
}

/** 🔖 Writes the staged source-stat index for one module directory. */
export function writeStagedSourceStatIndex(moduleDirectory: string, index: SourceStatIndex): void {
  if (index.schema !== "semio.dev.source-stat-index/v1") throw new Error("Invalid source stat index schema");
  if (!/^[a-f0-9]{64}$/.test(index.contentSha256)) throw new Error("Invalid source content hash");
  mkdirSync(moduleDirectory, { recursive: true });
  const temporary = join(moduleDirectory, `.source-stat-${index.contentSha256.slice(0, 8)}-${randomUUID()}.stage`);
  try {
    writeFileSync(temporary, `${JSON.stringify(index)}\n`);
    renameSync(temporary, join(moduleDirectory, STAGED_SOURCE_STAT_INDEX_FILE));
  } finally { rmSync(temporary, { force: true }); }
}

/** 🔖 Persists both the aggregate content-hash marker and the per-file stat index for one staged module. */
export async function writeStagedSourceFreshness(moduleDirectory: string, sourceRoot: string, signal?: AbortSignal): Promise<string> {
  const index = await buildComponentSourceStatIndex(sourceRoot, COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal);
  writeStagedSourceContentHash(moduleDirectory, index.contentSha256);
  writeStagedSourceStatIndex(moduleDirectory, index);
  return index.contentSha256;
}

/** 🕰 Boot freshness: walk the owner tree asynchronously, reuse per-file digests whose (size, mtimeNs) match the staged
 * index and hash only added or changed files; without an index every file is hashed — still asynchronously, so a dev
 * server keeps answering while it runs. `signal` cancels the walk. */
export async function resolveBootSourceContentHashes(options: {
  readonly sourceRoot: string;
  readonly moduleDirectory: string;
  readonly receiptSourceContentSha256?: string;
  readonly signal?: AbortSignal;
}): Promise<BootSourceHashResolution> {
  const stagedSourceContentSha256 = readStagedSourceContentHash(options.moduleDirectory) ?? options.receiptSourceContentSha256;
  const prior = new Map((readStagedSourceStatIndex(options.moduleDirectory)?.files ?? []).map((file) => [file.path, file]));
  const walk = await walkSourceStats(options.sourceRoot, prior, COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, options.signal);
  return {
    sourceContentSha256: aggregateSourceContentHash(walk.files),
    stagedSourceContentSha256,
    newestSourceMs: walk.newestSourceMs,
    newestSourcePath: walk.newestSourcePath,
    hashedFileCount: walk.hashedFileCount,
    reusedFileCount: walk.reusedFileCount,
  };
}

