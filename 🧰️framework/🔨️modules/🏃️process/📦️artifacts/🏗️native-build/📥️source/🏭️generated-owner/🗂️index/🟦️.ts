import type { BigIntStats } from "node:fs";
import { createHash } from "node:crypto";
import { mkdir, readFile, readdir, rename, stat, writeFile } from "node:fs/promises";
import { basename, dirname, join, relative, sep } from "node:path";

/** 📍️ A position in a change feed. */
export type WorkspaceFeedCursorV1 = Readonly<{ epoch: string; seq: number }>;
/** 🚫️ A feed that cannot vouch for an exact answer right now. */
export type WorkspaceFeedRefusedV1 = Readonly<{ exact: false }>;
/** 📡️ An exact, fenced change feed of one workspace: the paths that may have changed since a cursor, as `[kind, isDirectory, workspace-relative path]` rows, where a directory row of kind other than `m` stands for its whole subtree. The journal client of the repository satisfies it structurally.
 * @see ../../../../../../../📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts */
export interface WorkspaceChangeFeedV1 {
  excluder(): (path: string) => boolean;
  cursor(signal?: AbortSignal): Promise<WorkspaceFeedCursorV1 | WorkspaceFeedRefusedV1>;
  since(from: WorkspaceFeedCursorV1, roots: readonly string[], signal?: AbortSignal): Promise<Readonly<{ exact: true; cursor: WorkspaceFeedCursorV1; rows: readonly (readonly [string, 0 | 1, string])[] }> | WorkspaceFeedRefusedV1>;
}
/** 🎛️ Cancellation, progress and the optional change feed of one call; `report` runs once per listing. With a feed that vouches exactly, a repeated call reads only what the feed names (no directory or file is stamped), and with a `store` directory a fresh process resumes from the snapshot an earlier process saved there at a cursor of the same feed run; without a feed, or when it refuses, every directory is stamped. */
export type WorkspaceIndexControlV1 = Readonly<{ signal?: AbortSignal; report?: () => void | Promise<void>; changes?: WorkspaceChangeFeedV1; store?: string }>;
/** 🗂️ What a walk finds: the files whose names end in one of the suffixes, or equal one of the names, and are listed by every directory below the root. */
export type WorkspaceIndexQueryV1 = Readonly<{ skipped: ReadonlySet<string>; suffixes: readonly string[]; names: readonly string[] }>;
export type WorkspaceIndexListingV1 = Readonly<{ suffixed: readonly string[]; named: readonly string[] }>;

type DirectoryRowV1 = Readonly<{ stamp: string; trusted: boolean; directories: readonly string[]; suffixed: readonly string[]; named: readonly string[] }>;
type FileRowV1 = Readonly<{ stamp: string; trusted: boolean; value: unknown }>;
type IndexV1 = { directories: Map<string, DirectoryRowV1>; files: Map<string, Map<string, FileRowV1>>; cursor: WorkspaceFeedCursorV1 | undefined; busy: Promise<unknown> };

/** 🕰️ A stamp whose newest time lies this close behind the moment it was read is not trusted: a later change in the same clock tick of a coarse file system (FAT and HFS+ keep 1 s or 2 s) would carry the same stamp, so the entry is read again next time (the racy-entry rule of the git index). */
export const WORKSPACE_INDEX_RACY_MILLISECONDS_V1 = 3000;
/** 📦️ More changed rows than this in one answer are cheaper to settle by stamping everything again. */
const MAXIMUM_FEED_ROWS_V1 = 20000;
const MAXIMUM_ROOTS_V1 = 4, READ_CONCURRENCY_V1 = 64;
const VANISHED_V1 = new Set(["ENOENT", "ENOTDIR"]);
const indexes = new Map<string, IndexV1>();

const vanished = (error: unknown): boolean => VANISHED_V1.has((error as NodeJS.ErrnoException | null)?.code ?? "");
const stamped = (row: BigIntStats): { stamp: string; newestMs: number } => ({
  stamp: [row.dev, row.ino, row.mode, row.nlink, row.size, row.mtimeNs, row.ctimeNs].join(":"),
  newestMs: Number((row.mtimeNs > row.ctimeNs ? row.mtimeNs : row.ctimeNs) / 1000000n),
});
const statBig = (path: string) => stat(path, { bigint: true }).catch(error => { if (vanished(error)) return undefined; throw error; });
const indexOf = (root: string, query: WorkspaceIndexQueryV1): IndexV1 => {
  const key = [root, ...[...query.skipped].sort(), "", ...query.suffixes, "", ...query.names].join("\0");
  const known = indexes.get(key);
  if (known !== undefined) { indexes.delete(key); indexes.set(key, known); return known; }
  const created: IndexV1 = { directories: new Map(), files: new Map(), cursor: undefined, busy: Promise.resolve() };
  indexes.set(key, created);
  for (const stale of [...indexes.keys()].slice(0, Math.max(0, indexes.size - MAXIMUM_ROOTS_V1))) indexes.delete(stale);
  return created;
};
const exclusive = <T>(index: IndexV1, run: () => Promise<T>): Promise<T> => {
  const result = index.busy.then(run, run);
  index.busy = result.catch(() => undefined);
  return result;
};

