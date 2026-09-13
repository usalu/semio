import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export const ACTIVATION_RECEIPT_FILE = "🔣️receipt.json";
export const PLAYGROUND_SESSION_ARTIFACT_KEY = "🎮️playground-session/🟦️.ts";
export const PLAYGROUND_SESSION_OUTPUT_ROOT_ENV = "SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT";
export const PLAYGROUND_SESSION_VITE_SPECIFIER = "virtual:semio-playground-session";

/** 🎮️ Resolves one semantic session source below an explicit generated-output root. */
export function playgroundSessionOutputPath(outputRoot: string): string {
  return join(outputRoot, ...PLAYGROUND_SESSION_ARTIFACT_KEY.split("/"));
}

/** 🎮️ Resolves one variant source below an explicit staging root. */
export function playgroundSessionStagedOutputPath(stagingRoot: string, variant: string): string {
  return playgroundSessionOutputPath(join(stagingRoot, variant));
}

/** 🎮️ Gives Vite and native tests the same pure virtual-session alias. */
export function playgroundSessionViteAlias(stagingRoot: string, variant: string): { readonly find: string; readonly replacement: string } {
  return { find: PLAYGROUND_SESSION_VITE_SPECIFIER, replacement: playgroundSessionStagedOutputPath(stagingRoot, variant) };
}

export type ActivationArtifact = { readonly pluginId: string; readonly artifactSha256: string };
export type ActivationReceipt = {
  readonly schema: "semio.dev.activation/v1";
  readonly variant: string;
  readonly profile: "dev" | "release";
  readonly plugins: readonly (ActivationArtifact & { readonly rebuiltAt: number })[];
};

const identity = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const keys = (value: unknown, expected: readonly string[]): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === [...expected].sort().join();

/** 🧾️ Validates the completed runtime receipt defined by {@link ./🧬️schema/🔣️.json} `#/$defs/DevActivationV1`. */
export function parseActivationReceipt(value: unknown): ActivationReceipt {
  if (!keys(value, ["schema", "variant", "profile", "plugins"]) || value.schema !== "semio.dev.activation/v1" || typeof value.variant !== "string" || !identity.test(value.variant) || !["dev", "release"].includes(String(value.profile)) || !Array.isArray(value.plugins)) throw new Error("Invalid activation receipt");
  const seen = new Set<string>();
  for (const row of value.plugins) {
    if (!keys(row, ["pluginId", "artifactSha256", "rebuiltAt"]) || typeof row.pluginId !== "string" || !identity.test(row.pluginId) || typeof row.artifactSha256 !== "string" || !/^[a-f0-9]{64}$/.test(row.artifactSha256) || !Number.isSafeInteger(row.rebuiltAt) || Number(row.rebuiltAt) < 1) throw new Error("Invalid activation plugin");
    if (seen.has(row.pluginId)) throw new Error(`Duplicate activation plugin: ${row.pluginId}`);
    seen.add(row.pluginId);
  }
  return value as unknown as ActivationReceipt;
}

/** 🕰️ Keeps warm activations unchanged and advances changed content despite clock rollback. */
export function nextActivationReceipt(variant: string, profile: "dev" | "release", completed: readonly ActivationArtifact[], previous?: ActivationReceipt, now = Date.now()): ActivationReceipt {
  if (previous) parseActivationReceipt(previous);
  if (previous && (previous.variant !== variant || previous.profile !== profile)) throw new Error("Activation receipt identity mismatch");
  if (!Number.isSafeInteger(now)) throw new Error("Invalid activation clock");
  const prior = new Map(previous?.plugins.map((row) => [row.pluginId, row]));
  const timestamp = Math.max(now, 1, ...[...prior.values()].map((row) => row.rebuiltAt + 1));
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile, plugins: [...completed].sort((a, b) => a.pluginId < b.pluginId ? -1 : a.pluginId > b.pluginId ? 1 : 0).map((row) => ({ ...row, rebuiltAt: prior.get(row.pluginId)?.artifactSha256 === row.artifactSha256 ? prior.get(row.pluginId)!.rebuiltAt : timestamp })) });
}

