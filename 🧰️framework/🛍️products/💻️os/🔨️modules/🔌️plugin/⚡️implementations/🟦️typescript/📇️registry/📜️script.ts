#!/usr/bin/env bun
/**
 * 📜️ `@semio-tech/plugin-registry` — single-source plugin/playground/framework catalog codegen from
 * workspace packages. Discovery is the shared repo-wide contract (`🔣️taxonomy.json` +
 * `discoverPackages()` in `🦑️repo/📚️lib`), not path regexes local to this script; the plugin area's
 * declared `AreaState` decides how much pre-Shape-V2 layout is still tolerated.
 *
 * `generate` writes `🤖️generated/*` plus `.vscode/launch.json` (both derived from the same playground
 * catalog); `check` byte-compares every one of those artifacts and never writes.
 *
 * @see .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/REGISTRY-SCRIPT-REFACTOR-TO-VOCABULARY-DISCOVERY-LIBRARY
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import type { AreaState, DiscoveredPackage, PackageRole } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import { areaOf, BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain, loadTaxonomy, discoverPackages, discoverPackageProblems } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import { generateLaunchJson, LAUNCH_OUTPUT_REL_PATH } from "./🖥️launch.ts";

//#region 🔖️PluginRegistryEntry
export type PluginHostMetadata = {
  readonly landingAppId: string;
  readonly hostAppId: string;
};

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly cratePath: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  readonly host?: PluginHostMetadata;
};

//#region 🏛️DiscoveryContract
/** @emoji 🔣️ The one shared taxonomy vocabulary (`🦑️repo/📚️lib`'s `🔣️taxonomy.json`), read once. Every
 * directory-name, manifest-filename, role and area literal this script used to hardcode as a path regex
 * now comes from here, so registry discovery can never drift from the root policy script's or the SDK
 * testkit's view of the same contract — see mechanism ticket
 * `26/08/06/MECHANISM-VOCABULARY-AND-DISCOVERY-LIBRARY`. */
const TAXONOMY = loadTaxonomy();

/** @emoji 🗺️ Area root of the plugin tree, cross-checked against `taxonomy.areas` at load time so this
 * literal can never outlive a vocabulary rename: the area's declared `AreaState` — not a hand-flipped
 * boolean — decides whether pre-Shape-V2 crates are still discovered and whether taxonomy findings warn
 * or fail (see `PLUGINS_AREA_STATE`). */
const PLUGINS_AREA = "✏️s/🔌️plugins";
if (!(PLUGINS_AREA in TAXONOMY.areas)) throw new Error(`📇️registry: "${PLUGINS_AREA}" is not a declared area in 🔣️taxonomy.json (${Object.keys(TAXONOMY.areas).join(", ")})`);

/** @emoji 🗺️ Declared migration state of the plugin area — the per-area replacement for the removed
 * `LEGACY_LAYOUT_TOLERANT` boolean. `legacy`/`mixed` ⇒ pre-Shape-V2 crates are still part of the catalog
 * and taxonomy findings are warn-only; `clean` ⇒ the legacy arm goes silent and the findings fail the
 * gate (the W10 finalization flip becomes a one-word vocabulary edit, not a code change). */
const PLUGINS_AREA_STATE: AreaState = areaOf(PLUGINS_AREA, TAXONOMY) ?? "legacy";

/** @emoji 🏚️ True while an area still admits the pre-Shape-V2 sandwich layout. */
function areaAdmitsLegacyShape(state: AreaState): boolean {
  return state === "legacy" || state === "mixed";
}

/** @emoji 🎛️ Taxonomy tree segment names, single-sourced from the vocabulary: anything deriving an
 * app-root path (the constitutional gate, example discovery, the window audit) shares one value. */
const APPS_DIRNAME = TAXONOMY.appsDirName;
/** @emoji 📚️ Artifact-scoped example data dir (`artifactChildDirs`, not owner root). */
const EXAMPLES_DIRNAME = "📚️examples";
if (!TAXONOMY.artifactChildDirs.includes(EXAMPLES_DIRNAME)) {
  throw new Error(`📇️registry: "${EXAMPLES_DIRNAME}" must be listed in 🔣️taxonomy.json artifactChildDirs (${TAXONOMY.artifactChildDirs.join(", ")})`);
}
const EXAMPLE_COMPONENT_DIRS = TAXONOMY.exampleComponentDirs ?? [];
const RUST_LANG = "🦀️rust";
const RUST_MANIFEST_FILENAME = TAXONOMY.ecosystems[RUST_LANG].manifestFilename ?? "Cargo.toml";

/** @emoji 🚫️ Build/vendor noise and dot-directories (e.g. `.claude/worktrees/…`, which used to leak
 * duplicate registry rows for every crate that also exists inside a worktree checkout). */
const WALK_SKIP_DIRS = new Set(["node_modules", "target", "🤖️generated"]);

/** @emoji 🧩️ Roles whose packages may carry a `[package.metadata.component]` wasm component and thus
 * belong in the plugin catalog: the plugin itself and the extensions it contributes. Every other role
 * (`framework`, `tool`, `s-module`, …) is filtered out by `tryParsePluginCargo` anyway — listing them
 * here keeps the intent explicit instead of implicit in a downstream parse failure. */
const COMPONENT_ROLES: ReadonlySet<PackageRole> = new Set<PackageRole>(["plugin", "extension"]);

/** @emoji 📦️ Every rust package in the repo that declares a component-bearing role, via the shared
 * `discoverPackages()` walk (two-level `📦️packages/🦀️rust/` and three-level `🎯️targets/<t>/` shapes
 * alike). Replaces the two hand-written "new contract" path regexes this script used to carry. */
function discoverComponentPackages(repoRoot: string): DiscoveredPackage[] {
  return discoverPackages(repoRoot, TAXONOMY).filter((pkg) => pkg.lang === RUST_LANG && COMPONENT_ROLES.has(pkg.role));
}

/** @emoji 🏚️ Absolute path of a legacy `<dir>/<forbidden segment>/🦀️rust/Cargo.toml` sandwich manifest
 * when one exists — the vocabulary-driven replacement for the `"⚡️implementations"` literals this
 * script used to splice into paths by hand (both spellings are covered, per
 * `taxonomy.forbiddenPathSegments`). */
function legacyRustManifestIn(dir: string): string | undefined {
  for (const segment of TAXONOMY.forbiddenPathSegments) {
    const manifestPath = join(dir, segment, RUST_LANG, RUST_MANIFEST_FILENAME);
    if (existsSync(manifestPath)) return manifestPath;
  }
  return undefined;
}

/**
 * @emoji 🏚️ Pre-Shape-V2 component crates still on disk inside the plugin area: a
 * `<forbidden segment>/🦀️rust/Cargo.toml` sandwich (the legacy plugin bundle crate and its
 * `🧩️extensions` siblings). Selected structurally from the vocabulary — `forbiddenPathSegments` plus
 * the rust ecosystem's manifest filename — instead of the three hand-written path regexes this script
 * used to carry, and gated on the plugin area's declared state so the whole arm disappears by
 * vocabulary edit at the finalization flip. Crates matching the shape without a
 * `[package.metadata.component]` package are dropped downstream by `tryParsePluginCargo`, exactly as
 * before.
 */
function findLegacyComponentManifests(repoRoot: string): string[] {
  if (!areaAdmitsLegacyShape(PLUGINS_AREA_STATE)) return [];
  const forbidden = new Set(TAXONOMY.forbiddenPathSegments);
  const out: string[] = [];
  function walk(dir: string) {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || WALK_SKIP_DIRS.has(name) || name === TAXONOMY.packagesDirName) continue;
      const path = join(dir, name);
      let st: ReturnType<typeof statSync>;
      try {
        st = statSync(path);
      } catch {
        continue;
      }
      if (!st.isDirectory()) continue;
      if (forbidden.has(name)) {
        const manifestPath = join(path, RUST_LANG, RUST_MANIFEST_FILENAME);
        if (existsSync(manifestPath)) out.push(manifestPath);
        continue;
      }
      walk(path);
    }
  }
  const areaRoot = join(repoRoot, ...PLUGINS_AREA.split("/"));
  if (existsSync(areaRoot)) walk(areaRoot);
  return out;
}
//#endregion 🏛️DiscoveryContract

