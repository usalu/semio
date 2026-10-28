import { readFile } from "node:fs/promises";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { tmpdir } from "node:os";
import { setImmediate as yieldEventLoop } from "node:timers/promises";
import { withCurrentBunWorkspaceOwnershipV1 } from "../../🗂️workspaces/🟦️bun/🟦️.ts";
import { ownershipImplementationDigestV1, ownershipMemoAliasesV1, ownershipMemoCurrentV1, ownershipMemoPathV1, readOwnershipMemoV1, sealOwnershipMemoV1, writeOwnershipMemoV1 } from "./🧠️memo/🟦️.ts";

/** 📦️ One exact public runtime export selected from an authored package manifest. */
export interface PublicRuntimePackageExportV1 { readonly specifier: string; readonly target: string }
/** 🧭️ An exact module alias without exporting build-tool implementation types. */
export interface WorkspaceTestAliasV1 { readonly find: RegExp; readonly replacement: string }
/** 📣️ Reports genuine source ownership operations and selected package export reads. */
export interface WorkspaceTestResolutionProgressV1 { readonly operation: string; readonly path: string; readonly completedOperations: number; readonly stage: "ownership" | "exports"; readonly completedPackages?: number; readonly totalPackages?: number }
/** 🎛️ Preserves cancellation and asynchronous progress throughout current workspace ownership and export resolution. `cache` names the directory of the validated memo of the last walk (default: a directory under the operating system's temporary directory; `null` turns the memo off): a later call proves the observed tree unchanged by one stat per observed directory and one digest per manifest instead of walking again, hears the same progress and cancellation, and `report` tells which of the two answered. */
export interface WorkspaceTestResolutionControlV1 { readonly owner?: string; readonly signal?: AbortSignal; readonly cache?: string | null; readonly settleMilliseconds?: number; readonly advance?: (progress: WorkspaceTestResolutionProgressV1) => void | Promise<void>; readonly report?: (row: { readonly source: "memo" | "walk"; readonly operations: number }) => void }

function runtimeTargets(value: unknown): readonly string[] {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return [...new Set(value.flatMap(runtimeTargets))];
  if (value !== null && typeof value === "object") for (const [condition, target] of Object.entries(value)) if (["node", "import", "default"].includes(condition)) return runtimeTargets(target);
  return [];
}

/** 🧪️ Resolves only explicit public runtime package exports for native ESM tests. */
export function publicRuntimePackageExportsV1(manifest: unknown): readonly PublicRuntimePackageExportV1[] {
  if (manifest === null || typeof manifest !== "object" || Array.isArray(manifest)) throw Error("Package manifest required");
  const { name, exports: surface } = manifest as { name?: unknown; exports?: unknown };
  if (typeof name !== "string" || !/^(?:@[^/]+\/)?[^/]+$/u.test(name)) throw Error("Explicit package name required");
  if (surface === undefined || surface === null) return [];
  const keys = typeof surface === "object" && !Array.isArray(surface) ? Object.keys(surface) : [];
  const subpaths = keys.some(key => key.startsWith("."));
  const entries = subpaths ? Object.entries(surface as Record<string, unknown>) : [[".", surface] as const];
  return Object.freeze(entries.flatMap(([key, value]) => {
    if (key !== "." && (!key.startsWith("./") || key.slice(2).split("/").some(part => !part || part === "." || part === ".." || /[*\\:]/u.test(part)))) throw Error("Explicit portable public export key required");
    const target = runtimeTargets(value)[0];
    if (target === undefined) return [];
    if (!target.startsWith("./") || target.slice(2).split("/").some(part => !part || part === "." || part === ".." || part === "node_modules" || /[*\\:]/u.test(part))) throw Error("Portable package-relative runtime export required");
    return [Object.freeze({ specifier: key === "." ? name : name + key.slice(1), target })];
  }));
}

