import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstat, mkdir, readdir, readFile, rename, rm, stat, writeFile } from "node:fs/promises";
import { join, sep } from "node:path";
import { inferenceHeaderLine, observationsHold } from "../../../🔌️nx-plugin/👁️observation/🟨️.mjs";

/** 🔏️ The persisted seal format; see `./🧬️schema/🔣️.json`. */
export const NX_GRAPH_SEAL_FORMAT = 1;

const SEAL_FILE = "semio-graph-seal.json";
const PREVIOUS_SEAL_FILE = "semio-graph-seal.previous.json";
const GRAPH_FILES = ["project-graph.json", "file-map.json"] as const;
const RECEIPTS = ["emoji-project-json.receipt.json", "test-cases.receipt.json"] as const;
const WALK_CONCURRENCY = 64;
const NEVER_INDEXED = new Set([".git", "node_modules"]);
const INPUT_FILE = /^(?:package\.json|project\.json|📋️project\.json|pyproject\.toml|Cargo\.toml|uv\.lock|bun\.lock|package-lock\.json|yarn\.lock|pnpm-lock\.yaml|nx\.json|\.nxignore|\.gitignore|tsconfig(?:\.[^/]*)?\.json|[^/]*\.patch|[^/]*\.feature)$/u;

/** 🏷️ The identity of one node: device, inode, size and both times; any content or entry change alters it. */
type Stamp = readonly [dev: number, ino: number, size: number, mtimeMs: number, ctimeMs: number];

/** 🔏️ Proof that a cached Nx project graph was computed from inputs that can be re-verified without recomputing it. */
export interface NxGraphSeal {
  readonly format: typeof NX_GRAPH_SEAL_FORMAT;
  readonly nx: string;
  readonly graph: Readonly<Record<string, Stamp>>;
  readonly receipts: Readonly<Record<string, string>>;
  readonly directories: readonly DirectoryRow[];
  readonly inputs: readonly (readonly [path: string, stamp: Stamp])[];
}

/** 📂️ One indexable directory: its stamp, the indexable child directories, graph-input files and symbolic links it holds (by name). */
type DirectoryRow = readonly [path: string, stamp: Stamp, children: readonly string[], inputs: readonly string[], symlinks: readonly string[]];

/** 🏷️ One node's stamp. */
function stampOf(value: { dev: number; ino: number; size: number; mtimeMs: number; ctimeMs: number }): Stamp {
  return [value.dev, value.ino, value.size, value.mtimeMs, value.ctimeMs];
}

/** ⚖️ Stamp equality. */
function sameStamp(left: Stamp, right: Stamp): boolean {
  return left.every((value, index) => value === right[index]);
}

/** 🧾️ SHA-256 of bytes or text. */
function sha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}

