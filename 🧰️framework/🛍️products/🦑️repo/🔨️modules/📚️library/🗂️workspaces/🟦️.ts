//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js
// Physical package membership and explicit package payload ownership.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
//#endregion 🔌️Adapters

//#region 🔎️WorkspaceRoot
/** 🔎️Uses Nx's execution workspace before standalone hints and workspace-manifest discovery. Owned here
 * rather than in the repository library barrel so a consumer that only needs the root path never pulls
 * the barrel (and its taxonomy discovery walk) into its module graph. */
export function getWorkspaceRoot(): string {
  const fromNx = process.env.NX_WORKSPACE_ROOT?.trim();
  if (fromNx) return resolve(fromNx);
  const fromEnv = process.env.REPO_ROOT?.trim();
  if (fromEnv) return resolve(fromEnv);
  let dir = process.cwd();
  for (let i = 0; i < 30; i++) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      try {
        const j = JSON.parse(readFileSync(pkg, "utf8")) as { name?: string };
        if (j.name === "workspace") return dir;
      } catch {
        /* ignore */
      }
    }
    const up = dirname(dir);
    if (up === dir) break;
    dir = up;
  }
  return process.cwd();
}
//#endregion 🔎️WorkspaceRoot

//#region 🔣️Constants
const MANIFEST_FILENAME = "package.json";

/** 🧺️ Directory names never descended into — build/vendor/scratch trees, never real workspace source.
 * Includes the schema-owned opaque `compose` boundary (same isolation as `DISCOVERY_SKIP_DIRS`) so
 * workspace generation cannot reintroduce its intentionally deleted memberships. */
const WORKSPACE_SCAN_SKIP_DIR_NAMES = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "storybook-static", "temp", "coverage", "🔌️plugin-modules", ".🧬semio", "compose"]);

//#endregion 🔣️Constants

//#region 🔍️Scan
/** 📦️ One directory discovered to carry its own `package.json`, with the manifest's `name` (if any). */
interface WorkspaceCandidate {
  readonly relDir: string;
  readonly absDir: string;
  readonly name?: string;
  readonly exports?: unknown;
}

export interface WorkspaceDiscoveryProgress {
  readonly candidatesDiscovered: number;
  readonly directoriesScanned: number;
  readonly relativeDirectory: string;
}

export interface WorkspaceDiscoveryEntry {
  readonly kind: "directory" | "file" | "symlink" | "other";
  readonly name: string;
}

export interface WorkspaceDiscoveryOperations {
  readonly list: (path: string) => readonly WorkspaceDiscoveryEntry[];
  readonly readText: (path: string) => string;
  readonly state: (path: string) => "directory" | "file" | "missing" | "symlink" | "other";
}

export interface WorkspaceDiscoveryOptions {
  readonly onProgress?: (progress: WorkspaceDiscoveryProgress) => void;
  readonly operations?: WorkspaceDiscoveryOperations;
  readonly signal?: Pick<AbortSignal, "aborted">;
}

function errorCode(error: unknown): string | undefined {
  return typeof error === "object" && error !== null && "code" in error ? String((error as { code?: unknown }).code) : undefined;
}

function nativeState(path: string): "directory" | "file" | "missing" | "symlink" | "other" {
  try {
    const state = lstatSync(path);
    if (state.isSymbolicLink()) return "symlink";
    if (state.isDirectory()) return "directory";
    if (state.isFile()) return "file";
    return "other";
  } catch (error) {
    if (errorCode(error) === "ENOENT") return "missing";
    throw new Error(`Workspace source is unreadable: ${path}`, { cause: error });
  }
}

const NATIVE_DISCOVERY_OPERATIONS: WorkspaceDiscoveryOperations = {
  list: (path) => {
    try {
      return readdirSync(path, { withFileTypes: true }).map((entry) => ({
        kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other",
        name: entry.name,
      }));
    } catch (error) {
      if (errorCode(error) === "ENOENT") return [];
      throw new Error(`Workspace directory is unreadable: ${path}`, { cause: error });
    }
  },
  readText: (path) => {
    try {
      return readFileSync(path, "utf8");
    } catch (error) {
      throw new Error(`Workspace manifest is unreadable: ${path}`, { cause: error });
    }
  },
  state: nativeState,
};

function checkCancellation(options: WorkspaceDiscoveryOptions): void {
  if (options.signal?.aborted) throw new Error("Workspace discovery cancelled");
}

function readManifest(manifestPath: string, operations: WorkspaceDiscoveryOperations): { name?: string; exports?: unknown } {
  const state = operations.state(manifestPath);
  if (state === "missing") return {};
  if (state !== "file") throw new Error(`Workspace manifest must be a regular file: ${manifestPath} (${state})`);
  const source = operations.readText(manifestPath);
  let document: unknown;
  try {
    document = JSON.parse(source);
  } catch (error) {
    throw new Error(`Workspace manifest is malformed: ${manifestPath}`, { cause: error });
  }
  if (!document || typeof document !== "object" || Array.isArray(document)) throw new Error(`Workspace manifest must contain an object: ${manifestPath}`);
  const manifest = document as { name?: unknown; exports?: unknown };
  return { name: typeof manifest.name === "string" ? manifest.name : undefined, exports: manifest.exports };
}

