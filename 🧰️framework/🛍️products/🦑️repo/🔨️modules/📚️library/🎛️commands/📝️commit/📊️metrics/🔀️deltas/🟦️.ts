import { spawnCapturedSync } from "../../../../🏃️process/🔍️probe/🟦️.ts";
import { gitRepoRoot } from "../../🌳️git/🟦️.ts";
import { gitSpawnEnv } from "../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { classifyPathForMetrics, MAX_METRICS_FILE_BYTES, normalizeRepoPath, shouldSkipPathForUloc } from "../🛤️path-rules/🟦️.ts";
import { diffJsonUloc } from "../🔢️uloc/🟦️.ts";

/** ✂️Splits git numstat into replaced (edited), net added, and net removed lines. */
export function splitGitNumstatDelta(added: number, removed: number): { edited: number; added: number; removed: number } {
  const a = Math.max(0, added);
  const r = Math.max(0, removed);
  const edited = Math.min(a, r);
  return { edited, added: a - edited, removed: r - edited };
}

function parseGitNumstatZ(stdout: Buffer | string): { path: string; added: number; removed: number }[] {
  const raw = typeof stdout === "string" ? stdout : stdout.toString("utf8");
  if (!raw) return [];
  const out: { path: string; added: number; removed: number }[] = [];
  const entries = raw.split("\0");
  for (let index = 0; index < entries.length; index++) {
    const entry = entries[index]!;
    if (!entry) continue;
    const parts = entry.split("\t");
    if (parts.length < 3) continue;
    const added = parts[0] === "-" ? 0 : Number(parts[0]) || 0;
    const removed = parts[1] === "-" ? 0 : Number(parts[1]) || 0;
    let path = parts.slice(2).join("\t");
    if (!path && entries[index + 1] && entries[index + 2]) {
      path = `${entries[++index]!}\t${entries[++index]!}`;
    }
    if (path) out.push({ path, added, removed });
  }
  return out;
}

function gitCachedNumstat(root: string): { path: string; added: number; removed: number }[] {
  const r = spawnCapturedSync("git", ["diff", "--cached", "--numstat", "-z"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout);
}

function isJsonMetricsPath(path: string | null | undefined): boolean {
  if (!path) return false;
  const ext = path.slice(path.lastIndexOf(".")).toLowerCase();
  return ext === ".json" || ext === ".jsonc";
}

/** 📂️Whether a repo-relative path lies under any normalized prefix. */
export function pathUnderPrefixes(rel: string, prefixes: string[]): boolean {
  if (prefixes.length === 0) return true;
  const n = normalizeRepoPath(rel);
  for (const raw of prefixes) {
    const p = normalizeRepoPath(raw).replace(/\/$/, "");
    if (!p) continue;
    if (n === p || n.startsWith(`${p}/`)) return true;
  }
  return false;
}

/** 📈️Git numstat between two revisions. */
export function gitRangeNumstat(root: string, base: string, head: string): { path: string; added: number; removed: number }[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnCapturedSync("git", ["diff", "--numstat", "-z", `${base}..${head}`], {
    cwd: repoRoot,
    env: gitSpawnEnv(),
  });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout);
}

/** 🐙️Accumulates per-language git deltas from numstat rows (optional path prefixes). */
export function accumulateGitDeltasFromNumstat(root: string, rows: { path: string; added: number; removed: number }[], pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  const m = new Map<string, { added: number; removed: number; edited: number }>();
  for (const { path, added, removed } of rows) {
    if (shouldSkipPathForUloc(root, path)) continue;
    if (pathPrefixes && !pathUnderPrefixes(path, pathPrefixes)) continue;
    const lang = classifyPathForMetrics(path);
    if (!lang) continue;
    const d = splitGitNumstatDelta(added, removed);
    const row = m.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    row.edited += d.edited;
    row.added += d.added;
    row.removed += d.removed;
    m.set(lang, row);
  }
  return m;
}

function accumulateGitDeltas(root: string): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateGitDeltasFromNumstat(root, gitCachedNumstat(root));
}