/** @emoji 🧭️ Every manifest that may contribute a row to the plugin catalog: the shared package
 * discovery contract plus, while the plugin area is pre-`clean`, the legacy sandwich crates a plugin
 * sheds when it migrates. A plugin discoverable under both shapes at once is "in-flight", not an
 * error. */
function findPluginCargoFiles(root: string): string[] {
  const contract = discoverComponentPackages(root).map((pkg) => join(root, pkg.manifestPath));
  return [...new Set([...contract, ...findLegacyComponentManifests(root)])].sort();
}

function parsePluginCargo(manifestPath: string, repoRoot: string): PluginRegistryEntry {
  const text = readFileSync(manifestPath, "utf8");
  const packageName = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`missing package name in ${manifestPath}`);
  const componentPackage = text.match(/\[package\.metadata\.component\][\s\S]*?^package = "semio:([^"]+)"/m)?.[1];
  if (!componentPackage) throw new Error(`missing [package.metadata.component].package in ${manifestPath}`);
  const cratePath = relative(repoRoot, dirname(manifestPath));
  const wasmOut = `${packageName.replace(/-/g, "_")}.wasm`;
  const semioBlock = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[package.metadata.semio]")[0];
  const contributes = semioBlock ? parseTomlStringArray(semioBlock.join("\n"), "contributes") : [];
  const consumes = semioBlock ? parseTomlStringArray(semioBlock.join("\n"), "consumes") : [];
  const hostBlock = semioBlock?.join("\n").match(/^host\s*=\s*\{([^}]*)\}/m)?.[1];
  const landingAppId = hostBlock?.match(/landing\s*=\s*"([^"]+)"/)?.[1];
  const hostAppId = hostBlock?.match(/studio\s*=\s*"([^"]+)"/)?.[1];
  const host = landingAppId && hostAppId ? { landingAppId, hostAppId } : undefined;
  return { pluginId: componentPackage, cratePath, packageName, wasmOut, contributes, consumes, ...(host ? { host } : {}) };
}

//#region 🔖️PlaygroundEntry
/** @emoji 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
 * plugin crate. `app` optionally scopes the row to one playground variant of a multi-app crate (unset
 * ⇒ every variant of the crate). Mirrors the TS discriminated union emitted for consumers as
 * `PlaygroundAssetSpec` (see `emitPlaygroundsTypeScript`). */
export type AssetSpecRow = {
  readonly kind: "tile-proxy" | "static-dir" | "mesh-collection";
  readonly route: string;
  readonly app?: string;
  readonly upstream?: string;
  readonly cache?: string;
  readonly root?: string;
  readonly roots?: readonly string[];
  readonly placeholder?: string;
  readonly filterFromExamples?: boolean;
};

/** @emoji 🎮️ One `[[package.metadata.semio.playground]]` row scoped to its owning plugin crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** @emoji 🏷️ Shell brand id (see `framework/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  readonly examples: readonly string[];
  /** @emoji 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** @emoji 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};

function tomlBlocksAfterHeader(lines: readonly string[], headerTest: (line: string) => boolean): string[][] {
  const blocks: string[][] = [];
  for (let i = 0; i < lines.length; i++) {
    if (!headerTest(lines[i].trim())) continue;
    const body: string[] = [];
    for (let j = i + 1; j < lines.length; j++) {
      if (lines[j].trim().startsWith("[")) break;
      body.push(lines[j]);
    }
    blocks.push(body);
  }
  return blocks;
}

function parseTomlStringArray(block: string, key: string): string[] {
  const match = block.match(new RegExp(`^${key}\\s*=\\s*\\[([^\\]]*)\\]`, "m"));
  if (!match) return [];
  return [...match[1].matchAll(/"([^"]*)"/g)].map((m) => m[1]);
}

function parseTomlBoolField(block: string, key: string): boolean {
  return new RegExp(`^${key}\\s*=\\s*true\\s*$`, "m").test(block);
}

function parsePlaygroundBlock(block: string, pluginId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  const engines = parseTomlStringArray(block, "engines");
  return { variant, pluginId, cratePath, app, brand, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, examples: [], engines, assets: [] };
}

/** @emoji 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate manifest. */
function parseAssetsForCrate(manifestPath: string): AssetSpecRow[] {
  if (!existsSync(manifestPath)) return [];
  const text = readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.assets]]");
  const rows: AssetSpecRow[] = [];
  for (const blockLines of blocks) {
    const block = blockLines.join("\n");
    const kind = block.match(/^kind\s*=\s*"([^"]+)"/m)?.[1] as AssetSpecRow["kind"] | undefined;
    const route = block.match(/^route\s*=\s*"([^"]+)"/m)?.[1];
    if (!kind || !route) {
      continue;
    }
    const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
    const upstream = block.match(/^upstream\s*=\s*"([^"]+)"/m)?.[1];
    const cache = block.match(/^cache\s*=\s*"([^"]+)"/m)?.[1];
    const root = block.match(/^root\s*=\s*"([^"]+)"/m)?.[1];
    const roots = parseTomlStringArray(block, "roots");
    const placeholder = block.match(/^placeholder\s*=\s*"([^"]+)"/m)?.[1];
    const filterFromExamples = parseTomlBoolField(block, "filter_from_examples");
    rows.push({
      kind,
      route,
      ...(app ? { app } : {}),
      ...(upstream ? { upstream } : {}),
      ...(cache ? { cache } : {}),
      ...(root ? { root } : {}),
      ...(roots.length ? { roots } : {}),
      ...(placeholder ? { placeholder } : {}),
      ...(filterFromExamples ? { filterFromExamples: true } : {}),
    });
  }
  return rows;
}

/**
 * @emoji 🖼️ Example ids for one playground row: the bundle crate lives at
 * `s/plugin/<p>/manifest/artifact/rs`, so the plugin root is always the first 3 path segments.
 * Tries the plugin root's own `example/` dir (single-app flat plugins), then — for multi-app
 * plugins where the playground `variant` diverges from the plugin id (`puzzle2d` - `puzzle` =
 * `2d`) — the constitutional `app/<suffix>/example` dir. Mirrors
 * `framework/ui/tui/rs`'s `discover_examples_for_playground` byte-for-byte.
 */
function discoverExamplesForPlayground(repoRoot: string, cratePath: string, pluginId: string, variant: string): string[] {
  const idsIn = (dir: string): string[] => {
    if (!existsSync(dir)) return [];
    const ids = readdirSync(dir)
      .filter((name) => name.endsWith(".json"))
      .map((name) => name.split(".")[0]);
    return [...new Set(ids)].sort();
  };
  const segments = cratePath.split("/");
  // 🏛️ Inside the plugin area the tech root is the plugin folder (area segments + 1), not `✏️s` itself.
  const areaSegments = PLUGINS_AREA.split("/");
  const inPluginArea = areaSegments.every((segment, index) => segments[index] === segment);
  const techRoot = inPluginArea ? segments.slice(0, areaSegments.length + 1).join("/") : segments[0];
  const collectJsonExampleIds = (dir: string): string[] => {
    if (!existsSync(dir)) return [];
    return idsIn(dir);
  };
  const artifactExampleIds: string[] = [];
  const artifactsDir = join(repoRoot, techRoot, TAXONOMY.artifactsDirName);
  if (existsSync(artifactsDir)) {
    for (const artifact of listDirs(artifactsDir)) {
      const examplesRoot = join(artifactsDir, artifact, EXAMPLES_DIRNAME);
      for (const setName of listDirs(examplesRoot)) {
        artifactExampleIds.push(...collectJsonExampleIds(join(examplesRoot, setName)));
      }
      artifactExampleIds.push(...collectJsonExampleIds(examplesRoot));
    }
  }
  if (artifactExampleIds.length > 0) return [...new Set(artifactExampleIds)].sort();
  const rootDir = join(repoRoot, techRoot, EXAMPLES_DIRNAME);
  if (existsSync(rootDir)) return idsIn(rootDir);
  if (variant.startsWith(pluginId) && variant.length > pluginId.length) {
    const suffix = variant.slice(pluginId.length);
    // 🎛️ Uses the shared `APPS_DIRNAME` constant (not an independently hardcoded literal) so this
    // keeps resolving correctly once a plugin's crate path collapses to one crate per plugin — the
    // apps-container segment name is single-sourced against the constitutional gate below.
    const dir = join(repoRoot, techRoot, APPS_DIRNAME, suffix, EXAMPLES_DIRNAME);
    if (existsSync(dir)) return idsIn(dir);
  }
  return [];
}

