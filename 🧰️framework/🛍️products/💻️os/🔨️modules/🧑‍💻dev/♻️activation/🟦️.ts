import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { readdir as readdirAsync, readFile as readFileAsync, stat as statAsync } from "node:fs/promises";
import { createHash, randomUUID } from "node:crypto";
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

export type ActivationArtifact = { readonly pluginId: string; readonly artifactSha256: string; readonly sourceContentSha256?: string };
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
    const rowKeys = Object.keys(row as object).sort().join();
    if ((rowKeys !== "artifactSha256,pluginId,rebuiltAt" && rowKeys !== "artifactSha256,pluginId,rebuiltAt,sourceContentSha256") || typeof (row as any).pluginId !== "string" || !identity.test((row as any).pluginId) || typeof (row as any).artifactSha256 !== "string" || !/^[a-f0-9]{64}$/.test((row as any).artifactSha256) || !Number.isSafeInteger((row as any).rebuiltAt) || Number((row as any).rebuiltAt) < 1 || ((row as any).sourceContentSha256 !== undefined && (typeof (row as any).sourceContentSha256 !== "string" || !/^[a-f0-9]{64}$/.test((row as any).sourceContentSha256)))) throw new Error("Invalid activation plugin");
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
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile, plugins: [...completed].sort((a, b) => a.pluginId < b.pluginId ? -1 : a.pluginId > b.pluginId ? 1 : 0).map((row) => {
    const previous = prior.get(row.pluginId);
    const same = previous?.artifactSha256 === row.artifactSha256 && previous?.sourceContentSha256 === row.sourceContentSha256;
    const plugin: Record<string, unknown> = { pluginId: row.pluginId, artifactSha256: row.artifactSha256, rebuiltAt: same ? previous!.rebuiltAt : timestamp };
    if (row.sourceContentSha256) plugin.sourceContentSha256 = row.sourceContentSha256;
    return plugin;
  }) });
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

//#region 🩺️HealthySet
/** 🩺️ What one session component's staged module directory actually holds, as read off disk by the
 * caller — kept a plain record so the rule below is pure and a fixture can drive every outcome. */
export type PreparedComponentFacts = Readonly<{
  pluginId: string;
  /** 🧱️ `false` when the component's own `component-<profile>`/`materialize-<profile>` never produced
   * a staging directory at all — the shape a crate that failed to compile leaves behind. */
  directoryPresent: boolean;
  /** 🔣️ `pluginId` the staged `🔣️.json` names, `undefined` when that descriptor is absent or unreadable. */
  descriptorPluginId?: string;
  bridgePresent: boolean;
  artifactMarkerPresent: boolean;
}>;

/** 🩺️ One component's prepared-staging verdict. `unstaged` and `incomplete` are different facts: the
 * first means nothing was produced, the second means something was produced and does not hold together. */
export type PreparedComponentVerdict = Readonly<{
  pluginId: string;
  kind: "prepared" | "unstaged" | "incomplete";
  detail?: string;
}>;

/** 🩺️ Decides one staged component's prepared-ness from already-collected facts — pure, so the
 * preparation pass and the activation receipt share ONE rule. Precedence is most-fundamental-first:
 * nothing staged beats a descriptor naming a different plugin, which beats a missing bridge or
 * Nx artifact marker. */
export function preparedComponentVerdict(facts: PreparedComponentFacts): PreparedComponentVerdict {
  if (!facts.directoryPresent) return { pluginId: facts.pluginId, kind: "unstaged", detail: "no staged module directory" };
  if (facts.descriptorPluginId === undefined) return { pluginId: facts.pluginId, kind: "incomplete", detail: "no readable 🔣️.json descriptor" };
  if (facts.descriptorPluginId !== facts.pluginId) return { pluginId: facts.pluginId, kind: "incomplete", detail: `descriptor names ${facts.descriptorPluginId}` };
  if (!facts.bridgePresent) return { pluginId: facts.pluginId, kind: "incomplete", detail: "no module bridge" };
  if (!facts.artifactMarkerPresent) return { pluginId: facts.pluginId, kind: "incomplete", detail: "no .nx-artifact.json marker" };
  return { pluginId: facts.pluginId, kind: "prepared" };
}