/** 📖️ Stamps and lists one directory; `undefined` when it vanished or is no directory. */
async function readRow(directory: string, query: WorkspaceIndexQueryV1): Promise<DirectoryRowV1 | undefined> {
  const before = Date.now(), first = await statBig(directory);
  if (first === undefined || !first.isDirectory()) return undefined;
  const { stamp, newestMs } = stamped(first);
  let entries;
  try { entries = await readdir(directory, { withFileTypes: true }); } catch (error) { if (vanished(error)) return undefined; throw error; }
  const directories: string[] = [], suffixed: string[] = [], named: string[] = [];
  for (const entry of entries) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { if (!query.skipped.has(entry.name)) directories.push(path); }
    else if (query.suffixes.some(suffix => entry.name.endsWith(suffix))) suffixed.push(path);
    else if (query.names.includes(entry.name)) named.push(path);
  }
  return { stamp, trusted: before - newestMs > WORKSPACE_INDEX_RACY_MILLISECONDS_V1, directories, suffixed, named };
}

/** 🌲️ Lists `directory` and everything below it into `target`; a directory whose stamp equals the one in `known` and was not racy is taken from there. */
async function walk(directory: string, query: WorkspaceIndexQueryV1, known: ReadonlyMap<string, DirectoryRowV1> | undefined, target: Map<string, DirectoryRowV1>, signal: AbortSignal | undefined): Promise<void> {
  signal?.throwIfAborted();
  const old = known?.get(directory);
  let row: DirectoryRowV1 | undefined;
  if (old !== undefined && old.trusted) {
    const first = await statBig(directory);
    if (first === undefined || !first.isDirectory()) return;
    row = stamped(first).stamp === old.stamp ? old : await readRow(directory, query);
  } else row = await readRow(directory, query);
  if (row === undefined) return;
  target.set(directory, row);
  await Promise.all(row.directories.map(child => walk(child, query, known, target, signal)));
}

const forgetFiles = (index: IndexV1, paths: readonly string[]): void => { for (const table of index.files.values()) for (const path of paths) table.delete(path); };
const dropSubtree = (index: IndexV1, directory: string): void => {
  const stack = [directory];
  for (let next = stack.pop(); next !== undefined; next = stack.pop()) {
    const row = index.directories.get(next);
    if (row === undefined) continue;
    index.directories.delete(next);
    forgetFiles(index, row.suffixed);
    forgetFiles(index, row.named);
    stack.push(...row.directories);
  }
};

/** 📡️ Settles the index from the rows the feed names since the remembered cursor; `false` when the feed cannot answer exactly, in which case the cursor is forgotten. */
async function catchUp(index: IndexV1, root: string, query: WorkspaceIndexQueryV1, control: WorkspaceIndexControlV1): Promise<boolean> {
  const feed = control.changes;
  if (feed === undefined || index.cursor === undefined || index.directories.size === 0) return false;
  const answer = await feed.since(index.cursor, [""], control.signal);
  if (!("rows" in answer) || answer.rows.length > MAXIMUM_FEED_ROWS_V1) { index.cursor = undefined; return false; }
  const fresh = new Set<string>(), relist = new Set<string>();
  const rows = [...answer.rows].sort((left, right) => left[2] < right[2] ? -1 : left[2] > right[2] ? 1 : 0);
  const covered = (path: string): boolean => { for (let at = path; at !== root && at.length > root.length; at = dirname(at)) if (fresh.has(at)) return true; return false; };
  for (const [kind, isDirectory, relativePath] of rows) {
    control.signal?.throwIfAborted();
    const segments = relativePath.split("/").filter(Boolean);
    if (segments.length === 0 || segments.some(segment => query.skipped.has(segment))) continue;
    const path = join(root, ...segments);
    forgetFiles(index, [path]);
    let known = dirname(path);
    while (!index.directories.has(known) && known.length > root.length) known = dirname(known);
    relist.add(known);
    if (covered(path)) continue;
    if (isDirectory === 1 && kind !== "m") {
      dropSubtree(index, path);
      await walk(path, query, undefined, index.directories, control.signal);
      fresh.add(path);
    } else if (index.directories.has(path)) relist.add(path);
  }
  const ordered = [...relist].filter(directory => !covered(directory)).sort((left, right) => left.length - right.length);
  for (const directory of ordered) {
    control.signal?.throwIfAborted();
    const old = index.directories.get(directory);
    if (old === undefined) continue;
    const row = await readRow(directory, query);
    if (row === undefined) { dropSubtree(index, directory); continue; }
    for (const child of old.directories) if (!row.directories.includes(child)) dropSubtree(index, child);
    forgetFiles(index, old.suffixed.filter(path => !row.suffixed.includes(path)));
    forgetFiles(index, old.named.filter(path => !row.named.includes(path)));
    index.directories.set(directory, row);
    for (const child of row.directories) if (!index.directories.has(child)) await walk(child, query, undefined, index.directories, control.signal);
  }
  index.cursor = answer.cursor;
  return true;
}