function parsePlaygroundsForCrate(manifestPath: string, pluginId: string, cratePath: string): PlaygroundEntry[] {
  const text = readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]");
  const entries: PlaygroundEntry[] = [];
  for (const block of blocks) {
    const entry = parsePlaygroundBlock(block.join("\n"), pluginId, cratePath);
    if (entry) entries.push(entry);
  }
  return entries;
}

//#region 🧹️PlaygroundDedupe
/** @emoji 🧹️ In-flight migration dedupe (see `PLUGINS_AREA_STATE`): when a plugin's legacy bundle
 * crate and its new-contract taxonomy crate are BOTH on disk at once, they typically carry identical
 * `[[package.metadata.semio.playground]]` rows — same variant id, same aliases, same ports — which
 * would otherwise trip `validatePlaygroundRegistry`'s duplicate checks for the entire workspace on
 * every in-flight migration. Groups raw entries by variant id; a group where every entry shares one
 * `pluginId` AND at least one (but not all) entries come from a new-contract crate is the expected
 * transient migration shape, so only the new-contract entry/entries survive. A variant collision
 * across DIFFERENT plugin ids is left untouched — that is a genuine naming collision for
 * `validatePlaygroundRegistry` to catch. `contractCratePaths` is the repo-relative package-dir set from
 * `discoverComponentPackages`, so "is this the new shape?" is answered by the shared discovery walk
 * rather than by a second copy of a path regex. */
function dedupeInFlightPlaygroundEntries(playgrounds: readonly PlaygroundEntry[], contractCratePaths: ReadonlySet<string>): PlaygroundEntry[] {
  const byVariant = new Map<string, PlaygroundEntry[]>();
  for (const entry of playgrounds) byVariant.set(entry.variant, [...(byVariant.get(entry.variant) ?? []), entry]);
  const dropped = new Set<PlaygroundEntry>();
  for (const group of byVariant.values()) {
    if (group.length <= 1) continue;
    if (new Set(group.map((entry) => entry.pluginId)).size > 1) continue;
    const newContract = group.filter((entry) => contractCratePaths.has(entry.cratePath));
    if (newContract.length === 0 || newContract.length === group.length) continue;
    for (const entry of group) if (!newContract.includes(entry)) dropped.add(entry);
  }
  return dropped.size === 0 ? [...playgrounds] : playgrounds.filter((entry) => !dropped.has(entry));
}
//#endregion 🧹️PlaygroundDedupe

/** @emoji 🕹️ Scans every plugin/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PlaygroundEntry[] {
  const entries = generatePluginRegistry(repoRoot, options);
  const rawPlaygrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath);
    for (const playground of parsePlaygroundsForCrate(manifestPath, entry.pluginId, entry.cratePath)) {
      const assets = crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app);
      rawPlaygrounds.push({ ...playground, examples: discoverExamplesForPlayground(repoRoot, entry.cratePath, entry.pluginId, playground.variant), assets });
    }
  }
  const playgrounds = dedupeInFlightPlaygroundEntries(rawPlaygrounds, new Set(discoverComponentPackages(repoRoot).map((pkg) => pkg.packageRel)));
  for (let i = 0; i < playgrounds.length; i++) {
    const row = playgrounds[i];
    if (!row.brand || row.examples.length > 0) continue;
    const donor = playgrounds.find((other) => other !== row && other.cratePath === row.cratePath && other.app === row.app && other.examples.length > 0);
    if (donor) playgrounds[i] = { ...row, examples: donor.examples, engines: row.engines.length > 0 ? row.engines : donor.engines };
  }
  playgrounds.sort((a, b) => a.variant.localeCompare(b.variant));
  return playgrounds;
}
//#endregion

export type GeneratePluginRegistryOptions = {
  readonly filterPlaygroundPlugin?: string;
};

/** @emoji 🎯️ Resolves a playground variant/alias or bare plugin id to its wasm registry plugin id. */
export function resolveRegistryPluginIdForFilter(pluginFilter: string, repoRoot = getWorkspaceRoot()): string {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const text = readFileSync(manifestPath, "utf8");
    const componentPackage = text.match(/\[package\.metadata\.component\][\s\S]*?^package = "semio:([^"]+)"/m)?.[1];
    if (!componentPackage) continue;
    for (const block of tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]")) {
      const body = block.join("\n");
      const variant = body.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
      if (!variant) continue;
      const aliases = parseTomlStringArray(body, "aliases");
      if (variant === pluginFilter || aliases.includes(pluginFilter)) return componentPackage;
    }
  }
  return pluginFilter;
}

function pluginEntryHasHost(pluginId: string, repoRoot: string): boolean {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const entry = tryParsePluginCargo(manifestPath, repoRoot);
    if (entry?.pluginId === pluginId) return entry.host !== undefined;
  }
  return false;
}

/** @emoji 🏠️ True when the filter resolves to a plugin crate that declares `[package.metadata.semio].host`. */
export function isStudioPluginFilter(pluginFilter?: string, repoRoot = getWorkspaceRoot()): boolean {
  if (!pluginFilter) return true;
  return pluginEntryHasHost(resolveRegistryPluginIdForFilter(pluginFilter, repoRoot), repoRoot);
}

/**
 * 🎯️ Resolves a raw playground filter (a variant id like "puzzle5d", or an already-bare crate
 * pluginId like "note") to the set of crate pluginIds that must be built for one dev session: the
 * target crate itself plus every crate whose declared `contributes` intersects the target crate
 * `consumes` (per `[package.metadata.semio]` in each crate Cargo.toml — no more registry-id
 * indirection through framework/core/js).
 */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string): readonly string[] {
  const repoRoot = getWorkspaceRoot();
  const allEntries = generatePluginRegistry(repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const variantRow = playgrounds.find((p) => p.variant === filterPlaygroundPlugin);
  const targetPluginId = variantRow?.pluginId ?? filterPlaygroundPlugin;
  const targetEntry = allEntries.find((e) => e.pluginId === targetPluginId);
  const ids = new Set<string>([targetPluginId]);
  if (targetEntry) {
    for (const entry of allEntries) {
      if (entry.pluginId === targetPluginId) continue;
      if (entry.contributes.some((topic) => targetEntry.consumes.includes(topic))) ids.add(entry.pluginId);
    }
  }
  return [...ids];
}

function findPluginCargoPathsForIds(repoRoot: string, pluginIds: readonly string[]): string[] {
  const idSet = new Set(pluginIds);
  return findPluginCargoFiles(repoRoot).filter((path) => {
    const entry = tryParsePluginCargo(path, repoRoot);
    return entry !== undefined && idSet.has(entry.pluginId);
  });
}

function tryParsePluginCargo(manifestPath: string, repoRoot: string): PluginRegistryEntry | undefined {
  try {
    return parsePluginCargo(manifestPath, repoRoot);
  } catch {
    return undefined;
  }
}

export function generatePluginRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PluginRegistryEntry[] {
  const filterPlaygroundPlugin = options.filterPlaygroundPlugin;
  const filterIds = filterPlaygroundPlugin && !isStudioPluginFilter(filterPlaygroundPlugin) ? resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin) : undefined;
  const manifestPaths = filterIds ? findPluginCargoPathsForIds(repoRoot, filterIds) : findPluginCargoFiles(repoRoot);
  const entries: PluginRegistryEntry[] = [];
  for (const path of manifestPaths) {
    const entry = tryParsePluginCargo(path, repoRoot);
    if (entry) entries.push(entry);
  }
  entries.sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  return entries;
}