function gitObjectSizesAtSpecs(root: string, specs: string[]): Map<string, number> {
  const unique = [...new Set(specs)];
  if (unique.length === 0) return new Map();
  const result = spawnCapturedSync("git", ["cat-file", "--batch-check=%(objectname)\t%(objecttype)\t%(objectsize)\t%(rest)"], {
    cwd: gitRepoRoot(root),
    env: gitSpawnEnv(),
    input: `${unique.join("\n")}\n`,
  });
  if (result.status !== 0) return new Map();
  const sizes = new Map<string, number>();
  const lines = result.stdout.toString("utf8").split("\n");
  for (let index = 0; index < unique.length; index++) {
    const line = lines[index] ?? "";
    const first = line.indexOf("\t");
    const second = first < 0 ? -1 : line.indexOf("\t", first + 1);
    const third = second < 0 ? -1 : line.indexOf("\t", second + 1);
    if (third < 0) continue;
    const type = line.slice(first + 1, second);
    const size = Number(line.slice(second + 1, third));
    if (type === "blob" && Number.isSafeInteger(size) && size >= 0) sizes.set(unique[index]!, size);
  }
  return sizes;
}

const METRICS_BLOB_BATCH_BYTES = 16 * 1024 * 1024;

function gitBlobTextsAtSpecs(root: string, specs: string[]): Map<string, string | null> {
  const unique = [...new Set(specs)];
  const sizes = gitObjectSizesAtSpecs(root, unique);
  const texts = new Map<string, string | null>();
  let batch: string[] = [];
  let batchBytes = 0;
  const flush = (): void => {
    if (batch.length === 0) return;
    const result = spawnCapturedSync("git", ["cat-file", "--batch"], {
      cwd: gitRepoRoot(root),
      env: gitSpawnEnv(),
      input: `${batch.join("\n")}\n`,
    });
    if (result.status !== 0) {
      for (const spec of batch) texts.set(spec, null);
      batch = [];
      batchBytes = 0;
      return;
    }
    const output = result.stdout;
    let offset = 0;
    for (const spec of batch) {
      const lineEnd = output.indexOf(0x0a, offset);
      if (lineEnd < 0) {
        texts.set(spec, null);
        continue;
      }
      const header = output.subarray(offset, lineEnd).toString("utf8");
      offset = lineEnd + 1;
      const match = /^[0-9a-f]+ blob (\d+)$/u.exec(header);
      if (!match) {
        texts.set(spec, null);
        continue;
      }
      const size = Number(match[1]);
      if (!Number.isSafeInteger(size) || size < 0 || offset + size > output.length) {
        texts.set(spec, null);
        continue;
      }
      const bytes = output.subarray(offset, offset + size);
      offset += size;
      if (output[offset] === 0x0a) offset++;
      texts.set(spec, bytes.includes(0) ? null : bytes.toString("utf8"));
    }
    batch = [];
    batchBytes = 0;
  };
  for (const spec of unique) {
    const size = sizes.get(spec);
    if (size === undefined || size > MAX_METRICS_FILE_BYTES) {
      texts.set(spec, null);
      continue;
    }
    if (batch.length > 0 && batchBytes + size > METRICS_BLOB_BATCH_BYTES) flush();
    batch.push(spec);
    batchBytes += size;
  }
  flush();
  return texts;
}

type MetricsPathPair = { oldPath: string | null; newPath: string | null };

function metricsPathPair(row: { path: string }): MetricsPathPair | null {
  const paths = pathsFromNumstatRow(row.path);
  if (paths.length === 0) return null;
  if (paths.length === 1) return { oldPath: paths[0]!, newPath: paths[0]! };
  return { oldPath: paths[0]!, newPath: paths[1]! };
}

function addLanguageDelta(deltas: Map<string, { added: number; removed: number; edited: number }>, language: string, delta: { added: number; removed: number; edited: number }): void {
  const current = deltas.get(language) ?? { added: 0, removed: 0, edited: 0 };
  current.added += delta.added;
  current.removed += delta.removed;
  current.edited += delta.edited;
  deltas.set(language, current);
}

