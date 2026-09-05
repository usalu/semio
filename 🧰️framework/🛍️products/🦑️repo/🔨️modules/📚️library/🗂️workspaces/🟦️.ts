//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: generates root `package.json`'s bun `workspaces` array from a real
// on-disk scan for every `package.json`-carrying directory (Shape V1 `⚡️implementations/<lang>` and
// Shape V2 `📦️packages/<lang>` alike), replacing the ~68 hand-maintained literal globs that were
// already out of sync (~40 math npm wrapper packages were resolving via nx only, invisible to bun).
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/GENERATED-BUN-WORKSPACES-FROM-PACKAGE-CATALOG
//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
//#endregion 🔌️Adapters

//#region 🔎️WorkspaceRoot
/** 🔎️Resolves monorepo root (directory containing root package.json named `workspace`). Owned here
 * rather than in the repository library barrel so a consumer that only needs the root path never pulls
 * the barrel (and its taxonomy discovery walk) into its module graph. */
export function getWorkspaceRoot(): string {
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
const CARGO_MANIFEST_FILENAME = "Cargo.toml";

/** 🧺️ Directory names never descended into — build/vendor/scratch trees, never real workspace source.
 * Includes the schema-owned opaque `compose` boundary (same isolation as `DISCOVERY_SKIP_DIRS`) so
 * workspace generation cannot reintroduce its intentionally deleted memberships. */
const WORKSPACE_SCAN_SKIP_DIR_NAMES = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "storybook-static", "temp", "coverage", "🔌️plugin-modules", ".🧬semio", "compose"]);

/** 🧻️ wasm-pack's generated npm-wrapper dir name — gitignored, present only once built, handled
 * specially by `resolvePkgDir` and never generically recursed into (see its docstring for why). */
const WASM_PKG_DIR_NAME = "pkg";
//#endregion 🔣️Constants

//#region 🔍️Scan
/** 📦️ One directory discovered to carry its own `package.json`, with the manifest's `name` (if any). */
interface WorkspaceCandidate {
  readonly relDir: string;
  readonly absDir: string;
  readonly name?: string;
}

/** 🧻️ A `pkg/` dir queued for the special resolution pass in `computeWorkspaces` — see `resolvePkgDir`. */
interface PkgDirCandidate {
  readonly pkgAbsDir: string;
  readonly parentAbsDir: string;
}

function readManifestName(manifestPath: string): string | undefined {
  try {
    return (JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: string }).name;
  } catch {
    return undefined;
  }
}

/** 📁️ `readdirSync(dir, { withFileTypes: true })`, defaulting to `[]` for an unreadable/missing dir. */
function readdirSafe(absDir: string) {
  try {
    return readdirSync(absDir, { withFileTypes: true });
  } catch {
    return [];
  }
}

/**
 * 🗺️ ONE walk collecting every real package directory, at whatever depth it sits. Deliberately does
 * NOT reuse `discoverPackages()` — that function only sees Shape V2's `📦️packages/<lang>` contract, but
 * bun's `workspaces` array must resolve EVERY real npm package regardless of migration state (most of
 * the repo is still Shape V1 `⚡️implementations/<lang>` as of this writing), so a shape-agnostic
 * "does this dir have its own `package.json`" walk is root package.json's actual source of truth.
 * `pkg/` dirs are queued into `pkgCandidates` instead of resolved inline — see `resolvePkgDir`.
 */
function walk(absDir: string, repoRoot: string, pkgCandidates: PkgDirCandidate[], results: WorkspaceCandidate[]): void {
  for (const entry of readdirSafe(absDir)) {
    if (!entry.isDirectory() || entry.name.startsWith(".") || WORKSPACE_SCAN_SKIP_DIR_NAMES.has(entry.name)) continue;
    const absChild = join(absDir, entry.name);
    if (entry.name === WASM_PKG_DIR_NAME) {
      pkgCandidates.push({ pkgAbsDir: absChild, parentAbsDir: absDir });
      continue; // 🧻️ never generically recursed — a broken wasm-pack run can nest arbitrary junk inside
    }
    const manifestPath = join(absChild, MANIFEST_FILENAME);
    if (existsSync(manifestPath)) {
      results.push({ relDir: relative(repoRoot, absChild).replaceAll("\\", "/"), absDir: absChild, name: readManifestName(manifestPath) });
    }
    walk(absChild, repoRoot, pkgCandidates, results);
  }
}