function emitTypeScript(entries: PluginRegistryEntry[]): string {
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, landingAppId: ${JSON.stringify(entry.host!.landingAppId)}, hostAppId: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const rows = entries
    .map((entry) => {
      const host = entry.host ? `, host: { landingAppId: ${JSON.stringify(entry.host.landingAppId)}, hostAppId: ${JSON.stringify(entry.host.hostAppId)} }` : "";
      return `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, cratePath: ${JSON.stringify(entry.cratePath)}, wasmOut: ${JSON.stringify(entry.wasmOut)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)}${host} },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PluginHostMetadata = {
\treadonly landingAppId: string;
\treadonly hostAppId: string;
};

export type PluginHostConfig = PluginHostMetadata & {
\treadonly pluginId: string;
};

export type PluginBuildTarget = {
\treadonly pluginId: string;
\treadonly cratePath: string;
\treadonly wasmOut: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
\treadonly host?: PluginHostMetadata;
};

export const PLUGIN_HOST_CONFIGS: readonly PluginHostConfig[] = [
${hostRows}
];

export const PLUGIN_BUILD_TARGETS: readonly PluginBuildTarget[] = [
${rows}
];

export const PROGRAM_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
\tpluginId: target.pluginId,
\tmoduleUrl: \`/plugin-modules/\${target.pluginId}/\${target.wasmOut.replace(/\\.wasm$/, ".js")}\`,
}));

export const pluginModuleUrl = (pluginId: string, fileName: string) =>
\t\`/plugin-modules/\${pluginId}/\${fileName.replace(/\\.wasm$/, ".js")}\`;
`;
}

function emitAssetSpecTypeScript(asset: AssetSpecRow): string {
  const fields = [`kind: ${JSON.stringify(asset.kind)}`, `route: ${JSON.stringify(asset.route)}`];
  if (asset.upstream !== undefined) fields.push(`upstream: ${JSON.stringify(asset.upstream)}`);
  if (asset.cache !== undefined) fields.push(`cache: ${JSON.stringify(asset.cache)}`);
  if (asset.root !== undefined) fields.push(`root: ${JSON.stringify(asset.root)}`);
  if (asset.roots !== undefined) fields.push(`roots: ${JSON.stringify(asset.roots)}`);
  if (asset.placeholder !== undefined) fields.push(`placeholder: ${JSON.stringify(asset.placeholder)}`);
  if (asset.filterFromExamples) fields.push(`filterFromExamples: true`);
  return `{ ${fields.join(", ")} }`;
}

function emitPlaygroundsTypeScript(playgrounds: PlaygroundEntry[]): string {
  const rows = playgrounds
    .map((entry) => {
      const app = entry.app !== undefined ? `, app: ${JSON.stringify(entry.app)}` : "";
      const brand = entry.brand !== undefined ? `, brand: ${JSON.stringify(entry.brand)}` : "";
      const assets = entry.assets.map(emitAssetSpecTypeScript).join(", ");
      return `\t{ variant: ${JSON.stringify(entry.variant)}, pluginId: ${JSON.stringify(entry.pluginId)}, cratePath: ${JSON.stringify(entry.cratePath)}${app}${brand}, aliases: ${JSON.stringify(entry.aliases)}, ports: { react: ${entry.ports.react}, wgpu: ${entry.ports.wgpu} }, examples: ${JSON.stringify(entry.examples)}, engines: ${JSON.stringify(entry.engines)}, assets: [${assets}] },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PlaygroundAssetSpec =
\t| { readonly kind: "tile-proxy"; readonly route: string; readonly upstream: string; readonly cache: string }
\t| { readonly kind: "static-dir"; readonly route: string; readonly root: string }
\t| { readonly kind: "mesh-collection"; readonly route: string; readonly roots: readonly string[]; readonly placeholder: string; readonly filterFromExamples?: boolean };

export type PlaygroundBuildTarget = {
\treadonly variant: string;
\treadonly pluginId: string;
\treadonly cratePath: string;
\treadonly app?: string;
\treadonly brand?: string;
\treadonly aliases: readonly string[];
\treadonly ports: { readonly react: number; readonly wgpu: number };
\treadonly examples: readonly string[];
\treadonly engines: readonly string[];
\treadonly assets: readonly PlaygroundAssetSpec[];
};

export const PLAYGROUND_BUILD_TARGETS: readonly PlaygroundBuildTarget[] = [
${rows}
];
`;
}

//#region 🏛️FrameworkPackageCatalog
/** @emoji 🏛️ One framework package as seen by the shared discovery contract (`role = "framework"`).
 * The framework families are not wasm components, so they never enter `PLUGIN_BUILD_TARGETS`; this is
 * their own catalog section — the consumable answer to "which framework packages exist, in which
 * language/render target, and how far has their owner migrated" that every downstream mechanism
 * (workspaces generator, storybook scopes, dep-cruiser) previously had to rediscover by hand. */
export type FrameworkPackageEntry = {
  readonly id: string;
  readonly ownerPath: string;
  readonly packagePath: string;
  readonly lang: string;
  readonly target?: string;
  readonly area: string;
  readonly maturity: string;
};

/** @emoji 🏛️ Framework-role half of the shared `discoverPackages()` walk (three-level `🎯️targets`
 * aware), flattened into a stable catalog. Plugin and framework catalogs therefore come from one
 * traversal and one vocabulary, and can never drift apart. */
export function generateFrameworkPackageRegistry(repoRoot = getWorkspaceRoot()): FrameworkPackageEntry[] {
  return discoverPackages(repoRoot, TAXONOMY)
    .filter((pkg) => pkg.role === "framework")
    .map((pkg) => ({
      id: pkg.id,
      ownerPath: pkg.ownerRel,
      packagePath: pkg.packageRel,
      lang: pkg.lang,
      ...(pkg.target ? { target: pkg.target } : {}),
      area: pkg.area,
      maturity: pkg.maturity,
    }))
    .sort((a, b) => a.id.localeCompare(b.id) || a.packagePath.localeCompare(b.packagePath));
}

function emitFrameworkPackagesTypeScript(entries: FrameworkPackageEntry[]): string {
  const rows = entries
    .map((entry) => {
      const target = entry.target !== undefined ? `, target: ${JSON.stringify(entry.target)}` : "";
      return `\t{ id: ${JSON.stringify(entry.id)}, ownerPath: ${JSON.stringify(entry.ownerPath)}, packagePath: ${JSON.stringify(entry.packagePath)}, lang: ${JSON.stringify(entry.lang)}${target}, area: ${JSON.stringify(entry.area)}, maturity: ${JSON.stringify(entry.maturity)} },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type FrameworkPackage = {
\treadonly id: string;
\treadonly ownerPath: string;
\treadonly packagePath: string;
\treadonly lang: string;
\treadonly target?: string;
\treadonly area: string;
\treadonly maturity: string;
};

export const FRAMEWORK_PACKAGES: readonly FrameworkPackage[] = [
${rows}
];
`;
}
//#endregion 🏛️FrameworkPackageCatalog

//#region 🎮️PlaygroundSession
export type PlaygroundSessionPlugin = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
};

export type PlaygroundSession = {
  readonly variant: string;
  readonly registryPluginId: string;
  readonly defaultAppId?: string;
  readonly studioMode: boolean;
  readonly host?: PluginHostMetadata;
  readonly plugins: readonly PlaygroundSessionPlugin[];
};

/** @emoji 🎮️ Builds the pre-expanded plugin list and host metadata for one playground launch. */
export function buildPlaygroundSession(variant: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const studioMode = isStudioPluginFilter(variant, repoRoot);
  const registryPluginId = resolveRegistryPluginIdForFilter(variant, repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const playground = playgrounds.find((entry) => entry.variant === variant || entry.aliases.includes(variant));
  const entries = generatePluginRegistry(repoRoot, studioMode ? {} : { filterPlaygroundPlugin: registryPluginId });
  const host = entries.find((entry) => entry.pluginId === registryPluginId)?.host;
  return {
    variant,
    registryPluginId,
    defaultAppId: playground?.app,
    studioMode,
    ...(host ? { host } : {}),
    plugins: entries.map((entry) => ({
      pluginId: entry.pluginId,
      moduleUrl: `/plugin-modules/${entry.pluginId}/${entry.wasmOut.replace(/\.wasm$/, ".js")}`,
      contributes: entry.contributes,
      consumes: entry.consumes,
    })),
  };
}

function emitSessionTypeScript(session: PlaygroundSession): string {
  const host = session.host ? `{ landingAppId: ${JSON.stringify(session.host.landingAppId)}, hostAppId: ${JSON.stringify(session.host.hostAppId)} }` : "undefined";
  const defaultAppId = session.defaultAppId !== undefined ? JSON.stringify(session.defaultAppId) : "undefined";
  const pluginRows = session.plugins
    .map((entry) => `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, moduleUrl: ${JSON.stringify(entry.moduleUrl)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)} },`)
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PlaygroundSessionPlugin = {
\treadonly pluginId: string;
\treadonly moduleUrl: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
};

export type PlaygroundSession = {
\treadonly variant: string;
\treadonly registryPluginId: string;
\treadonly defaultAppId?: string;
\treadonly studioMode: boolean;
\treadonly host?: { readonly landingAppId: string; readonly hostAppId: string };
\treadonly plugins: readonly PlaygroundSessionPlugin[];
};

export const PLAYGROUND_SESSION: PlaygroundSession = {
\tvariant: ${JSON.stringify(session.variant)},
\tregistryPluginId: ${JSON.stringify(session.registryPluginId)},
\tdefaultAppId: ${defaultAppId},
\tstudioMode: ${session.studioMode},
\thost: ${host},
\tplugins: [
${pluginRows}
\t],
};
`;
}

function emitRustHosts(entries: PluginRegistryEntry[], playgrounds: PlaygroundEntry[]): string {
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `    PluginHostConfig { plugin_id: ${JSON.stringify(entry.pluginId)}, landing_app_id: ${JSON.stringify(entry.host!.landingAppId)}, host_app_id: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const variantRows = playgrounds.map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.pluginId)}),`).join("\n");
  const aliasRows = playgrounds.flatMap((entry) => entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.pluginId)}),`)).join("\n");
  const variantAppRows = playgrounds.filter((entry) => entry.app).map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.app)}),`).join("\n");
  const aliasAppRows = playgrounds.flatMap((entry) => entry.app ? entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.app)}),`) : []).join("\n");
  return `// @generated by framework/plugin/registry/script.ts — do not edit.

pub struct PluginHostConfig {
    pub plugin_id: &'static str,
    pub landing_app_id: &'static str,
    pub host_app_id: &'static str,
}

pub const PLUGIN_HOST_CONFIGS: &[PluginHostConfig] = &[
${hostRows}
];

const PLAYGROUND_VARIANT_REGISTRY_IDS: &[(&str, &str)] = &[
${variantRows}
${aliasRows}
];

const PLAYGROUND_VARIANT_APP_IDS: &[(&str, &str)] = &[
${variantAppRows}
${aliasAppRows}
];

pub fn resolve_registry_plugin_id(plugin_filter: &str) -> &str {
    for (variant, plugin_id) in PLAYGROUND_VARIANT_REGISTRY_IDS {
        if *variant == plugin_filter {
            return plugin_id;
        }
    }
    plugin_filter
}

pub fn resolve_playground_app_id(plugin_filter: &str) -> Option<&'static str> {
    PLAYGROUND_VARIANT_APP_IDS.iter().find_map(|(variant, app_id)| (*variant == plugin_filter).then_some(*app_id))
}

pub fn resolve_plugin_host_config(plugin_filter: &str) -> Option<&'static PluginHostConfig> {
    let registry_id = resolve_registry_plugin_id(plugin_filter);
    PLUGIN_HOST_CONFIGS.iter().find(|entry| entry.plugin_id == registry_id)
}

pub fn is_space_mode(plugin_filter: &str) -> bool {
    resolve_plugin_host_config(plugin_filter).is_some()
}
`;
}