/** 🚶️ Stamps the whole tree again and, when the feed vouches for it, remembers the cursor taken before the walk, so that the next call reads only what the feed names. */
async function rewalk(index: IndexV1, root: string, query: WorkspaceIndexQueryV1, control: WorkspaceIndexControlV1): Promise<void> {
  const feed = control.changes;
  const taken = feed === undefined ? undefined : await feed.cursor(control.signal);
  const next = new Map<string, DirectoryRowV1>();
  try { await walk(root, query, index.directories, next, control.signal); } catch (error) { for (const [path, row] of next) index.directories.set(path, row); throw error; }
  index.directories = next;
  if (feed === undefined) return;
  index.cursor = undefined;
  if (taken === undefined || "exact" in taken) return;
  const excluded = feed.excluder();
  for (const path of next.keys()) {
    const name = relative(root, path).split(sep).join("/");
    if (name !== "" && excluded(name)) return;
  }
  index.files.clear();
  index.cursor = taken;
}

/** 🗺️ Lists the files below `root` that match `query` and returns the same answer as a fresh full walk, in a fraction of the time once the process has walked the root before.
 *
 * With a change feed that vouches exactly, a repeated call re-lists only the directories the feed names (a changed entry always names its parent) and touches nothing else. Without one, every directory is stamped (device, inode, mode, link count, size, modification and change time in nanoseconds) before it is listed, and only directories whose stamp changed, or was racy when read, are listed again; a deleted directory drops out with its subtree either way.
 * @see https://git-scm.com/docs/racy-git */
export async function listWorkspaceIndexedV1(root: string, query: WorkspaceIndexQueryV1, control: WorkspaceIndexControlV1 = {}): Promise<WorkspaceIndexListingV1> {
  const index = indexOf(root, query);
  await control.report?.();
  return exclusive(index, async () => {
    control.signal?.throwIfAborted();
    if (control.changes !== undefined) await restore(index, root, query, control.store);
    if (!await catchUp(index, root, query, control)) { await rewalk(index, root, query, control); await persist(index, root, query, control.store); }
    const suffixed: string[] = [], named: string[] = [];
    for (const row of index.directories.values()) { suffixed.push(...row.suffixed); named.push(...row.named); }
    return { suffixed, named };
  });
}

/** 🧠️ Answers `derive(path)` for every path, reusing the answer of an earlier call while the file is unchanged: by the change feed when it vouches exactly (no file is stamped), else by its stamp when it was not racy when read (see {@link WORKSPACE_INDEX_RACY_MILLISECONDS_V1}). A vanished file answers `undefined` when stamped; a `derive` that throws is not remembered and the error is thrown. `kind` names the question, so one root keeps one table per question.
 * @see {@link listWorkspaceIndexedV1} */
export async function deriveWorkspaceIndexedV1<T>(root: string, query: WorkspaceIndexQueryV1, kind: string, paths: readonly string[], derive: (path: string) => Promise<T>, control: WorkspaceIndexControlV1 = {}): Promise<(T | undefined)[]> {
  const index = indexOf(root, query);
  return exclusive(index, async () => {
    control.signal?.throwIfAborted();
    const fed = await catchUp(index, root, query, control);
    const previous = index.files.get(kind) ?? new Map<string, FileRowV1>(), next = fed ? previous : new Map<string, FileRowV1>();
    index.files.set(kind, next);
    const results: (T | undefined)[] = new Array(paths.length);
    let at = 0;
    const work = async (): Promise<void> => {
      while (at < paths.length) {
        control.signal?.throwIfAborted();
        const slot = at++, path = paths[slot]!, known = previous.get(path);
        if (fed) {
          if (known !== undefined) { results[slot] = known.value as T; continue; }
          const value = await counted(path);
          next.set(path, { stamp: "", trusted: true, value });
          results[slot] = value;
          continue;
        }
        const before = Date.now(), first = await statBig(path);
        if (first === undefined) continue;
        const { stamp, newestMs } = stamped(first);
        if (known !== undefined && known.trusted && known.stamp === stamp) { next.set(path, known); results[slot] = known.value as T; continue; }
        const value = await derive(path);
        next.set(path, { stamp, trusted: before - newestMs > WORKSPACE_INDEX_RACY_MILLISECONDS_V1, value });
        results[slot] = value;
      }
    };
    let computed = 0;
    const counted = (path: string): Promise<T> => { computed++; return derive(path); };
    try { await Promise.all(Array.from({ length: Math.min(READ_CONCURRENCY_V1, paths.length) }, work)); } catch (error) { if (!fed) for (const [path, row] of next) previous.set(path, row); index.files.set(kind, fed ? next : previous); throw error; }
    if (fed && computed >= PERSIST_ROWS_V1) await persist(index, root, query, control.store);
    return results;
  });
}