/** 🩺️ The HEALTHY SET rule for a multi-plugin host: one component that failed to build is a missing
 * component, not a broken product. `s` fans its Nx closure out to the entire registered catalog, so an
 * all-or-nothing preparation makes the whole hub un-bootable whenever any single one of ~60 crates is
 * red — which is exactly how every cold `dev s` attempt before ticket 26/09/18 ended (`🧱️block`).
 *
 * The exclusion is explicit and principled rather than a swallowed error: every excluded component is
 * NAMED with its verdict, it is dropped from the activation receipt (so the serve-start freshness pass
 * reports it `unactivated` and the shell's own per-plugin install isolation refuses it at open time
 * instead of pretending it is there), and the two components that genuinely cannot be missing still
 * refuse the boot — the variant's OWN host plugin, and the degenerate case where nothing prepared at
 * all. Anything else is reported and excluded. */
export function healthyPreparedComponents(verdicts: readonly PreparedComponentVerdict[], hostPluginId: string): {
  readonly prepared: readonly string[];
  readonly excluded: readonly PreparedComponentVerdict[];
  readonly refusal?: string;
} {
  const prepared = verdicts.filter((row) => row.kind === "prepared").map((row) => row.pluginId);
  const excluded = verdicts.filter((row) => row.kind !== "prepared");
  const host = excluded.find((row) => row.pluginId === hostPluginId);
  if (host) return { prepared, excluded, refusal: `Host component ${hostPluginId} is ${host.kind}${host.detail ? ` — ${host.detail}` : ""}` };
  if (prepared.length === 0) return { prepared, excluded, refusal: `No component of ${verdicts.length} prepared` };
  return { prepared, excluded };
}

/** 📣️ Renders one `[excluded]` line per component the healthy set drops, each ending in the exact
 * command that would bring it back — a component silently missing from a 60-plugin hub is unreadable.
 * Same line shape as {@link stagedModuleReportLines} so one boot capture reads as one report. */
export function preparedComponentReportLines(excluded: readonly PreparedComponentVerdict[], command: string): readonly string[] {
  return excluded.map((row) => `[excluded] ${row.pluginId}: ${row.kind}${row.detail ? ` — ${row.detail}` : ""} — run: ${command}`);
}
//#endregion 🩺️HealthySet

//#region 🔖️StagedModuleFreshness
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
  /** 🔖️ SHA-256 of the plugin-owner source tree (plugin paths only — framework changes never enter). */
  sourceContentSha256?: string;
  /** 🔖️ Source content hash recorded when the staged module / receipt row was published. */
  stagedSourceContentSha256?: string;
  receiptArtifactSha256?: string;
  installedPackageHash?: string;
}>;

export type StagedModuleVerdict = Readonly<{
  pluginId: string;
  kind: "fresh" | "unstaged" | "unactivated" | "unpublished" | "source-changed";
  detail?: string;
}>;

/** 🕰️ Formats an epoch millisecond for a `[stale]` line — UTC ISO so two machines print the same text. */
function stagedInstant(value: number): string {
  return new Date(Math.round(value)).toISOString();
}

/** 🔎️ Decides one staged component's freshness from already-collected facts — pure, so the serve-start
 * pass and the activation-receipt watcher share ONE rule and a fixture can drive every outcome.
 * Precedence is most-fundamental-first: nothing staged beats no receipt row, which beats an extension
 * that was materialized but never published, which beats a plugin-owner content-hash mismatch.
 * Mtime never decides freshness (framework clocks and restages would false-stale). */