/** @emoji 🗂️ Emits plugin wasm artifact constants for headless `semio-framework-os-run`.
 * Paths are profile-relative; `resolve_plugin_paths` tries `debug` then `wasm-release`. */
function emitRustArtifacts(entries: PluginRegistryEntry[]): string {
  const rows = entries.map((entry) => `    (${JSON.stringify(entry.pluginId)}, ${JSON.stringify(entry.wasmOut)}),`).join("\n");
  return `// @generated by framework/plugin/registry/script.ts — do not edit.

pub const PLUGIN_WASM_TARGET_DIR: &str = "target/wasm32-wasip2";
pub const PLUGIN_WASM_PROFILE_DIRS: &[&str] = &["debug", "wasm-release"];
pub const PLUGIN_WASM_ARTIFACTS: &[(&str, &str)] = &[
${rows}
];
`;
}

/** @emoji 💾️ Writes the per-launch playground session artifact consumed by os/dev and wgpu boot. */
export function writePlaygroundSession(variant: string, outPath: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const session = buildPlaygroundSession(variant, repoRoot);
  mkdirSync(dirname(outPath), { recursive: true });
  writeFileSync(outPath, emitSessionTypeScript(session));
  return session;
}
//#endregion 🎮️PlaygroundSession

/** @emoji 🚦️ Cross-checks the flattened playground catalog for global uniqueness and multi-app crate discipline; returns human-readable violations. */
function validatePlaygroundRegistry(playgrounds: PlaygroundEntry[]): string[] {
  const errors: string[] = [];
  const variantOwners = new Map<string, string>();
  const aliasOwners = new Map<string, string>();
  const portOwners = new Map<string, string>();
  const entriesByCrate = new Map<string, PlaygroundEntry[]>();
  for (const entry of playgrounds) {
    if (variantOwners.has(entry.variant)) {
      errors.push(`duplicate playground variant "${entry.variant}" (${variantOwners.get(entry.variant)} and ${entry.cratePath})`);
    } else {
      variantOwners.set(entry.variant, entry.cratePath);
    }
    for (const alias of entry.aliases) {
      if (aliasOwners.has(alias)) {
        errors.push(`duplicate playground alias "${alias}" (variants "${aliasOwners.get(alias)}" and "${entry.variant}")`);
      } else {
        aliasOwners.set(alias, entry.variant);
      }
    }
    const portKey = `${entry.ports.react}:${entry.ports.wgpu}`;
    if (portOwners.has(portKey)) {
      errors.push(`duplicate playground ports react=${entry.ports.react}/wgpu=${entry.ports.wgpu} (variants "${portOwners.get(portKey)}" and "${entry.variant}")`);
    } else {
      portOwners.set(portKey, entry.variant);
    }
    entriesByCrate.set(entry.cratePath, [...(entriesByCrate.get(entry.cratePath) ?? []), entry]);
  }
  for (const group of entriesByCrate.values()) {
    if (group.length <= 1) continue;
    for (const entry of group) {
      if (!entry.app) errors.push(`playground variant "${entry.variant}" in ${entry.cratePath} must set "app" (crate declares ${group.length} playground entries)`);
    }
  }
  return errors;
}

//#region 🏛️ConstitutionalCrateGate
/** @emoji 🏛️ The seven mandatory constitutional-crate slots every `s/plugin/<p>/app/<a>/` must carry
 * (see `.🦑️repo/🎫️tickets/26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES/w31-constitutional-split-recipe.md`).
 * `rs` sits directly at `<appDir>/rs`; every other slot sits at `<appDir>/<slot>/rs`. */