/** 🚫️ Whole-tree exclusions `.nxignore` declares as `/<path>/**` or `**\/<name>/**`; anything else is walked (a superset of Nx's view). */
function nxIgnoredTrees(text: string): Readonly<{ anchored: ReadonlySet<string>; names: ReadonlySet<string> }> {
  const anchored = new Set<string>(), names = new Set<string>();
  for (const raw of text.split(/\r?\n/u)) {
    const line = raw.trim();
    const root = line.match(/^\/([^*?[\]!#]+)\/\*\*$/u), name = line.match(/^\*\*\/([^/*?[\]!#]+)\/\*\*$/u);
    if (root) anchored.add(root[1]!);
    else if (name) names.add(name[1]!);
  }
  return { anchored, names };
}

/** 🧵️ Runs `visit` over a growing work list with at most `limit` visits in flight. */
async function boundedWalk<T>(initial: readonly T[], limit: number, visit: (item: T, enqueue: (next: T) => void) => Promise<void>): Promise<void> {
  const pending = [...initial];
  let active = 0;
  await new Promise<void>((accept, reject) => {
    let failed = false;
    const pump = (): void => {
      while (!failed && active < limit && pending.length) {
        active++;
        visit(pending.pop()!, (next) => { pending.push(next); }).then(() => { active--; if (!failed) { if (pending.length) pump(); else if (active === 0) accept(); } }, (error: unknown) => { if (!failed) { failed = true; reject(error); } });
      }
      if (!failed && active === 0 && pending.length === 0) accept();
    };
    pump();
  });
}

/**
 * 🗺️ Every directory Nx can index under `root` with its stamp, and the stamps of every graph-input file in them, walked level by level:
 * child directories newly listed pass one batched `git check-ignore` per level, and only `.git`, `node_modules`, `.nxignore` whole trees and
 * Git-ignored directories are left out, so any file Nx could glob lives in a stamped directory. A directory whose stamp equals its row in
 * `previous` reuses that row's listing (entries only change by restamping their directory).
 */
async function graphInputTree(root: string, previous: ReadonlyMap<string, DirectoryRow>): Promise<Readonly<{ directories: DirectoryRow[]; inputs: [string, Stamp][] }>> {
  let ignoreText = "";
  try { ignoreText = await readFile(join(root, ".nxignore"), "utf8"); } catch {}
  const trees = nxIgnoredTrees(ignoreText), directories: DirectoryRow[] = [], inputs: [string, Stamp][] = [];
  for (let level = [""]; level.length > 0;) {
    const reused: string[] = [], listed: string[] = [];
    await boundedWalk(level, WALK_CONCURRENCY, async (path) => {
      const absolute = nativePath(root, path), value = await lstat(absolute);
      if (!value.isDirectory()) return;
      const stamp = stampOf(value), prior = previous.get(path);
      let row: DirectoryRow;
      if (prior && sameStamp(prior[1], stamp)) row = prior;
      else {
        const children: string[] = [], files: string[] = [], links: string[] = [];
        for (const entry of await readdir(absolute, { withFileTypes: true })) {
          const child = path ? `${path}/${entry.name}` : entry.name;
          if (entry.isSymbolicLink()) links.push(entry.name);
          if (entry.isDirectory()) { if (!NEVER_INDEXED.has(entry.name) && !trees.anchored.has(child) && !trees.names.has(entry.name)) children.push(entry.name); }
          else if (INPUT_FILE.test(entry.name)) files.push(entry.name);
        }
        row = [path, stamp, children.sort(), files.sort(), links.sort()];
      }
      directories.push(row);
      for (const name of row[3]) { const child = path ? `${path}/${name}` : name; inputs.push([child, stampOf(await lstat(nativePath(root, child)))]); }
      for (const name of row[2]) (row === prior ? reused : listed).push(path ? `${path}/${name}` : name);
    });
    let admitted: string[] = [];
    if (listed.length > 0) {
      const check = spawnSync("git", ["check-ignore", "--stdin", "-z", "--no-index"], { cwd: root, input: listed.map((path) => `${path}/`).join("\0") + "\0", maxBuffer: 1 << 28 });
      if (check.status !== 0 && check.status !== 1) throw new Error(`git check-ignore failed (${check.status}): ${check.stderr.toString("utf8")}`);
      const ignored = new Set(check.stdout.toString("utf8").split("\0").filter(Boolean).map((path) => path.replace(/\/$/u, "")));
      admitted = listed.filter((path) => !ignored.has(path));
    }
    level = [...reused, ...admitted];
  }
  const kept = new Set(directories.map(([path]) => path));
  return { directories: directories.map((row) => [row[0], row[1], row[2].filter((name) => kept.has(row[0] ? `${row[0]}/${name}` : name)), row[3], row[4]] as const).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0), inputs: inputs.sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0) };
}

/** 🙈️ The ignore authorities among recorded inputs, by path and stamp, so any added, removed or edited ignore file is noticed. */
function ignoreAuthorities(inputs: readonly (readonly [string, Stamp])[]): string {
  return JSON.stringify(inputs.filter(([path]) => /(?:^|\/)\.(?:git|nx)ignore$/u.test(path)));
}

/** 🧭️ The Nx tooling identity a graph depends on: the installed Nx and plugin package manifests and `nx.json`. */
async function nxIdentity(root: string): Promise<string> {
  const parts = await Promise.all(["node_modules/nx/package.json", "node_modules/@nx/js/package.json", "node_modules/@nxlv/python/package.json", "nx.json"].map(async (path) => { try { return await readFile(join(root, path)); } catch { return Buffer.from(`!${path}`); } }));
  return sha256(Buffer.concat(parts.flatMap((part) => [Buffer.from(`${part.byteLength}:`), part])));
}

/** 🗂️ Stamps of the graph cache files, or `null` when one is missing. */
async function graphStamps(workspaceData: string): Promise<Record<string, Stamp> | null> {
  const stamps: Record<string, Stamp> = {};
  for (const name of GRAPH_FILES) { try { stamps[name] = stampOf(await stat(join(workspaceData, name))); } catch { return null; } }
  return stamps;
}

/**
 * ✍️ Seals the graph a finished Nx process (`pid`, started at `startedAt`) left in `workspaceData`, when both plugin receipts are that
 * process's and the graph was written after them; otherwise removes any seal. Returns whether a seal was written.
 */
export async function writeNxGraphSeal(root: string, workspaceData: string, pid: number, startedAt: number): Promise<boolean> {
  const sealPath = join(workspaceData, SEAL_FILE);
  await rm(sealPath, { force: true });
  const graph = await graphStamps(workspaceData), receipts: Record<string, string> = {};
  if (!graph || graph["project-graph.json"]![3] < startedAt) return false;
  for (const name of RECEIPTS) {
    let text: string;
    try { text = await readFile(join(workspaceData, "inference", name), "utf8"); } catch { return false; }
    const receipt = JSON.parse(text) as { pid: number; completedAt: number };
    if (receipt.pid !== pid || receipt.completedAt < startedAt || receipt.completedAt > graph["project-graph.json"]![3]) return false;
    receipts[name] = sha256(text);
  }
  let prior: NxGraphSeal | undefined;
  try { prior = JSON.parse(await readFile(join(workspaceData, PREVIOUS_SEAL_FILE), "utf8")) as NxGraphSeal; } catch {}
  let tree = await graphInputTree(root, new Map(prior?.format === NX_GRAPH_SEAL_FORMAT ? prior.directories.map((row) => [row[0], row] as const) : []));
  if (prior && ignoreAuthorities(tree.inputs) !== ignoreAuthorities(prior.inputs)) tree = await graphInputTree(root, new Map());
  const seal: NxGraphSeal = { format: NX_GRAPH_SEAL_FORMAT, nx: await nxIdentity(root), graph, receipts, directories: tree.directories, inputs: tree.inputs };
  if (JSON.stringify(await graphStamps(workspaceData)) !== JSON.stringify(graph)) return false;
  await mkdir(workspaceData, { recursive: true });
  const temporary = `${sealPath}.${process.pid}.tmp`, text = JSON.stringify(seal);
  await writeFile(temporary, text);
  await rename(temporary, sealPath);
  await writeFile(`${temporary}.previous`, text);
  await rename(`${temporary}.previous`, join(workspaceData, PREVIOUS_SEAL_FILE));
  return true;
}

/** 🧭️ The absolute native path of a repository-relative POSIX path, built without re-normalizing (seal paths are already normal). */
function nativePath(root: string, path: string): string {
  return path ? `${root}${sep}${sep === "/" ? path : path.replaceAll("/", sep)}` : root;
}

/** 🏷️ True when every recorded path still carries its stamp, checked `WALK_CONCURRENCY` at a time. */
async function stampsHold(root: string, rows: readonly (readonly [string, Stamp])[], read: (path: string) => Promise<{ dev: number; ino: number; size: number; mtimeMs: number; ctimeMs: number }>): Promise<boolean> {
  let next = 0, holds = true;
  const worker = async (): Promise<void> => {
    while (holds && next < rows.length) {
      const [path, stamp] = rows[next++]!;
      try { if (!sameStamp(stampOf(await read(nativePath(root, path))), stamp)) holds = false; } catch { holds = false; }
    }
  };
  await Promise.all(Array.from({ length: Math.min(WALK_CONCURRENCY, rows.length) }, worker));
  return holds;
}

/**
 * ✅️ True when the sealed graph in `workspaceData` still equals what Nx would compute: the Nx tooling and graph files are unchanged, no
 * indexable directory gained, lost or renamed an entry, no graph-input file changed, and every filesystem fact both plugins' node
 * inference and dependency computations observed still holds. Nx may then reuse its cached graph (`NX_FORCE_REUSE_CACHED_GRAPH`).
 */
export async function nxGraphSealHolds(root: string, workspaceData: string): Promise<boolean> {
  let seal: NxGraphSeal;
  try { seal = JSON.parse(await readFile(join(workspaceData, SEAL_FILE), "utf8")) as NxGraphSeal; } catch { return false; }
  if (seal.format !== NX_GRAPH_SEAL_FORMAT || seal.nx !== await nxIdentity(root)) return false;
  const graph = await graphStamps(workspaceData);
  if (!graph || GRAPH_FILES.some((name) => !seal.graph[name] || !sameStamp(graph[name]!, seal.graph[name]!))) return false;
  const facts: unknown[] = [];
  for (const name of RECEIPTS) {
    let text: string;
    try { text = await readFile(join(workspaceData, "inference", name), "utf8"); } catch { return false; }
    if (sha256(text) !== seal.receipts[name]) return false;
    const receipt = JSON.parse(text) as { inference: [string, string][]; observations: unknown[] };
    for (const [cachePath, digest] of receipt.inference) {
      let header: string | null;
      try { header = inferenceHeaderLine(cachePath); } catch { return false; }
      if (header === null || sha256(header) !== digest) return false;
      facts.push(...(JSON.parse(header) as { observations: unknown[] }).observations);
    }
    facts.push(...receipt.observations);
  }
  return await stampsHold(root, seal.directories.map(([path, stamp]) => [path, stamp] as const), lstat) && await stampsHold(root, seal.inputs, lstat) && await observationsHold(facts, 64, new Set(seal.directories.map(([path]) => nativePath(root, path))), new Set(seal.directories.flatMap(([path, , , , links]) => links.map((name) => nativePath(root, path ? `${path}/${name}` : name)))));
}