/** 🗂️ Projects current authored workspace public exports into exact test aliases. */
export async function workspaceTestAliasesV1(workspace: string, control: WorkspaceTestResolutionControlV1 = {}): Promise<readonly WorkspaceTestAliasV1[]> {
  const signal = control.signal ?? new AbortController().signal;
  signal.throwIfAborted();
  workspace = resolve(workspace);
  const owner = control.owner ?? "", cacheDirectory = control.cache === undefined ? join(tmpdir(), "semio-workspace-ownership") : control.cache ?? undefined, implementation = cacheDirectory === undefined ? undefined : ownershipImplementationDigestV1(new URL("../../🗂️workspaces/🟦️bun/🟨️.cjs", import.meta.url), new URL("../../🗂️workspaces/🔣️policy.json", import.meta.url));
  const file = cacheDirectory !== undefined && implementation !== undefined ? ownershipMemoPathV1(cacheDirectory, workspace, owner) : undefined;
  if (file !== undefined) {
    const memo = readOwnershipMemoV1(file), advance = control.advance, checks = memo === undefined ? undefined : await ownershipMemoCurrentV1(memo, workspace, implementation!, signal, advance ? (checked, path) => advance({ operation: "state", path, completedOperations: checked, stage: "ownership" }) : undefined);
    if (memo !== undefined && checks !== undefined) { control.report?.({ source: "memo", operations: checks }); return Object.freeze(ownershipMemoAliasesV1(memo, workspace)); }
  }
  const startedAtMs = Date.now();
  let reported = 0, sliced = Number.NEGATIVE_INFINITY, operations = 0;
  const advance = async (progress: WorkspaceTestResolutionProgressV1): Promise<void> => {
    signal.throwIfAborted();
    operations++;
    if (control.advance) await control.advance(progress);
    else if (Date.now() - reported >= 2000) { console.log(`[test-resolution] ${progress.stage}: ${progress.completedOperations} source operations`); reported = Date.now(); }
    signal.throwIfAborted();
    if (performance.now() - sliced >= 20) { await yieldEventLoop(); signal.throwIfAborted(); sliced = performance.now(); }
  };
  const selection = control.owner === undefined ? { kind: "all" as const } : { kind: "scope" as const, owner: control.owner };
  const walked = await withCurrentBunWorkspaceOwnershipV1(workspace, selection, { signal, advance: progress => advance({ ...progress, stage: "ownership" }) }, async custody => {
    const members = [...new Set(custody.scopes.flatMap(scope => scope.packages))], aliases: WorkspaceTestAliasV1[] = [], names = new Set<string>();
    for (const [index, member] of members.entries()) {
      signal.throwIfAborted();
      const root = resolve(workspace, member), local = relative(workspace, root);
      if (!local || isAbsolute(local) || local === ".." || local.startsWith(".." + sep)) throw Error("Workspace package must belong to its declared root");
      const manifest = JSON.parse(await readFile(join(root, "package.json"), { encoding: "utf8", signal }));
      for (const entry of publicRuntimePackageExportsV1(manifest)) {
        if (names.has(entry.specifier)) throw Error("Unique current workspace public export required: " + entry.specifier);
        names.add(entry.specifier);
        aliases.push(Object.freeze({ find: new RegExp("^" + entry.specifier.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&") + "$", "u"), replacement: resolve(root, entry.target) }));
      }
      await advance({ operation: "readText", path: join(root, "package.json"), completedOperations: index + 1, stage: "exports", completedPackages: index + 1, totalPackages: members.length });
    }
    return { aliases: Object.freeze(aliases), observed: custody.observed() };
  }).catch(error => { signal.throwIfAborted(); throw error; });
  if (file !== undefined) {
    const sealed = sealOwnershipMemoV1({ workspace, owner, implementation: implementation!, startedAtMs, ...(control.settleMilliseconds === undefined ? {} : { settleMilliseconds: control.settleMilliseconds }), observed: walked.observed, aliases: walked.aliases });
    if (sealed !== undefined) try { writeOwnershipMemoV1(file, sealed); } catch { }
  }
  control.report?.({ source: "walk", operations });
  return walked.aliases;
}
