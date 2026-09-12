import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import type { RegistryCatalogInputView } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { discoverCatalogPackages, getWorkspaceRoot, registryExampleCatalog } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { generatePluginRegistry, parseTomlStringArray, readDescriptorJson, tomlBlocksAfterHeader, TAXONOMY, type GeneratePluginRegistryOptions, type PluginRegistryEntry } from "../../🔎️discovery/🟦️.ts";



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
  readonly catalog?: string;
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
  /** @emoji 👥️ Extra per-user dev ports for a multi-user collaborative session (e.g. hub-backed `s`
   * studio dev launchers) — one port per concurrent user, over and above the single-user `ports` row. */
  readonly userPorts?: { readonly react: readonly number[]; readonly wgpu: readonly number[] };
  readonly examples: readonly string[];
  /** @emoji 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** @emoji 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};


/** @emoji 🔢️ Every integer in a `key = [1, 2]` inline TOML array found inside `text` (used for
 * sub-blocks like `user_ports = { react = [...], wgpu = [...] }` where `react`/`wgpu` aren't at the
 * start of a line). */
export function parseTomlInlineNumberArray(text: string, key: string): number[] {
  const match = text.match(new RegExp(`${key}\\s*=\\s*\\[([^\\]]*)\\]`));
  if (!match) return [];
  return [...match[1].matchAll(/\d+/g)].map((m) => Number(m[0]));
}


export function parsePlaygroundBlock(block: string, pluginId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  const userPortsBlock = block.match(/^user_ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const userPortsReact = userPortsBlock ? parseTomlInlineNumberArray(userPortsBlock, "react") : [];
  const userPortsWgpu = userPortsBlock ? parseTomlInlineNumberArray(userPortsBlock, "wgpu") : [];
  const userPorts = userPortsReact.length > 0 && userPortsWgpu.length > 0 ? { react: userPortsReact, wgpu: userPortsWgpu } : undefined;
  const engines = parseTomlStringArray(block, "engines");
  return { variant, pluginId, cratePath, app, brand, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, ...(userPorts ? { userPorts } : {}), examples: [], engines, assets: [] };
}


/** @emoji 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate manifest. */
export function parseAssetsForCrate(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): AssetSpecRow[] {
  const path = relative(repoRoot, manifestPath).replaceAll("\\", "/");
  if (view ? view.kind(path) === null : !existsSync(manifestPath)) return [];
  const text = view ? view.readText(path) : readFileSync(manifestPath, "utf8");
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
    const catalog = block.match(/^catalog\s*=\s*"([^"]+)"/m)?.[1];
    if (kind === "mesh-collection") {
      for (const field of block.matchAll(/^([a-z_]+)\s*=/gm)) {
        if (!["kind", "route", "app", "catalog"].includes(field[1]!)) throw new Error(`Unknown mesh asset field ${field[1]} in ${path}`);
      }
    }
    rows.push({
      kind,
      route,
      ...(app ? { app } : {}),
      ...(upstream ? { upstream } : {}),
      ...(cache ? { cache } : {}),
      ...(root ? { root } : {}),
      ...(catalog ? { catalog } : {}),
    });
  }
  return rows;
}


/** @emoji ✂️ The variation selector that closes an emoji identity in `exampleSlugPattern` — everything after it is the example id. */
export const EXAMPLE_SLUG_IDENTITY_SEPARATOR = "\uFE0F";


/** @emoji 🪪️ The example ids one owner descriptor declares for a playground's app — every `manifest.examples` row when the playground names no app. */
export function declaredExampleIdsForPlayground(descriptor: Record<string, unknown> | undefined, app?: string): ReadonlySet<string> {
  const manifest = descriptor?.manifest as { examples?: unknown } | undefined;
  const rows = Array.isArray(manifest?.examples) ? (manifest.examples as unknown[]) : [];
  return new Set(
    rows
      .filter((row) => app === undefined || (row as { appId?: unknown }).appId === app)
      .map((row) => (row as { id?: unknown }).id)
      .filter((id): id is string => typeof id === "string"),
  );
}