const CONSTITUTIONAL_SLOTS = ["engine", "dsl", "op", "pack", "protocol", "ui"] as const;
/** @emoji 🔤️ Slot word -> its emoji+name directory segment (matches the repo's emoji+name naming scheme). */
const CONSTITUTIONAL_SLOT_DIRNAME: Record<(typeof CONSTITUTIONAL_SLOTS)[number], string> = {
  engine: "⚙️engine",
  dsl: "🗣️dsl",
  op: "🔧️op",
  pack: "🎒️pack",
  protocol: "📡️protocol",
  ui: "🖱️ui",
};

/** @emoji 🚦️ Gate 1 (build-time): every `s/plugin/<p>/app/…` directory — whether a single flattened
 * app (`app/rs/Cargo.toml` sits directly under `app/`) or a multi-app plugin (`app/<a>/rs/Cargo.toml`
 * per subdirectory) — must carry all seven constitutional-crate slot manifests. Hard-fails `check` so
 * a future split can never silently regress to a partial 4-of-7 or 5-of-7 crate set. Plugins that
 * haven't reached `s/plugin/` yet (nothing under `app/`) are out of scope, not a violation — this gate
 * only enforces completeness for plugins that have already started the split. Goes silent entirely
 * once the plugin area is declared `clean`: the legacy constitution has no meaning without legacy
 * crates. */
function validateConstitutionalCrates(repoRoot: string, migratedPluginIds: ReadonlySet<string> = new Set()): string[] {
  const errors: string[] = [];
  if (!areaAdmitsLegacyShape(PLUGINS_AREA_STATE)) return errors;
  const pluginRoot = join(repoRoot, ...PLUGINS_AREA.split("/"));
  if (!existsSync(pluginRoot)) return errors;
  for (const pluginName of readdirSync(pluginRoot).sort()) {
    // 🚦️ A plugin already discovered under the new one-crate-per-plugin taxonomy contract
    // (`findNewContractPluginRoots`) is validated by `validateTaxonomyTree` instead — it has
    // deliberately shed the seven legacy per-app crate slots, that is not a regression.
    if (migratedPluginIds.has(pluginName)) continue;
    const appRoot = join(pluginRoot, pluginName, APPS_DIRNAME);
    if (!existsSync(appRoot) || !statSync(appRoot).isDirectory()) continue;
    const isFlatSingleApp = legacyRustManifestIn(appRoot) !== undefined;
    const appDirs = isFlatSingleApp ? [{ label: pluginName, dir: appRoot }] : readdirSync(appRoot)
      .filter((name) => statSync(join(appRoot, name)).isDirectory())
      .filter((name) => name !== "🤝️shared")
      .map((name) => ({ label: `${pluginName}/${name}`, dir: join(appRoot, name) }));
    for (const { label, dir } of appDirs) {
      const missing: string[] = [];
      if (legacyRustManifestIn(dir) === undefined) missing.push("rs");
      for (const slot of CONSTITUTIONAL_SLOTS) {
        if (legacyRustManifestIn(join(dir, "🔨️modules", CONSTITUTIONAL_SLOT_DIRNAME[slot])) === undefined) missing.push(slot);
      }
      if (missing.length > 0) {
        errors.push(`${PLUGINS_AREA}/${label} is missing constitutional crate slot(s): ${missing.join(", ")} (expected 7: rs, ${CONSTITUTIONAL_SLOTS.join(", ")})`);
      }
    }
  }
  return errors;
}
//#endregion 🏛️ConstitutionalCrateGate

//#region 🗿️TaxonomyValidator
/** @emoji 🗿️ Every artifact node must carry all five taxonomy component slots — sourced from
 * `🔣️taxonomy.json` (single vocabulary source of truth, see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`; this used to be an independently
 * hand-maintained copy, which is exactly the drift `🔣️taxonomy.json` exists to prevent). */
const TAXONOMY_ARTIFACT_COMPONENTS = TAXONOMY.artifactComponentDirs;
const TAXONOMY_ARTIFACT_SPEC_FILENAMES = TAXONOMY.artifactSpecFilenames ?? {};
const TAXONOMY_TS_LEAF_FILENAME = TAXONOMY.ecosystems["🟦️typescript"]?.leafFilename ?? "🟦️component.ts";
/** @emoji 🪟️ A window dir may only contain these children, each itself a `🦀️component.rs` leaf. */
const TAXONOMY_WINDOW_CHILDREN = new Set(TAXONOMY.windowChildDirs);
const TAXONOMY_LEAF_FILENAME = TAXONOMY.taxonomyLeafFilenames[RUST_LANG];
/** @emoji 🚪️ Rust entry filename and its Shape V2 home relative to the owner root. */
const RUST_ENTRY_FILENAME = TAXONOMY.entryFilenames[RUST_LANG];
const RUST_ENTRY_DIR_FROM_OWNER = TAXONOMY.rustEntryPathRules.entryDirFromOwner.split("/");

/** @emoji 🧭️ Plugin roots discovered via the shared package contract (`role = "plugin"`, rust, owner
 * sitting directly under the plugin area) — distinct from `findPluginCargoFiles`, which additionally
 * matches the legacy sandwich shape. Drives both the taxonomy tree audit and the constitutional gate's
 * migrated-plugin exemption, so the two can never disagree about which plugins have moved. */
function findNewContractPluginRoots(repoRoot: string): { pluginId: string; pluginRoot: string }[] {
  return discoverPackages(repoRoot, TAXONOMY)
    .filter((pkg) => pkg.role === "plugin" && pkg.lang === RUST_LANG && dirname(pkg.ownerRel) === PLUGINS_AREA)
    .map((pkg) => ({ pluginId: basename(pkg.ownerRel), pluginRoot: join(repoRoot, pkg.ownerRel) }))
    .sort((a, b) => a.pluginId.localeCompare(b.pluginId));
}

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

/** @emoji 🚦️ Structural audit of one migrated plugin's taxonomy tree, entirely against
 * `🔣️taxonomy.json`'s vocabulary. Severity is decided by the caller from the plugin area's declared
 * maturity: warn while it is `legacy`/`mixed`, hard failure once it is `clean` (at which point it
 * fully replaces `validateConstitutionalCrates`). */
