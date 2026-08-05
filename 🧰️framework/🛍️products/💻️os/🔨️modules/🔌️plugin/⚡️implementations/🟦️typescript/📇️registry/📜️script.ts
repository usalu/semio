#!/usr/bin/env bun
/** 📜️ `@semio-tech/plugin-registry` — single-source plugin registry codegen from workspace crates. */
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript, getWorkspaceRoot, ScriptRouter, runBundleScriptMain } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

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
/** @emoji 🧭️ Accepts both the current 7-crate-per-app plugin shape and the future one-crate-per-plugin
 * taxonomy shape (`📦️packages/🦀️rust/Cargo.toml` + `[package.metadata.semio] role = "…"`) while
 * migration is in flight — see the discovery contract in master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`. A plugin discoverable under both
 * shapes at once is "in-flight", not an error. Flipped off in the W4 finalization wave once every
 * plugin/framework-module-family has migrated, at which point the legacy regexes are deleted outright. */
const LEGACY_LAYOUT_TOLERANT = true;

/** @emoji 🎛️ Single source of truth for the taxonomy tree's apps-container segment name, so anything
 * deriving an app-root path (the constitutional gate, example discovery) shares one literal instead of
 * duplicating it independently and drifting apart if the segment is ever renamed. */
const APPS_DIRNAME = "🎛️apps";

/** @emoji 🏛️ True when a manifest's `[package.metadata.semio]` block declares `role = "<role>"` — the
 * identity check for the new one-crate-per-plugin/framework-module-family taxonomy contract. */
function hasSemioRole(text: string, role: "plugin" | "framework"): boolean {
  const block = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[package.metadata.semio]")[0];
  if (!block) return false;
  return new RegExp(`^role\\s*=\\s*"${role}"\\s*$`, "m").test(block.join("\n"));
}

/** @emoji 🧭️ True when a manifest path matches the new one-crate-per-plugin taxonomy contract shape
 * (`✏️s/🔌️plugins/<p>/📦️packages/🦀️rust/Cargo.toml`) — the single source of truth for that shape,
 * shared by crate discovery (`findPluginCargoFiles`) and in-flight playground dedupe
 * (`dedupeInFlightPlaygroundEntries`) so the two never drift apart. */
function isNewContractPluginManifestPath(path: string): boolean {
  return /\/✏️s\/🔌️plugins\/[^/]+\/📦️packages\/🦀️rust\/Cargo\.toml$/.test(path);
}
//#endregion 🏛️DiscoveryContract