/**
 * 🧻️ Decides whether a `pkg/` dir (wasm-pack's generated npm wrapper, gitignored, present only once
 * built) should become its own workspace entry. Auditing the real repo found two hazards a naive
 * "any `package.json` counts" rule would hit:
 * 1. A misplaced/broken wasm-pack invocation can leave a `pkg/` dir with no sibling `Cargo.toml` (found
 *    once, under 🌊️flow's `🫀️core` — a stray `pkg/⚡️implementation/🦀️rust` re-emission alongside the
 *    real `📦️packages/🦀️rust/pkg`). Such a dir is never a real package — skipped outright, and
 *    never descended into (see `walk`), so nothing inside it is ever considered either.
 * 2. Most wasm crates' checked-in outer wrapper `package.json` already re-exports the built `pkg/*.js`
 *    under the SAME package name (e.g. both `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/`
 *    and its `pkg/` declare `@semio-tech/framework-editor-rs`) — listing the nested copy too would be a
 *    duplicate-name workspace. It is only included when its name genuinely differs from the outer
 *    wrapper's (or the outer dir has no wrapper at all — e.g. 🌊️flow's dynamically-loaded
 *    `flow-extension-bim`, whose `pkg/` is its only manifest and has no `workspace:*` dependent at all,
 *    since it is loaded by path at runtime, not imported by name — deliberately NOT gated on real
 *    `workspace:*` usage for exactly that reason, confirmed by a real `bun install` requiring any listed
 *    workspace dir to exist on disk, never on being depended upon).
 */
function resolvePkgDir(pkgAbsDir: string, parentAbsDir: string): string | undefined {
  const manifestPath = join(pkgAbsDir, MANIFEST_FILENAME);
  if (!existsSync(manifestPath)) return undefined; // e.g. a `--target web` wasm-pack build with no npm wrapper
  if (!existsSync(join(parentAbsDir, CARGO_MANIFEST_FILENAME))) return undefined; // hazard 1
  const pkgName = readManifestName(manifestPath);
  if (!pkgName) return undefined;
  const parentManifestPath = join(parentAbsDir, MANIFEST_FILENAME);
  const parentName = existsSync(parentManifestPath) ? readManifestName(parentManifestPath) : undefined;
  if (pkgName === parentName) return undefined; // hazard 2: shadowed by the outer wrapper's re-export
  return manifestPath;
}
//#endregion 🔍️Scan

//#region 🏗️Generate
/**
 * 🏗️ Computes root `package.json`'s `workspaces` array by walking the real repo tree for every
 * directory carrying its own `package.json` — see `walk`. Nothing is hand-listed, so nothing can drift
 * again: as areas migrate from Shape V1 to Shape V2 (or gain/lose packages), the next `--write` simply
 * reflects it. Throws if two discovered packages declare the identical `name` (`bun install` would not
 * be able to tell them apart) — a real collision is a genuine problem worth failing loudly on rather
 * than silently emitting a broken array (see `resolvePkgDir` for the one near-miss this already avoids).
 */
export function computeWorkspaces(repoRoot: string): string[] {
  const pkgCandidates: PkgDirCandidate[] = [];
  const results: WorkspaceCandidate[] = [];
  walk(repoRoot, repoRoot, pkgCandidates, results);

  for (const { pkgAbsDir, parentAbsDir } of pkgCandidates) {
    const manifestPath = resolvePkgDir(pkgAbsDir, parentAbsDir);
    if (!manifestPath) continue;
    results.push({ relDir: relative(repoRoot, pkgAbsDir).replaceAll("\\", "/"), absDir: pkgAbsDir, name: readManifestName(manifestPath) });
  }

  const dirByName = new Map<string, string>();
  for (const { relDir, name } of results) {
    if (!name) continue;
    const existing = dirByName.get(name);
    if (existing && existing !== relDir) {
      throw new Error(`🗂️workspaces.ts: duplicate package name "${name}" at both "${existing}" and "${relDir}" — bun install would not resolve this unambiguously.`);
    }
    dirByName.set(name, relDir);
  }

  return results.map((r) => r.relDir).sort((a, b) => a.localeCompare(b));
}

/** 🔎️ Diagnostic split for `--check`: entries `computeWorkspaces` wants that root `package.json` is
 * missing, and entries root `package.json` still lists that no longer resolve to a real package. */
export function diffWorkspaces(repoRoot: string, current: readonly string[]): { readonly expected: readonly string[]; readonly missing: readonly string[]; readonly stale: readonly string[] } {
  const expected = computeWorkspaces(repoRoot);
  const expectedSet = new Set(expected);
  const currentSet = new Set(current);
  return {
    expected,
    missing: expected.filter((entry) => !currentSet.has(entry)),
    stale: current.filter((entry) => !expectedSet.has(entry)),
  };
}
//#endregion 🏗️Generate