type UlocDeltaEntry = { index: number; row: { path: string; added: number; removed: number }; pair: MetricsPathPair };

function ulocDeltaEntries(root: string, rows: { path: string; added: number; removed: number }[], pathPrefixes?: string[]): UlocDeltaEntry[] {
  return rows
    .map((row, index) => ({ index, row, pair: metricsPathPair(row) }))
    .filter((entry): entry is UlocDeltaEntry => entry.pair !== null)
    .filter(({ pair }) => {
      const paths = [pair.oldPath, pair.newPath].filter((path): path is string => path !== null);
      return paths.some((path) => !shouldSkipPathForUloc(root, path) && (!pathPrefixes || pathUnderPrefixes(path, pathPrefixes)));
    });
}

function ulocBlobSpecs(entries: UlocDeltaEntry[], oldRev: string, newRev: string): string[] {
  return entries.flatMap(({ pair }) => [...(isJsonMetricsPath(pair.oldPath) ? [`${oldRev}:${pair.oldPath}`] : []), ...(isJsonMetricsPath(pair.newPath) ? [`${newRev}:${pair.newPath}`] : [])]);
}

function ulocDeltaForEntry(root: string, entry: UlocDeltaEntry, oldRev: string, newRev: string, blobs: Map<string, string | null>): Map<string, { added: number; removed: number; edited: number }> {
  const { row, pair } = entry;
  const deltas = new Map<string, { added: number; removed: number; edited: number }>();
  const oldLanguage = pair.oldPath && !shouldSkipPathForUloc(root, pair.oldPath) ? classifyPathForMetrics(pair.oldPath) : "";
  const newLanguage = pair.newPath && !shouldSkipPathForUloc(root, pair.newPath) ? classifyPathForMetrics(pair.newPath) : "";
  const oldJson = isJsonMetricsPath(pair.oldPath) ? (blobs.get(`${oldRev}:${pair.oldPath}`) ?? null) : null;
  const newJson = isJsonMetricsPath(pair.newPath) ? (blobs.get(`${newRev}:${pair.newPath}`) ?? null) : null;
  if (oldLanguage === "JSON" && newLanguage === "JSON") {
    const delta = diffJsonUloc(oldJson, newJson);
    if (delta !== null) {
      addLanguageDelta(deltas, "JSON", delta);
      return deltas;
    }
  }
  if (oldLanguage && oldLanguage === newLanguage && oldLanguage !== "JSON") {
    addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(row.added, row.removed));
    return deltas;
  }
  if (oldLanguage === "JSON") {
    const delta = diffJsonUloc(oldJson, null);
    if (delta !== null) addLanguageDelta(deltas, oldLanguage, delta);
    else if (oldLanguage === newLanguage) addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(row.added, row.removed));
  } else if (oldLanguage) {
    addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(0, row.removed));
  }
  if (newLanguage === "JSON") {
    const delta = diffJsonUloc(null, newJson);
    if (delta !== null) addLanguageDelta(deltas, newLanguage, delta);
    else if (oldLanguage !== newLanguage) addLanguageDelta(deltas, newLanguage, splitGitNumstatDelta(row.added, 0));
  } else if (newLanguage) {
    addLanguageDelta(deltas, newLanguage, splitGitNumstatDelta(row.added, 0));
  }
  return deltas;
}

export function accumulateUlocDeltasByRow(root: string, rows: { path: string; added: number; removed: number }[], oldRev: string, newRev: string, pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }>[] {
  const deltas = rows.map(() => new Map<string, { added: number; removed: number; edited: number }>());
  const entries = ulocDeltaEntries(root, rows, pathPrefixes);
  const blobs = gitBlobTextsAtSpecs(root, ulocBlobSpecs(entries, oldRev, newRev));
  for (const entry of entries) deltas[entry.index] = ulocDeltaForEntry(root, entry, oldRev, newRev, blobs);
  return deltas;
}