const PERSISTED_VERSION_V1 = 1, PERSIST_ROWS_V1 = 500;
type PersistedV1 = Readonly<{ version: number; root: string; epoch: string; seq: number; directories: readonly (readonly [number, string, readonly string[], readonly string[]])[]; files: Readonly<Record<string, readonly (readonly [number, string, unknown])[]>> }>;
const storeFile = (store: string, root: string, query: WorkspaceIndexQueryV1): string =>
  join(store, `index-${createHash("sha256").update([root, ...[...query.skipped].sort(), "", ...query.suffixes, "", ...query.names].join("\0")).digest("hex").slice(0, 24)}.json`);

/** 💾️ Saves the directory rows and derived answers, which are exactly those at the remembered cursor, to `store` by writing a temporary file and renaming it, so a reader never sees half a snapshot; a failure to save loses only the speed-up. Derived answers must be JSON values. */
async function persist(index: IndexV1, root: string, query: WorkspaceIndexQueryV1, store: string | undefined): Promise<void> {
  if (store === undefined || index.cursor === undefined) return;
  const names = (paths: readonly string[]) => paths.map(path => basename(path)), order = new Map<string, number>();
  for (const path of index.directories.keys()) order.set(path, order.size);
  const snapshot: PersistedV1 = {
    version: PERSISTED_VERSION_V1, root, epoch: index.cursor.epoch, seq: index.cursor.seq,
    directories: [...index.directories].map(([path, row]) => [order.get(dirname(path)) ?? -1, path === root ? "" : basename(path), names(row.suffixed), names(row.named)] as const),
    files: Object.fromEntries([...index.files].map(([kind, table]) => [kind, [...table].flatMap(([path, row]) => { const owner = order.get(dirname(path)); return owner === undefined ? [] : [[owner, basename(path), row.value] as const]; })])),
  };
  const file = storeFile(store, root, query), temporary = `${file}.${process.pid}.${Date.now()}.tmp`;
  try { await mkdir(store, { recursive: true }); await writeFile(temporary, JSON.stringify(snapshot)); await rename(temporary, file); } catch { return; }
}

/** 📂️ Restores the snapshot a process saved earlier into an empty index; the feed then decides, by cursor, whether it is still exact. */
async function restore(index: IndexV1, root: string, query: WorkspaceIndexQueryV1, store: string | undefined): Promise<void> {
  if (store === undefined || index.directories.size > 0) return;
  let saved: PersistedV1;
  try { saved = JSON.parse(await readFile(storeFile(store, root, query), "utf8")) as PersistedV1; } catch { return; }
  if (saved.version !== PERSISTED_VERSION_V1 || saved.root !== root) return;
  const paths: string[] = [], children: string[][] = [];
  const directories = new Map<string, DirectoryRowV1>();
  saved.directories.forEach(([parent, name], at) => { paths.push(parent < 0 ? root : join(paths[parent]!, name)); children.push([]); if (parent >= 0) children[parent]!.push(paths[at]!); });
  saved.directories.forEach(([, , suffixed, named], at) => {
    const directory = paths[at]!;
    directories.set(directory, { stamp: "", trusted: false, directories: children[at]!, suffixed: suffixed.map(name => join(directory, name)), named: named.map(name => join(directory, name)) });
  });
  index.directories = directories;
  index.files = new Map(Object.entries(saved.files).map(([kind, rows]) => [kind, new Map(rows.map(([owner, name, value]) => [join(paths[owner]!, name), { stamp: "", trusted: true, value }] as const))]));
  index.cursor = { epoch: saved.epoch, seq: saved.seq };
}

/** 🧹️ Forgets everything remembered about `root`, so the next call walks it cold; what tests use to compare against a fresh full walk. */
export function forgetWorkspaceIndexV1(root?: string): void {
  for (const key of [...indexes.keys()]) if (root === undefined || key.startsWith(`${root}\0`)) indexes.delete(key);
}