/** 📖️ Reads only explicit activation completion, never the existence or mtime of cached outputs. */
export function readActivationReceipt(directory: string): ActivationReceipt {
  return parseActivationReceipt(JSON.parse(readFileSync(join(directory, ACTIVATION_RECEIPT_FILE), "utf8")));
}

/** 🗂️ Separates each renderer, variant and profile's activation and installations from compiled artifacts. */
export function developmentRuntimeRoot(packageRoot: string, variant: string, profile: "dev" | "release", renderer: "react" | "wgpu"): string {
  if (!["react", "wgpu"].includes(renderer)) throw new Error("Select a development renderer: react or wgpu");
  parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile, plugins: [] });
  return join(packageRoot, "dist", "runtime", renderer, profile, variant);
}

/** 🔌️ THE staging root for a profile — the ONE directory every producer writes and every consumer
 * reads: `@semio-tech/framework-plugin-web:support-<profile>`, every crate's `materialize-<profile>`
 * and `@semio-tech/framework-os-dev:plugin` write it; the react Vite `/🔌️plugin-modules` mount, the
 * WGPU browser host, the native runner's `SEMIO_PLUGIN_MODULES`, `prepare`/`activate`
 * and the production distribution copy read it. Derived from this module's own location — never from a
 * workspace walk — so Vite's config bundler resolves it without pulling repository discovery in, and so
 * no caller can pick a second root. Two roots is what this function replaces: a `🧑‍💻dev/🔌️plugin-modules`
 * written only by the catalog builder while `materialize-*` wrote here drifted silently for two days
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-playground-boot-2026-09-12.md` §2.1). */
export function pluginModulesRoot(profile: "dev" | "release"): string {
  return pluginModulesRootIn(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../.."), profile);
}

/** 🗂️ The same staging root inside an EXPLICIT workspace — the one form a sandboxed consumer (the
 * production distribution copy, driven against a throwaway workspace in its own tests) may use. The
 * repository-relative path lives here once so no caller ever spells a second one. */
export function pluginModulesRootIn(workspace: string, profile: "dev" | "release"): string {
  if (profile !== "dev" && profile !== "release") throw new Error("Select a plugin staging profile: dev or release");
  return join(workspace, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "📦️packages", "🟦️typescript", "dist", profile, "🔌️plugin-modules");
}

/** 📬️ Atomically announces completed preparation without rewriting a warm receipt. */
export function publishActivationReceipt(directory: string, receipt: ActivationReceipt): boolean {
  const text = JSON.stringify(parseActivationReceipt(receipt)) + "\n", destination = join(directory, ACTIVATION_RECEIPT_FILE);
  if (existsSync(destination)) {
    if (lstatSync(destination).isSymbolicLink()) throw new Error("Invalid activation receipt path");
    const previous = readActivationReceipt(directory);
    if (previous.variant !== receipt.variant || previous.profile !== receipt.profile) throw new Error("Activation receipt identity mismatch");
    if (JSON.stringify(previous) + "\n" === text) return false;
  }
  mkdirSync(directory, { recursive: true });
  const temporary = join(directory, `.receipt-${randomUUID()}.stage`);
  try { writeFileSync(temporary, text, { flag: "wx" }); renameSync(temporary, destination); }
  finally { rmSync(temporary, { force: true }); }
  return true;
}

/** 👀️ Observes atomic completion receipts and owns its filesystem subscription. */
export function observeActivationReceipts(directory: string, listener: (receipt: ActivationReceipt) => void, onError: (error: unknown) => void): { snapshot: () => ActivationReceipt; close: () => void } {
  let current: ActivationReceipt, serialized: string | undefined, closed = false;
  const refresh = (): void => {
    if (closed) return;
    const next = readActivationReceipt(directory), text = JSON.stringify(next);
    if (serialized === text) return;
    current = next;
    serialized = text;
    listener(next);
  };
  const watcher = watch(directory, () => {
    try { refresh(); } catch (error) { onError(error); }
  });
  watcher.on("error", onError);
  const close = (): void => { if (!closed) { closed = true; watcher.close(); } };
  try { refresh(); } catch (error) { close(); throw error; }
  return { snapshot: () => current, close };
}

//#region 🔖️StagedModuleFreshness
/** 🗑️ Directory names inside a component's owner tree that hold BUILD OUTPUT, never the sources whose
 * mtime decides whether the staged module is behind. Walking them would make every crate permanently
 * "stale" the moment its own `dist/component-dev/*.wasm` lands. */
export const UNWATCHED_COMPONENT_SOURCE_DIRECTORIES: readonly string[] = ["dist", "target", "node_modules", "pkg", ".git"];

/** 🔒️ Bound on one component's source walk so a serve-start freshness pass over ~20 crates stays a
 * few milliseconds and can never be turned into an unbounded repository scan by a stray symlink. */
export const COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20_000;

export type StagedModuleFacts = Readonly<{
  pluginId: string;
  role: "plugin" | "extension";
  /** 🧾️ `true` for a lane governed by an activation receipt, `false` for one that
   * reads the staging root directly without a receipt — the receipt-derived
   * verdicts are simply not askable there, and inventing an empty receipt would report every extension
   * as unpublished. */
  activationTracked: boolean;
  stagedAtMs?: number;
  newestSourceMs?: number;
  newestSourcePath?: string;
  receiptArtifactSha256?: string;
  installedPackageHash?: string;
}>;

export type StagedModuleVerdict = Readonly<{
  pluginId: string;
  kind: "fresh" | "unstaged" | "unactivated" | "unpublished" | "source-newer";
  detail?: string;
}>;

/** 🕰️ Formats an epoch millisecond for a `[stale]` line — UTC ISO so two machines print the same text. */
function stagedInstant(value: number): string {
  return new Date(Math.round(value)).toISOString();
}

/** 🔎️ Decides one staged component's freshness from already-collected facts — pure, so the serve-start
 * pass and the activation-receipt watcher share ONE rule and a fixture can drive every outcome.
 * Precedence is most-fundamental-first: nothing staged beats no receipt row, which beats an extension
 * that was materialized but never published, which beats sources newer than the staged bytes. */
export function stagedModuleVerdict(facts: StagedModuleFacts): StagedModuleVerdict {
  if (facts.stagedAtMs === undefined) return { pluginId: facts.pluginId, kind: "unstaged", detail: "no staged module directory" };
  if (facts.activationTracked) {
    if (facts.receiptArtifactSha256 === undefined) return { pluginId: facts.pluginId, kind: "unactivated", detail: `staged ${stagedInstant(facts.stagedAtMs)} but absent from the activation receipt` };
    if (facts.role === "extension" && facts.installedPackageHash !== facts.receiptArtifactSha256) {
      return { pluginId: facts.pluginId, kind: "unpublished", detail: `installed ${facts.installedPackageHash ?? "(nothing)"} ≠ activated ${facts.receiptArtifactSha256}` };
    }
  }
  if (facts.newestSourceMs !== undefined && facts.newestSourceMs > facts.stagedAtMs) {
    return { pluginId: facts.pluginId, kind: "source-newer", detail: `staged ${stagedInstant(facts.stagedAtMs)} < ${facts.newestSourcePath ?? "source"} ${stagedInstant(facts.newestSourceMs)}` };
  }
  return { pluginId: facts.pluginId, kind: "fresh" };
}

/** 📣️ Renders one `[stale]` line per non-fresh component, each ending in the exact command that fixes
 * it — a served module that is behind its own crate must never be a silent no-op. */
export function stagedModuleReportLines(verdicts: readonly StagedModuleVerdict[], command: string): readonly string[] {
  return verdicts.filter((row) => row.kind !== "fresh").map((row) => `[stale] ${row.pluginId}: ${row.kind}${row.detail ? ` — ${row.detail}` : ""} — run: ${command}`);
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
//#endregion 🔖️StagedModuleFreshness