/** 🧩️Accumulates per-language ULOC deltas from revision blobs, retaining JSON's key-based unit. */
export function accumulateUlocDeltasFromPaths(root: string, rows: { path: string; added: number; removed: number }[], oldRev: string, newRev: string, pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  const deltas = new Map<string, { added: number; removed: number; edited: number }>();
  for (const rowDeltas of accumulateUlocDeltasByRow(root, rows, oldRev, newRev, pathPrefixes)) {
    for (const [language, delta] of rowDeltas) addLanguageDelta(deltas, language, delta);
  }
  return deltas;
}

/** 📏️Accumulates per-language byte deltas from path size changes between two git revisions. */
export function accumulateSizeDeltasFromPaths(root: string, rows: { path: string }[], oldRev: string, newRev: string, pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  const m = new Map<string, { added: number; removed: number; edited: number }>();
  const paths = rows.flatMap((row) => pathsFromNumstatRow(row.path)).filter((path) => !shouldSkipPathForUloc(root, path) && (!pathPrefixes || pathUnderPrefixes(path, pathPrefixes)));
  const specs = paths.flatMap((path) => [`${oldRev}:${path}`, `${newRev === ":0" ? ":0" : newRev}:${path}`]);
  const sizes = gitObjectSizesAtSpecs(root, specs);
  for (const path of paths) {
    const lang = classifyPathForMetrics(path);
    if (!lang) continue;
    const oldBytes = sizes.get(`${oldRev}:${path}`) ?? 0;
    const newBytes = sizes.get(`${newRev === ":0" ? ":0" : newRev}:${path}`) ?? 0;
    const d = splitGitNumstatDelta(newBytes, oldBytes);
    const rowDelta = m.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    rowDelta.edited += d.edited;
    rowDelta.added += d.added;
    rowDelta.removed += d.removed;
    m.set(lang, rowDelta);
  }
  return m;
}

export function accumulateStagedSizeDeltas(root: string): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateSizeDeltasFromPaths(root, gitCachedNumstat(root), "HEAD", ":0");
}

export function accumulateStagedUlocDeltas(root: string): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateUlocDeltasFromPaths(root, gitCachedNumstat(root), "HEAD", ":0");
}

export function accumulateRangeSizeDeltas(root: string, base: string, head: string, pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateSizeDeltasFromPaths(root, gitRangeNumstat(root, base, head), base, head, pathPrefixes);
}

export function accumulateRangeUlocDeltas(root: string, base: string, head: string, pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateUlocDeltasFromPaths(root, gitRangeNumstat(root, base, head), base, head, pathPrefixes);
}

/** ➕️Sums git delta counters across all languages. */
export function sumGitLangDeltas(deltas: Map<string, { added: number; removed: number; edited: number }>): {
  added: number;
  removed: number;
  edited: number;
} {
  let added = 0;
  let removed = 0;
  let edited = 0;
  for (const d of deltas.values()) {
    added += d.added;
    removed += d.removed;
    edited += d.edited;
  }
  return { added, removed, edited };
}

/** 🟰️Sum of net added, edited (replaced), and removed line counts from git numstat. */
export function gitDeltaLineTotal(d: { added: number; removed: number; edited: number }): number {
  return d.added + d.edited + d.removed;
}

/** 🛤️Paths from one numstat row (rename rows may join old/new with tabs or `=>`). */
export function pathsFromNumstatRow(pathField: string): string[] {
  const parts = pathField
    .split("\t")
    .map((p) => p.trim())
    .filter(Boolean);
  if (parts.length === 0) return [];
  const out = new Set<string>();
  for (const part of parts) {
    const brace = /^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$/u.exec(part);
    if (brace) {
      const a = brace[2]!.trim();
      const b = brace[3]!.trim();
      if (a) out.add(`${brace[1]}${a}${brace[4]}`);
      if (b) out.add(`${brace[1]}${b}${brace[4]}`);
      continue;
    }
    const arrow = /\s*=>\s*/u.exec(part);
    if (arrow) {
      const [from, to] = part.split(/\s*=>\s*/u);
      if (from?.trim()) out.add(from.trim());
      if (to?.trim()) out.add(to.trim());
      continue;
    }
    out.add(part);
  }
  return [...out];
}