/**
 * @emoji 🖼️ Example ids for one playground row: emoji-slug dirs under `🗿️artifacts/<a>/📚️examples/` and
 * every `👁️viewer`/`✏️editor` surface's `📚️examples/` that carry a definition leaf, narrowed to the
 * examples this playground's own app declares. One crate serves several apps (`puzzle` ships 2d/3d/5d
 * from one crate), so the membership scan alone advertises a sibling app's examples and test-only
 * surface fixtures; the owner descriptor is the app's own declaration, so it decides whenever it
 * carries one. The id of an example slug is its `exampleSlugPattern` tail — everything after the
 * emoji identity's `U+FE0F`.
 */
export function discoverExamplesForPlayground(repoRoot: string, cratePath: string, declared: ReadonlySet<string>, view?: RegistryCatalogInputView): string[] {
  const slugs = registryExampleCatalog(repoRoot, cratePath, TAXONOMY, view);
  return declared.size === 0 ? slugs : slugs.filter((slug) => declared.has(slug.slice(slug.lastIndexOf(EXAMPLE_SLUG_IDENTITY_SEPARATOR) + 1)));
}



export function parsePlaygroundsForCrate(manifestPath: string, pluginId: string, cratePath: string, repoRoot: string, view?: RegistryCatalogInputView): PlaygroundEntry[] {
  const text = view ? view.readText(relative(repoRoot, manifestPath).replaceAll("\\", "/")) : readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]");
  const entries: PlaygroundEntry[] = [];
  for (const block of blocks) {
    const entry = parsePlaygroundBlock(block.join("\n"), pluginId, cratePath);
    if (entry) entries.push(entry);
  }
  return entries;
}


/** @emoji 🕹️ Scans every plugin/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PlaygroundEntry[] {
  const entries = generatePluginRegistry(repoRoot, options);
  const playgrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath, repoRoot, options.view);
    const descriptor = readDescriptorJson(repoRoot, entry.cratePath, options.view);
    for (const playground of parsePlaygroundsForCrate(manifestPath, entry.pluginId, entry.cratePath, repoRoot, options.view)) {
      const assets = crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app);
      const declared = declaredExampleIdsForPlayground(descriptor, playground.app);
      playgrounds.push({ ...playground, examples: discoverExamplesForPlayground(repoRoot, entry.cratePath, declared, options.view), assets });
    }
  }
  for (let i = 0; i < playgrounds.length; i++) {
    const row = playgrounds[i];
    if (!row.brand || row.examples.length > 0) continue;
    const donor = playgrounds.find((other) => other !== row && other.cratePath === row.cratePath && other.app === row.app && other.examples.length > 0);
    if (donor) playgrounds[i] = { ...row, examples: donor.examples, engines: row.engines.length > 0 ? row.engines : donor.engines };
  }
  playgrounds.sort((a, b) => a.variant.localeCompare(b.variant));
  return playgrounds;
}




/** @emoji 🏠️ Resolves the one playground variant that boots as the host/shell session: the data-driven
 * replacement for the previous hardcoded `"s"` literal. Exactly one plugin crate in the catalog may
 * declare `[package.metadata.semio].host` (see `parsePluginCargo`'s `host`/`shell` parse) — this scans
 * for that crate and returns its own `[[package.metadata.semio.playground]]` variant id, throwing a
 * clear error if zero or more than one plugin crate declares the host table. */
export function resolveDefaultHostVariant(repoRoot = getWorkspaceRoot()): string {
  const packages = discoverCatalogPackages(repoRoot, TAXONOMY);
  return defaultHostVariant(generatePluginRegistry(repoRoot, { packages }), generatePlaygroundRegistry(repoRoot, { packages }));
}




/** 🏠️ One host identity resolved from the same already-rendered catalog rows. */
export function defaultHostVariant(entries: readonly PluginRegistryEntry[], playgrounds: readonly PlaygroundEntry[]): string {
  const hostEntries = entries.filter((entry) => entry.host !== undefined);
  if (hostEntries.length !== 1) {
    throw new Error(`📇️registry: expected exactly one plugin crate to declare [package.metadata.semio].host, found ${hostEntries.length}${hostEntries.length > 0 ? ` (${hostEntries.map((entry) => entry.pluginId).join(", ")})` : ""}`);
  }
  const hostPluginId = hostEntries[0].pluginId;
  const hostPlayground = playgrounds.find((entry) => entry.pluginId === hostPluginId);
  if (!hostPlayground) throw new Error(`📇️registry: host plugin "${hostPluginId}" declares no [[package.metadata.semio.playground]] variant`);
  return hostPlayground.variant;
}