function validateTaxonomyTree(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  const artifactsDir = join(pluginRoot, TAXONOMY.artifactsDirName);
  for (const artifact of listDirs(artifactsDir)) {
    for (const component of TAXONOMY_ARTIFACT_COMPONENTS) {
      const facetDir = join(artifactsDir, artifact, component);
      if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
      }
      const specName = TAXONOMY_ARTIFACT_SPEC_FILENAMES[component];
      if (specName && !existsSync(join(facetDir, specName))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${specName}`);
      }
      if (!existsSync(join(facetDir, TAXONOMY_TS_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_TS_LEAF_FILENAME}`);
      }
    }
    const examplesRoot = join(artifactsDir, artifact, EXAMPLES_DIRNAME);
    if (!existsSync(examplesRoot)) {
      findings.push(`${pluginId}: artifact "${artifact}" is missing ${EXAMPLES_DIRNAME}/`);
      continue;
    }
    const exampleSets = listDirs(examplesRoot);
    if (exampleSets.length === 0) {
      findings.push(`${pluginId}: artifact "${artifact}" ${EXAMPLES_DIRNAME} has no example set`);
      continue;
    }
    for (const exampleSet of exampleSets) {
      for (const kind of EXAMPLE_COMPONENT_DIRS) {
        if (!existsSync(join(examplesRoot, exampleSet, kind))) {
          findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is missing ${kind}/`);
        }
      }
    }
  }

  if (existsSync(join(pluginRoot, EXAMPLES_DIRNAME))) {
    findings.push(`${pluginId}: plugin-root ${EXAMPLES_DIRNAME}/ is forbidden — relocate under 🗿️artifacts/<artifact>/${EXAMPLES_DIRNAME}`);
  }

  const appsDir = join(pluginRoot, APPS_DIRNAME);
  for (const app of listDirs(appsDir)) {
    const engineExamples = join(appsDir, app, "⚙️engine", EXAMPLES_DIRNAME);
    if (!existsSync(engineExamples)) {
      findings.push(`${pluginId}: app "${app}" is missing ⚙️engine/${EXAMPLES_DIRNAME}/`);
    }
  }

  // 🪟️ windows live under apps/<app>/modes/<mode>/windows/<w> and may only contain the fixed child set.
  for (const app of listDirs(appsDir)) {
    const modesDir = join(appsDir, app, TAXONOMY.modesDirName);
    for (const mode of listDirs(modesDir)) {
      const windowsDir = join(modesDir, mode, TAXONOMY.windowsDirName);
      for (const w of listDirs(windowsDir)) {
        for (const child of listDirs(join(windowsDir, w))) {
          if (!TAXONOMY_WINDOW_CHILDREN.has(child)) {
            findings.push(`${pluginId}: window "${app}/${mode}/${w}" has unexpected child "${child}" (expected one of ${[...TAXONOMY_WINDOW_CHILDREN].join(", ")})`);
          }
        }
      }
    }
  }

  // 🦀️ collect every actual component.rs on disk (for the lib.rs cross-check below) and flag any
  // taxonomy leaf file that isn't literally named `component.rs`.
  const componentFiles: string[] = [];
  const taxonomyLeafParents = new Set<string>([...TAXONOMY_ARTIFACT_COMPONENTS, ...TAXONOMY_WINDOW_CHILDREN, ...EXAMPLE_COMPONENT_DIRS]);
  function walkPluginTree(dir: string) {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (statSync(path).isDirectory()) {
        walkPluginTree(path);
        continue;
      }
      if (!name.endsWith(".rs")) continue;
      if (name === TAXONOMY_LEAF_FILENAME) {
        componentFiles.push(path);
      } else if (taxonomyLeafParents.has(dir.split("/").pop() ?? "")) {
        findings.push(`${pluginId}: taxonomy leaf file must be named ${TAXONOMY_LEAF_FILENAME}, found ${relative(pluginRoot, path)}`);
      }
    }
  }
  walkPluginTree(pluginRoot);

  // 📦️ lib.rs mod/#[path] cross-check: every component.rs on disk must be declared, and no declared
  // #[path] target may dangle (point at a file that doesn't exist) — reported as separate findings.
  // 🌳️ Shape V2-aware: the entry file lives at `📦️packages/🦀️rust/📦️lib.rs` under V2 or at `📦️lib.rs`
  // directly under the older V1 shape.
  //
  // 🧮️ #[path] resolution is CUMULATIVE, not always-relative-to-the-raw-file: each nested `pub mod X`
  // (or leaf `mod X;`) resolves its own `#[path]` string relative to its immediately enclosing mod's
  // *already-resolved* directory (defaulting to `<enclosing dir>/X` when no `#[path]` is given at all,
  // and to "no change" when the string is exactly `"."`) — confirmed empirically against a real,
  // compiling plugin (🖨️raster) that resets the base ONCE via `#[path = "../../."]` on an outer
  // grouping module and lets every nested `#[path = "."]` inherit it, as well as plugins (🏛️architect,
  // 📸️remodel, 🖍️draw) that instead prefix every LEAF path with `../../` and leave every nested `"."`
  // unprefixed — both are valid, and a flat "resolve every #[path] against the raw file directory"
  // approach mis-resolves the first style. So: walk the file's brace structure with a resolved-base
  // stack, seeded with the file's own directory.
  const v1LibRsPath = join(pluginRoot, RUST_ENTRY_FILENAME);
  const v2LibRsPath = join(pluginRoot, ...RUST_ENTRY_DIR_FROM_OWNER, RUST_ENTRY_FILENAME);
  const libRsPath = existsSync(v2LibRsPath) ? v2LibRsPath : v1LibRsPath;
  if (existsSync(libRsPath)) {
    const libDir = dirname(libRsPath);
    const libText = readFileSync(libRsPath, "utf8");
    const declaredAbs = new Set<string>();
    const danglingLeafPaths: string[] = [];

    // 🥞️ One stack frame per open `{` that followed a `mod`/`pub mod` declaration, holding that
    // scope's resolved base dir. A pending `#[path = "…"]` applies to the NEXT `mod` line only.
    const baseStack: string[] = [libDir];
    let pendingPath: string | null = null;
    const lines = libText.split("\n");
    for (const rawLine of lines) {
      const line = rawLine.trim();
      const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
      if (pathMatch) {
        pendingPath = pathMatch[1];
        continue;
      }
      const modMatch = line.match(/^(?:pub\s+)?mod\s+(\w+)\s*(\{|;)/);
      if (modMatch) {
        const parentBase = baseStack[baseStack.length - 1];
        const rawTarget = pendingPath ?? modMatch[1]; // no #[path] ⇒ default splice of the mod's own name
        const resolved = join(parentBase, rawTarget); // node:path's join already normalizes "." / ".." segments
        pendingPath = null;
        if (modMatch[2] === ";") {
          // Leaf: either a real component file (ends .rs) or a `mod tests;`-style non-path leaf — only
          // cross-check paths that look like a file (the taxonomy only ever points #[path] at .rs files).
          if (pendingPathLooksLikeFile(rawTarget)) {
            declaredAbs.add(resolved);
            if (!existsSync(resolved)) danglingLeafPaths.push(rawTarget);
          }
        } else {
          baseStack.push(resolved);
        }
        continue;
      }
      // Count bare closing braces against open mod scopes (lib.rs is wiring-only, so every `{`/`}` in
      // the file belongs to a mod block or the trailing semio_plugin! macro call — once the stack is
      // back to just the file base, further closes belong to the macro call and are ignored).
      const closes = (line.match(/\}/g) ?? []).length;
      const opens = (line.match(/\{/g) ?? []).length;
      for (let i = 0; i < closes - opens; i++) {
        if (baseStack.length > 1) baseStack.pop();
      }
    }

    function pendingPathLooksLikeFile(p: string): boolean {
      return p.endsWith(".rs");
    }

    for (const file of componentFiles) {
      if (!declaredAbs.has(file)) findings.push(`${pluginId}: ${relative(pluginRoot, file)} is not declared by any #[path] in ${RUST_ENTRY_FILENAME}`);
    }
    for (const p of danglingLeafPaths) {
      findings.push(`${pluginId}: ${RUST_ENTRY_FILENAME} declares #[path = "${p}"] but the file does not exist on disk`);
    }
  } else {
    findings.push(`${pluginId}: missing ${RUST_ENTRY_FILENAME} (checked plugin root and ${TAXONOMY.rustEntryPathRules.entryDirFromOwner}/)`);
  }

  // 🚫️ no `📡️protocol` path segment may remain under a migrated plugin (renamed to `📡️spr`).
  function containsProtocolSegment(dir: string): boolean {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (!statSync(path).isDirectory()) continue;
      if (name === "📡️protocol" || containsProtocolSegment(path)) return true;
    }
    return false;
  }
  if (containsProtocolSegment(pluginRoot)) findings.push(`${pluginId}: found a "📡️protocol" path segment under the plugin dir (renamed to 📡️spr)`);

  return findings;
}
//#endregion 🗿️TaxonomyValidator

/** @emoji 🧪️ Verifies that representative standalone and studio launches expand to complete sessions,
 * asserting shape rather than hardcoded plugin-id lists/counts so a plugin's crate-name change (or the
 * crate-consolidation restructure itself) can't silently break this check. */
function validatePlaygroundSessions(repoRoot: string): string[] {
  const errors: string[] = [];

  const standaloneVariant = "playbook";
  const standalone = buildPlaygroundSession(standaloneVariant, repoRoot);
  const standalonePluginIds = standalone.plugins.map((entry) => entry.pluginId).sort();
  // 🎯️ "standalone session = target plugin plus every plugin whose `contributes` intersects the
  // target's `consumes`" — exactly what `resolveRegistryPluginIdsForFilter` computes, so re-derive the
  // expectation instead of asserting a hardcoded id list.
  const expectedStandaloneIds = [...resolveRegistryPluginIdsForFilter(standaloneVariant)].sort();
  const expectedRegistryPluginId = resolveRegistryPluginIdForFilter(standaloneVariant, repoRoot);
  if (standalone.registryPluginId !== expectedRegistryPluginId || standalone.studioMode || standalonePluginIds.join(",") !== expectedStandaloneIds.join(",")) {
    errors.push(`standalone session "${standaloneVariant}" resolved unexpectedly (${JSON.stringify({ registryPluginId: standalone.registryPluginId, expectedRegistryPluginId, studioMode: standalone.studioMode, pluginIds: standalonePluginIds, expectedPluginIds: expectedStandaloneIds })})`);
  }

  const studioVariant = "s";
  const studio = buildPlaygroundSession(studioVariant, repoRoot);
  // 🎯️ "studio/host session has landingAppId==='home', hostAppId==='studio', and includes every
  // registry plugin" — `buildPlaygroundSession` expands studio sessions with no filter, so the exact
  // registry plugin count is the structural expectation, not a magic threshold.
  const totalRegistryPlugins = generatePluginRegistry(repoRoot).length;
  if (!studio.studioMode || studio.host?.landingAppId !== "home" || studio.host.hostAppId !== "studio" || studio.plugins.length !== totalRegistryPlugins) {
    errors.push(`studio session "${studioVariant}" resolved unexpectedly (${JSON.stringify({ studioMode: studio.studioMode, host: studio.host, pluginCount: studio.plugins.length, totalRegistryPlugins })})`);
  }

  if (!isStudioPluginFilter(studioVariant, repoRoot) || isStudioPluginFilter(standaloneVariant, repoRoot)) {
    errors.push("studio filter metadata does not distinguish host and standalone playgrounds");
  }
  return errors;
}

/** @emoji 🗂️ The full generated catalog, rendered in memory once and consumed by both `generate`
 * (writes) and `check` (byte-compares) so the two can never disagree about what belongs in
 * `🤖️generated/`. */
function renderCatalogFiles(repoRoot: string): { files: Record<string, string>; entries: PluginRegistryEntry[]; playgrounds: PlaygroundEntry[]; frameworkPackages: FrameworkPackageEntry[] } {
  const entries = generatePluginRegistry(repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const frameworkPackages = generateFrameworkPackageRegistry(repoRoot);
  return {
    entries,
    playgrounds,
    frameworkPackages,
    files: {
      "🔣️plugins.json": `${JSON.stringify(entries, null, 2)}\n`,
      "🟦️plugins.ts": emitTypeScript(entries),
      "🔣️playgrounds.json": `${JSON.stringify(playgrounds, null, 2)}\n`,
      "🟦️playgrounds.ts": emitPlaygroundsTypeScript(playgrounds),
      "🔣️framework.json": `${JSON.stringify(frameworkPackages, null, 2)}\n`,
      "🟦️framework.ts": emitFrameworkPackagesTypeScript(frameworkPackages),
      "🦀️hosts.rs": emitRustHosts(entries, playgrounds),
      "🦀️artifacts.rs": emitRustArtifacts(entries),
    },
  };
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const { files, entries, playgrounds, frameworkPackages } = renderCatalogFiles(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    mkdirSync(outDir, { recursive: true });
    for (const [name, content] of Object.entries(files)) writeFileSync(join(outDir, name), content);
    console.log(`plugin registry catalog refreshed (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages) -> ${outDir}`);
    // 🖥️ `.vscode/launch.json` is the second consumer of the very same playground catalog, so it is
    // regenerated here rather than from a separate entry point — `check` enforces its freshness. Written
    // last so a seed/devLaunchers problem can never leave the catalog itself unwritten.
    const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
    writeFileSync(launchPath, generateLaunchJson(repoRoot, playgrounds));
    console.log(`${LAUNCH_OUTPUT_REL_PATH} regenerated -> ${launchPath}`);
  }
}

/** @emoji 🔎️ Renders the catalog in memory and byte-compares it against `generated/*` plus
 * `.vscode/launch.json` — never writes (a lint/verify step must never let the auto-commit daemon land
 * regenerated files). Launch freshness is folded in here rather than living in a second, unenforced
 * entry point, so one `check` covers every artifact `generate` produces. */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const { files, entries, playgrounds, frameworkPackages } = renderCatalogFiles(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    const stale = Object.entries(files)
      .filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content)
      .map(([name]) => `generated/${name}`);
    // 🖥️ A seed/devLaunchers mismatch throws out of `generateLaunchJson`; report it as a violation
    // instead of an unhandled exception so the rest of the gate's findings still reach the dev.
    const launchViolations: string[] = [];
    try {
      const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
      const expectedLaunch = generateLaunchJson(repoRoot, playgrounds);
      if (!existsSync(launchPath) || readFileSync(launchPath, "utf8") !== expectedLaunch) stale.push(LAUNCH_OUTPUT_REL_PATH);
    } catch (error) {
      launchViolations.push(`${LAUNCH_OUTPUT_REL_PATH} cannot be rendered: ${(error as Error).message}`);
    }
    if (stale.length > 0) {
      console.error(`plugin registry catalog is stale: ${stale.join(", ")}`);
      console.error("run `bun nx run @semio-tech/plugin-registry:generate` to refresh.");
      process.exit(1);
    }
    const newContractPluginRoots = findNewContractPluginRoots(repoRoot);
    const migratedPluginIds = new Set(newContractPluginRoots.map(({ pluginId }) => pluginId));
    const violations = [...launchViolations, ...validatePlaygroundRegistry(playgrounds), ...validatePlaygroundSessions(repoRoot), ...validateConstitutionalCrates(repoRoot, migratedPluginIds)];
    if (violations.length > 0) {
      console.error("plugin registry catalog has playground validation errors:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    // 🗿️ Taxonomy tree audit for plugins discovered via the shared package contract. Its severity is
    // the plugin area's declared maturity, not a hand-flipped flag: warn while the area is
    // `legacy`/`mixed` (plugins still mid-migration), hard failure once it is declared `clean` — the
    // finalization flip is then a one-word edit in `🔣️taxonomy.json`.
    const taxonomyFindings = newContractPluginRoots.flatMap(({ pluginId, pluginRoot }) => validateTaxonomyTree(pluginRoot, pluginId));
    if (taxonomyFindings.length > 0) {
      if (areaAdmitsLegacyShape(PLUGINS_AREA_STATE)) {
        console.warn(`plugin taxonomy tree findings (area "${PLUGINS_AREA}" is "${PLUGINS_AREA_STATE}" — not failing the gate yet):`);
        for (const finding of taxonomyFindings) console.warn(`  - ${finding}`);
      } else {
        console.error(`plugin taxonomy tree violations (area "${PLUGINS_AREA}" is "${PLUGINS_AREA_STATE}"):`);
        for (const finding of taxonomyFindings) console.error(`  - ${finding}`);
        process.exit(1);
      }
    }
    // 🧭️ Shared-discovery diagnostics: a non-empty `discoverPackageProblems` outside a
    // legacy/mixed/exempt area means a manifest lost its role marker (the failure mode that silently
    // dropped a migrated extension crate from this very catalog) or a `🎯️targets/<target>/` dir is
    // missing its manifest. Warn-only while any area is pre-`clean`.
    const discoveryProblems = discoverPackageProblems(repoRoot, TAXONOMY);
    if (discoveryProblems.length > 0) {
      console.warn("package discovery problems:");
      for (const problem of discoveryProblems) console.warn(`  - [${problem.kind}] ${problem.message}`);
    }
    console.log(`plugin registry catalog is fresh (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages); ${LAUNCH_OUTPUT_REL_PATH} is fresh.`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("check", CheckScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
}