function findPluginCargoFiles(root: string): string[] {
  const out: string[] = [];
  function walk(dir: string) {
    for (const name of readdirSync(dir)) {
      // 🚫️ Skip build/vendor noise and any dot-directory (e.g. `.claude/worktrees/…`), which used to
      // leak duplicate registry rows for every crate that also exists inside a worktree checkout.
      if (name.startsWith(".") || name === "node_modules" || name === "🤖️generated" || name === "target") continue;
      const path = join(dir, name);
      let st: ReturnType<typeof statSync>;
      try {
        st = statSync(path);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        walk(path);
      } else if (name === "Cargo.toml" && !path.includes("/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/")) {
        const isModuleCrate = /\/🔨️modules\/[^/]+\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(path);
        // 🏛️ Contribution bundle crate: `✏️s/🔌️plugins/<p>/🧩️extensions/<e>/⚡️implementations/🦀️rust/Cargo.toml` (new node/document
        // types the plugin contributes, e.g. imperative's control/logic/math/text).
        const isExtensionCrate = /\/✏️s\/🔌️plugins\/[^/]+\/🧩️extensions\/[^/]+\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(path);
        // 🏛️ Plugin bundle crate lives at `✏️s/🔌️plugins/<p>/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/Cargo.toml`.
        const isPluginBundleCrate = /\/✏️s\/🔌️plugins\/[^/]+\/🛂️manifest\/🗿️artifact\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(path);
        if (isModuleCrate || isExtensionCrate || isPluginBundleCrate) {
          out.push(path);
          continue;
        }
        if (LEGACY_LAYOUT_TOLERANT) {
          // 🏛️ New taxonomy contract crates, identified by `[package.metadata.semio] role` rather than
          // by path shape. Framework crates are discovered the same way but never carry
          // `[package.metadata.component]`, so `tryParsePluginCargo` drops them below — matching how
          // ordinary (non-plugin) module crates already get silently filtered out of the registry today.
          const isNewPluginCrate = isNewContractPluginManifestPath(path);
          const isNewFrameworkCrate = !isNewPluginCrate && /\/🧰️framework\/.*\/📦️packages\/🦀️rust\/Cargo\.toml$/.test(path);
          if (isNewPluginCrate || isNewFrameworkCrate) {
            const text = readFileSync(path, "utf8");
            if (hasSemioRole(text, isNewPluginCrate ? "plugin" : "framework")) out.push(path);
          }
        }
      }
    }
  }
  walk(root);
  return out.sort();
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
  // 🏛️ Under `✏️s/🔌️plugins/<p>/...` the tech root is the plugin folder (3 segments), not `✏️s` itself.
  const techRoot = segments[0] === "✏️s" && segments[1] === "🔌️plugins" ? segments.slice(0, 3).join("/") : segments[0];
  const rootDir = join(repoRoot, techRoot, "📚️examples");
  if (existsSync(rootDir)) return idsIn(rootDir);
  if (variant.startsWith(pluginId) && variant.length > pluginId.length) {
    const suffix = variant.slice(pluginId.length);
    // 🎛️ Uses the shared `APPS_DIRNAME` constant (not an independently hardcoded literal) so this
    // keeps resolving correctly once a plugin's crate path collapses to one crate per plugin — the
    // apps-container segment name is single-sourced against the constitutional gate below.
    const dir = join(repoRoot, techRoot, APPS_DIRNAME, suffix, "📚️examples");
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
/** @emoji 🧹️ In-flight migration dedupe (see `LEGACY_LAYOUT_TOLERANT`): when a plugin's legacy bundle
 * crate and its new-contract taxonomy crate are BOTH on disk at once, they typically carry identical
 * `[[package.metadata.semio.playground]]` rows — same variant id, same aliases, same ports — which
 * would otherwise trip `validatePlaygroundRegistry`'s duplicate checks for the entire workspace on
 * every in-flight migration. Groups raw entries by variant id; a group where every entry shares one
 * `pluginId` AND at least one (but not all) entries come from a new-contract crate is the expected
 * transient migration shape, so only the new-contract entry/entries survive. A variant collision
 * across DIFFERENT plugin ids is left untouched — that is a genuine naming collision for
 * `validatePlaygroundRegistry` to catch. */
function dedupeInFlightPlaygroundEntries(playgrounds: readonly PlaygroundEntry[]): PlaygroundEntry[] {
  const byVariant = new Map<string, PlaygroundEntry[]>();
  for (const entry of playgrounds) byVariant.set(entry.variant, [...(byVariant.get(entry.variant) ?? []), entry]);
  const dropped = new Set<PlaygroundEntry>();
  for (const group of byVariant.values()) {
    if (group.length <= 1) continue;
    if (new Set(group.map((entry) => entry.pluginId)).size > 1) continue;
    // 🏛️ `cratePath` is repo-root-relative (no leading slash) while `isNewContractPluginManifestPath`
    // matches an absolute-shaped suffix (`/✏️s/🔌️plugins/…`) — prefix with "/" so the shared regex
    // still matches instead of duplicating it in a relative-path form.
    const newContract = group.filter((entry) => isNewContractPluginManifestPath(`/${entry.cratePath}/Cargo.toml`));
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
  const playgrounds = dedupeInFlightPlaygroundEntries(rawPlaygrounds);
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
 * only enforces completeness for plugins that have already started the split. */
function validateConstitutionalCrates(repoRoot: string, migratedPluginIds: ReadonlySet<string> = new Set()): string[] {
  const errors: string[] = [];
  const pluginRoot = join(repoRoot, "✏️s", "🔌️plugins");
  if (!existsSync(pluginRoot)) return errors;
  for (const pluginName of readdirSync(pluginRoot).sort()) {
    // 🚦️ A plugin already discovered under the new one-crate-per-plugin taxonomy contract
    // (`findNewContractPluginRoots`) is validated by `validateTaxonomyTree` instead — it has
    // deliberately shed the seven legacy per-app crate slots, that is not a regression.
    if (migratedPluginIds.has(pluginName)) continue;
    const appRoot = join(pluginRoot, pluginName, APPS_DIRNAME);
    if (!existsSync(appRoot) || !statSync(appRoot).isDirectory()) continue;
    const isFlatSingleApp = existsSync(join(appRoot, "⚡️implementations", "🦀️rust", "Cargo.toml"));
    const appDirs = isFlatSingleApp ? [{ label: pluginName, dir: appRoot }] : readdirSync(appRoot)
      .filter((name) => statSync(join(appRoot, name)).isDirectory())
      .filter((name) => name !== "🤝️shared")
      .map((name) => ({ label: `${pluginName}/${name}`, dir: join(appRoot, name) }));
    for (const { label, dir } of appDirs) {
      const missing: string[] = [];
      if (!existsSync(join(dir, "⚡️implementations", "🦀️rust", "Cargo.toml"))) missing.push("rs");
      for (const slot of CONSTITUTIONAL_SLOTS) {
        if (!existsSync(join(dir, "🔨️modules", CONSTITUTIONAL_SLOT_DIRNAME[slot], "⚡️implementations", "🦀️rust", "Cargo.toml"))) missing.push(slot);
      }
      if (missing.length > 0) {
        errors.push(`✏️s/🔌️plugins/${label} is missing constitutional crate slot(s): ${missing.join(", ")} (expected 7: rs, ${CONSTITUTIONAL_SLOTS.join(", ")})`);
      }
    }
  }
  return errors;
}
//#endregion 🏛️ConstitutionalCrateGate

//#region 🗿️TaxonomyValidator
/** @emoji 🗿️ Every artifact node must carry all five taxonomy component slots (see the discovery
 * contract in master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`). */
const TAXONOMY_ARTIFACT_COMPONENTS = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"] as const;
/** @emoji 🪟️ A window dir may only contain these children, each itself a `🦀️component.rs` leaf. */
const TAXONOMY_WINDOW_CHILDREN = new Set(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options"]);
const TAXONOMY_LEAF_FILENAME = "🦀️component.rs";

/** @emoji 🧭️ Plugin roots discovered via the *new* taxonomy contract (`📦️packages/🦀️rust/Cargo.toml`
 * with `role = "plugin"`) — distinct from `findPluginCargoFiles`, which also matches the legacy
 * 7-crate shape. Empty until a plugin has migrated (W1+); the taxonomy validator below is a no-op
 * until then. */
function findNewContractPluginRoots(repoRoot: string): { pluginId: string; pluginRoot: string }[] {
  const roots: { pluginId: string; pluginRoot: string }[] = [];
  const pluginsDir = join(repoRoot, "✏️s", "🔌️plugins");
  if (!existsSync(pluginsDir)) return roots;
  for (const pluginId of readdirSync(pluginsDir).sort()) {
    const manifestPath = join(pluginsDir, pluginId, "📦️packages", "🦀️rust", "Cargo.toml");
    if (!existsSync(manifestPath)) continue;
    if (hasSemioRole(readFileSync(manifestPath, "utf8"), "plugin")) roots.push({ pluginId, pluginRoot: join(pluginsDir, pluginId) });
  }
  return roots;
}

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

/** @emoji 🚦️ Warn-only structural audit of one migrated plugin's taxonomy tree. Not wired to fail
 * `check` yet — every plugin is still on the legacy shape, so this only ever runs against
 * `findNewContractPluginRoots` results, which are empty until W1+. Promoted to a hard error in W4
 * finalization once every plugin has moved (replaces `validateConstitutionalCrates` at that point). */
function validateTaxonomyTree(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  const artifactsDir = join(pluginRoot, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    for (const component of TAXONOMY_ARTIFACT_COMPONENTS) {
      if (!existsSync(join(artifactsDir, artifact, component, TAXONOMY_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
      }
    }
  }

  // 🪟️ windows live under apps/<app>/modes/<mode>/windows/<w> and may only contain the fixed child set.
  const appsDir = join(pluginRoot, APPS_DIRNAME);
  for (const app of listDirs(appsDir)) {
    const modesDir = join(appsDir, app, "🎭️modes");
    for (const mode of listDirs(modesDir)) {
      const windowsDir = join(modesDir, mode, "🪟️windows");
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
  const taxonomyLeafParents = new Set<string>([...TAXONOMY_ARTIFACT_COMPONENTS, ...TAXONOMY_WINDOW_CHILDREN]);
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
  const libRsPath = join(pluginRoot, "📦️lib.rs");
  if (existsSync(libRsPath)) {
    const libText = readFileSync(libRsPath, "utf8");
    const declaredPaths = [...libText.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]);
    const declaredAbs = new Set(declaredPaths.map((p) => join(pluginRoot, p)));
    for (const file of componentFiles) {
      if (!declaredAbs.has(file)) findings.push(`${pluginId}: ${relative(pluginRoot, file)} is not declared by any #[path] in 📦️lib.rs`);
    }
    for (const p of declaredPaths) {
      if (p.endsWith(".rs") && !existsSync(join(pluginRoot, p))) findings.push(`${pluginId}: 📦️lib.rs declares #[path = "${p}"] but the file does not exist on disk`);
    }
  } else {
    findings.push(`${pluginId}: missing 📦️lib.rs at plugin root`);
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

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generatePluginRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, "🔣️plugins.json"), `${JSON.stringify(entries, null, 2)}\n`);
    writeFileSync(join(outDir, "🟦️plugins.ts"), emitTypeScript(entries));
    writeFileSync(join(outDir, "🔣️playgrounds.json"), `${JSON.stringify(playgrounds, null, 2)}\n`);
    writeFileSync(join(outDir, "🟦️playgrounds.ts"), emitPlaygroundsTypeScript(playgrounds));
    writeFileSync(join(outDir, "🦀️hosts.rs"), emitRustHosts(entries, playgrounds));
    writeFileSync(join(outDir, "🦀️artifacts.rs"), emitRustArtifacts(entries));
    console.log(`plugin registry catalog refreshed (${entries.length} plugin crates, ${playgrounds.length} playgrounds) -> ${outDir}`);
  }
}

/** @emoji 🔎️ Renders the catalog in memory and byte-compares it against `generated/*` — never writes (a lint/verify step must never let the auto-commit daemon land regenerated files). */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const entries = generatePluginRegistry(repoRoot);
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    const expected: Record<string, string> = {
      "🔣️plugins.json": `${JSON.stringify(entries, null, 2)}\n`,
      "🟦️plugins.ts": emitTypeScript(entries),
      "🔣️playgrounds.json": `${JSON.stringify(playgrounds, null, 2)}\n`,
      "🟦️playgrounds.ts": emitPlaygroundsTypeScript(playgrounds),
      "🦀️hosts.rs": emitRustHosts(entries, playgrounds),
      "🦀️artifacts.rs": emitRustArtifacts(entries),
    };
    const stale = Object.entries(expected)
      .filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content)
      .map(([name]) => name);
    if (stale.length > 0) {
      console.error(`plugin registry catalog is stale: ${stale.map((name) => `generated/${name}`).join(", ")}`);
      console.error("run `bun nx run @semio-tech/plugin-registry:generate` to refresh.");
      process.exit(1);
    }
    const newContractPluginRoots = findNewContractPluginRoots(repoRoot);
    const migratedPluginIds = new Set(newContractPluginRoots.map(({ pluginId }) => pluginId));
    const violations = [...validatePlaygroundRegistry(playgrounds), ...validatePlaygroundSessions(repoRoot), ...validateConstitutionalCrates(repoRoot, migratedPluginIds)];
    if (violations.length > 0) {
      console.error("plugin registry catalog has playground validation errors:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    // 🗿️ Taxonomy tree audit for plugins already discovered via the new one-crate-per-plugin contract
    // (see `findNewContractPluginRoots`). Warn-only during migration — promoted to a hard failure in
    // W4 finalization once every plugin has moved off the legacy 7-crate shape.
    if (LEGACY_LAYOUT_TOLERANT) {
      const taxonomyFindings = newContractPluginRoots.flatMap(({ pluginId, pluginRoot }) => validateTaxonomyTree(pluginRoot, pluginId));
      if (taxonomyFindings.length > 0) {
        console.warn("plugin taxonomy tree findings (in-flight plugins — not failing the gate yet):");
        for (const finding of taxonomyFindings) console.warn(`  - ${finding}`);
      }
    }
    console.log(`plugin registry catalog is fresh (${entries.length} plugin crates, ${playgrounds.length} playgrounds).`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("check", CheckScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
}
