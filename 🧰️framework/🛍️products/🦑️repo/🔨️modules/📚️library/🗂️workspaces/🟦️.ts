//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js
// Physical package membership and explicit package payload ownership.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { ownsPayload, NATIVE_DISCOVERY_OPERATIONS, type PackagePayloadOperations } from "./📦️payload/🟦️.ts";
import { bunRepositoryPackages } from "./🟦️bun/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔎️WorkspaceRoot
/** 🔎️Uses Nx's execution workspace before standalone hints and workspace-manifest discovery. Owned here
 * rather than in the repository library barrel so a consumer that only needs the root path never pulls
 * the barrel (and its taxonomy discovery walk) into its module graph. */
export function getWorkspaceRoot(environment: Readonly<Record<string,string|undefined>> = process.env): string {
  const fromNx = environment.NX_WORKSPACE_ROOT?.trim();
  if (fromNx) return resolve(fromNx);
  const fromEnv = environment.REPO_ROOT?.trim();
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

//#region 🗂️Declared
/** 🗂️ The package directories the root `package.json` declares as workspaces (kept exact by
 * `workspaces --check`), repository-relative. A consumer that needs every package manifest reads these
 * instead of walking the repository. */
export function declaredWorkspaces(repoRoot: string): readonly string[] {
  const manifest = JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8")) as { workspaces?: unknown };
  if (!Array.isArray(manifest.workspaces) || manifest.workspaces.some((entry) => typeof entry !== "string")) throw new Error("Root package.json must declare workspaces as a string array.");
  return bunRepositoryPackages(repoRoot);
}
//#endregion 🗂️Declared

//#region 🔣️Constants
const MANIFEST_FILENAME = "package.json";

/** 🧺️ Directory names never descended into — build/vendor/scratch trees, never real workspace source.
 * Includes the schema-owned opaque `compose` boundary (same isolation as `DISCOVERY_SKIP_DIRS`) so
 * workspace generation cannot reintroduce its intentionally deleted memberships. */
const WORKSPACE_SCAN_SKIP_DIR_NAMES = new Set(["🗑️generated", "node_modules", "target", "dist", "build", "🤖️generated", "storybook-static", "temp", "coverage", "🔌️plugin-modules", ".🧬semio", "compose"]);

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

export interface WorkspaceDiscoveryOptions {
  readonly onProgress?: (progress: WorkspaceDiscoveryProgress) => void;
  readonly operations?: PackagePayloadOperations;
  readonly signal?: Pick<AbortSignal, "aborted">;
}

function checkCancellation(options: WorkspaceDiscoveryOptions): void {
  if (options.signal?.aborted) throw new Error("Workspace discovery cancelled");
}

function readManifest(manifestPath: string, operations: PackagePayloadOperations): { name?: string; exports?: unknown } {
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
function walk(absDir: string, repoRoot: string, results: WorkspaceCandidate[], options: WorkspaceDiscoveryOptions, operations: PackagePayloadOperations, progress: { directoriesScanned: number }): void {
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