export function stagedModuleVerdict(facts: StagedModuleFacts): StagedModuleVerdict {
  if (facts.stagedAtMs === undefined) return { pluginId: facts.pluginId, kind: "unstaged", detail: "no staged module directory" };
  if (facts.activationTracked) {
    if (facts.receiptArtifactSha256 === undefined) return { pluginId: facts.pluginId, kind: "unactivated", detail: `staged ${stagedInstant(facts.stagedAtMs)} but absent from the activation receipt` };
    if (facts.role === "extension" && facts.installedPackageHash !== facts.receiptArtifactSha256) {
      return { pluginId: facts.pluginId, kind: "unpublished", detail: `installed ${facts.installedPackageHash ?? "(nothing)"} ≠ activated ${facts.receiptArtifactSha256}` };
    }
  }
  if (facts.sourceContentSha256 && facts.stagedSourceContentSha256) {
    if (facts.sourceContentSha256 !== facts.stagedSourceContentSha256) {
      return { pluginId: facts.pluginId, kind: "source-changed", detail: `source ${facts.sourceContentSha256.slice(0, 12)}… ≠ staged ${facts.stagedSourceContentSha256.slice(0, 12)}…` };
    }
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
export function componentSourceContentHash(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): string {
  return buildComponentSourceStatIndex(sourceRoot, maximumEntries).contentSha256;
}


/** 🔖 Sibling marker next to a staged module recording the plugin-source content hash used to build it. */
export const STAGED_SOURCE_CONTENT_HASH_FILE = ".source-content-sha256";

/** 🔖 Sibling marker next to a staged module recording the per-file source stat index used for boot freshness. */
export const STAGED_SOURCE_STAT_INDEX_FILE = ".source-stat-index.json";

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

/** 🕰 Lists plugin-owner source files under one root (same bounds as content hashing). */
export function listComponentSourceFiles(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): readonly string[] {
  if (!existsSync(sourceRoot)) return [];
  let visited = 0;
  const pending = [sourceRoot];
  const files: string[] = [];
  while (pending.length > 0) {
    const directory = pending.pop()!;
    for (const entry of readableDirectoryEntries(directory)) {
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

export async function listComponentSourceFilesAsync(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): Promise<readonly string[]> {
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

/** 🔖 Builds the per-file source stat index and aggregate content hash for one plugin owner tree. */
export function buildComponentSourceStatIndex(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): SourceStatIndex {
  const files: SourceStatFileEntry[] = [];
  for (const absolute of listComponentSourceFiles(sourceRoot, maximumEntries)) {
    let stats: ReturnType<typeof statSync>;
    let bytes: Buffer;
    try {
      stats = statSync(absolute);
      bytes = readFileSync(absolute);
    } catch { continue; }
    files.push({
      path: relativeSourcePath(sourceRoot, absolute),
      size: stats.size,
      mtimeNs: fileMtimeNs(stats),
      sha256: createHash("sha256").update(bytes).digest("hex"),
    });
  }
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
export function writeStagedSourceFreshness(moduleDirectory: string, sourceRoot: string): string {
  const index = buildComponentSourceStatIndex(sourceRoot);
  writeStagedSourceContentHash(moduleDirectory, index.contentSha256);
  writeStagedSourceStatIndex(moduleDirectory, index);
  return index.contentSha256;
}

/**
 * 🕰 Boot freshness: stat-walk the owner tree, reuse per-file digests when (size, mtimeNs) match
 * the staged index, and rehash only added/changed files. Missing index falls back to a full content hash.
 */
export function resolveBootSourceContentHashes(options: {
  readonly sourceRoot: string;
  readonly moduleDirectory: string;
  readonly receiptSourceContentSha256?: string;
}): BootSourceHashResolution {
  const stagedSourceContentSha256 = readStagedSourceContentHash(options.moduleDirectory) ?? options.receiptSourceContentSha256;
  const index = readStagedSourceStatIndex(options.moduleDirectory);
  if (!index) {
    const built = buildComponentSourceStatIndex(options.sourceRoot);
    let newestSourceMs: number | undefined;
    let newestSourcePath: string | undefined;
    for (const absolute of listComponentSourceFiles(options.sourceRoot)) {
      try {
        const stats = statSync(absolute);
        if (newestSourceMs === undefined || stats.mtimeMs > newestSourceMs) {
          newestSourceMs = stats.mtimeMs;
          newestSourcePath = absolute;
        }
      } catch { continue; }
    }
    return {
      sourceContentSha256: built.contentSha256,
      stagedSourceContentSha256,
      newestSourceMs,
      newestSourcePath,
      hashedFileCount: built.files.length,
      reusedFileCount: 0,
    };
  }
  const prior = new Map(index.files.map((file) => [file.path, file]));
  const absolutes = listComponentSourceFiles(options.sourceRoot);
  const next: SourceStatFileEntry[] = [];
  let hashedFileCount = 0;
  let reusedFileCount = 0;
  let newestSourceMs: number | undefined;
  let newestSourcePath: string | undefined;
  for (const absolute of absolutes) {
    let stats: ReturnType<typeof statSync>;
    try { stats = statSync(absolute); } catch { continue; }
    const relativePath = relativeSourcePath(options.sourceRoot, absolute);
    const mtimeNs = fileMtimeNs(stats);
    if (newestSourceMs === undefined || stats.mtimeMs > newestSourceMs) {
      newestSourceMs = stats.mtimeMs;
      newestSourcePath = absolute;
    }
    const previous = prior.get(relativePath);
    if (previous && previous.size === stats.size && previous.mtimeNs === mtimeNs) {
      next.push(previous);
      reusedFileCount += 1;
      continue;
    }
    let bytes: Buffer;
    try { bytes = readFileSync(absolute); } catch { continue; }
    next.push({
      path: relativePath,
      size: stats.size,
      mtimeNs,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    });
    hashedFileCount += 1;
  }
  return {
    sourceContentSha256: aggregateSourceContentHash(next),
    stagedSourceContentSha256,
    newestSourceMs,
    newestSourcePath,
    hashedFileCount,
    reusedFileCount,
  };
}

/** 🕰 Async boot freshness for one plugin — scheduled so callers can `Promise.all` across plugins. */
export async function resolveBootSourceContentHashesAsync(options: {
  readonly sourceRoot: string;
  readonly moduleDirectory: string;
  readonly receiptSourceContentSha256?: string;
}): Promise<BootSourceHashResolution> {
  const stagedSourceContentSha256 = readStagedSourceContentHash(options.moduleDirectory) ?? options.receiptSourceContentSha256;
  const index = readStagedSourceStatIndex(options.moduleDirectory);
  if (!index) {
    const built = buildComponentSourceStatIndex(options.sourceRoot);
    return {
      sourceContentSha256: built.contentSha256,
      stagedSourceContentSha256,
      hashedFileCount: built.files.length,
      reusedFileCount: 0,
    };
  }
  const prior = new Map(index.files.map((file) => [file.path, file]));
  const absolutes = await listComponentSourceFilesAsync(options.sourceRoot);
  const parts = await Promise.all(absolutes.map(async (absolute) => {
    let stats: Awaited<ReturnType<typeof statAsync>>;
    try { stats = await statAsync(absolute); } catch { return undefined; }
    const relativePath = relativeSourcePath(options.sourceRoot, absolute);
    const mtimeNs = fileMtimeNs(stats);
    const previous = prior.get(relativePath);
    if (previous && previous.size === stats.size && previous.mtimeNs === mtimeNs) {
      return { entry: previous, hashed: false as const, mtimeMs: stats.mtimeMs, absolute };
    }
    let bytes: Buffer;
    try { bytes = Buffer.from(await readFileAsync(absolute)); } catch { return undefined; }
    return {
      entry: { path: relativePath, size: stats.size, mtimeNs, sha256: createHash("sha256").update(bytes).digest("hex") },
      hashed: true as const,
      mtimeMs: stats.mtimeMs,
      absolute,
    };
  }));
  const next: SourceStatFileEntry[] = [];
  let hashedFileCount = 0;
  let reusedFileCount = 0;
  let newestSourceMs: number | undefined;
  let newestSourcePath: string | undefined;
  for (const part of parts) {
    if (!part) continue;
    next.push(part.entry);
    if (part.hashed) hashedFileCount += 1; else reusedFileCount += 1;
    if (newestSourceMs === undefined || part.mtimeMs > newestSourceMs) {
      newestSourceMs = part.mtimeMs;
      newestSourcePath = part.absolute;
    }
  }
  return {
    sourceContentSha256: aggregateSourceContentHash(next),
    stagedSourceContentSha256,
    newestSourceMs,
    newestSourcePath,
    hashedFileCount,
    reusedFileCount,
  };
}

//#endregion 🔖️StagedModuleFreshness