/** 🗺️ Discovers physical package manifests without language or output-directory assumptions. */
function walk(absDir: string, repoRoot: string, results: WorkspaceCandidate[], options: WorkspaceDiscoveryOptions, operations: WorkspaceDiscoveryOperations, progress: { directoriesScanned: number }): void {
  checkCancellation(options);
  const entries = operations.list(absDir);
  progress.directoriesScanned += 1;
  options.onProgress?.({ candidatesDiscovered: results.length, directoriesScanned: progress.directoriesScanned, relativeDirectory: relative(repoRoot, absDir).replaceAll("\\", "/") });
  checkCancellation(options);
  for (const entry of entries) {
    checkCancellation(options);
    if (entry.kind !== "directory" || entry.name.startsWith(".") || WORKSPACE_SCAN_SKIP_DIR_NAMES.has(entry.name)) continue;
    const absChild = join(absDir, entry.name);
    const manifestPath = join(absChild, MANIFEST_FILENAME);
    if (operations.state(manifestPath) !== "missing") {
      results.push({ relDir: relative(repoRoot, absChild).replaceAll("\\", "/"), absDir: absChild, ...readManifest(manifestPath, operations) });
    }
    walk(absChild, repoRoot, results, options, operations, progress);
  }
}

/** 📦️ Enumerates explicit export targets across package subpaths and conditions. */
function exportTargets(value: unknown, subpaths = true): string[] {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return value.flatMap((entry) => exportTargets(entry, false));
  if (!value || typeof value !== "object") return [];
  const entries = Object.entries(value);
  if (entries.some(([key]) => key.startsWith("."))) {
    if (!subpaths || entries.some(([key]) => key !== "." && (!key.startsWith("./") || key.includes("*")))) return [];
    return entries.flatMap(([, entry]) => exportTargets(entry, false));
  }
  if (entries.some(([key]) => !key || /^\d+$/u.test(key))) return [];
  const targets: string[] = [];
  for (const [condition, entry] of entries) {
    targets.push(...exportTargets(entry, false));
    if (condition === "default") break;
  }
  return targets;
}

/** 🔗️ Binds a payload to its nearest package owner through a concrete physical export. */
function ownsPayload(owner: WorkspaceCandidate, payload: WorkspaceCandidate, operations: WorkspaceDiscoveryOperations): boolean {
  if (!owner.name || owner.name !== payload.name) return false;
  const prefix = relative(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
  return exportTargets(owner.exports).some((target) => {
    if (!target.startsWith("./") || /[\\:*?%#\u0000]/u.test(target)) return false;
    const segments = target.slice(2).split("/");
    if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment === "node_modules")) return false;
    if (!segments.join("/").startsWith(prefix)) return false;
    let path = owner.absDir;
    return segments.every((segment, index) => {
      path = join(path, segment);
      return operations.state(path) === (index === segments.length - 1 ? "file" : "directory");
    });
  });
}

//#endregion 🔍️Scan

//#region 🏗️Generate
/** 🏗️ Emits each independent package once and rejects unbound duplicate identities. */
export function computeWorkspaces(repoRoot: string, options: WorkspaceDiscoveryOptions = {}): string[] {
  const operations = options.operations ?? NATIVE_DISCOVERY_OPERATIONS;
  const rootState = operations.state(repoRoot);
  if (rootState !== "directory") throw new Error(`Workspace root must be a physical directory: ${repoRoot} (${rootState})`);
  const candidates: WorkspaceCandidate[] = [];
  walk(repoRoot, repoRoot, candidates, options, operations, { directoriesScanned: 0 });
  const byDirectory = new Map(candidates.map((candidate) => [candidate.absDir, candidate]));
  const results = candidates.filter((candidate) => {
    checkCancellation(options);
    let parent = dirname(candidate.absDir);
    while (parent !== repoRoot && parent !== dirname(parent)) {
      const owner = byDirectory.get(parent);
      if (owner) return !ownsPayload(owner, candidate, operations);
      parent = dirname(parent);
    }
    return true;
  });

  const dirByName = new Map<string, string>();
  for (const { relDir, name } of results) {
    if (!name) continue;
    const existing = dirByName.get(name);
    if (existing && existing !== relDir) {
      throw new Error(`Workspace discovery: duplicate package name "${name}" at both "${existing}" and "${relDir}" — bun install would not resolve this unambiguously.`);
    }
    dirByName.set(name, relDir);
  }

  return results.map((r) => r.relDir).sort((a, b) => a.localeCompare(b));
}

/** 🔎️ Diagnostic split for `--check`: entries `computeWorkspaces` wants that root `package.json` is
 * missing, and entries root `package.json` still lists that no longer resolve to a real package. */
export function diffWorkspaces(repoRoot: string, current: readonly string[], options: WorkspaceDiscoveryOptions = {}): { readonly expected: readonly string[]; readonly missing: readonly string[]; readonly stale: readonly string[] } {
  const expected = computeWorkspaces(repoRoot, options);
  const expectedSet = new Set(expected);
  const currentSet = new Set(current);
  return {
    expected,
    missing: expected.filter((entry) => !currentSet.has(entry)),
    stale: current.filter((entry) => !expectedSet.has(entry)),
  };
}
//#endregion 🏗️Generate
