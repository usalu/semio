import { declaredPlaygroundCatalogDefaultV1 } from "../⭐️default/🟦️.ts";
import { parseTileProxyAssetSpecV1 } from "../../../../../../../🔨️modules/🖼️assets/🗺️tile-proxy/🟦️.ts";
import {admitPlaygroundNativeHostV1,parsePlaygroundNativeHostV1,nativeHostFilesystemViewV1,type PlaygroundNativeHostV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { declaredLaunchNamePrefix } from "../../🚀️launch/🏷️name-prefix/🧬️schema/🟦️.ts";
import { join, relative } from "node:path";
import type { RegistryCatalogInputView } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { getWorkspaceRoot, registryCatalogInputView, registryExampleCatalog } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { generatePluginRegistry, parseTomlStringArray, readDescriptorJson, tomlBlocksAfterHeader, TAXONOMY, type GeneratePluginRegistryOptions, type RegistryChannelDiagnosticV1 } from "../../🔎️discovery/🟦️.ts";



//#region 🔖️PlaygroundEntry
/** 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
 * plugin crate. `app` optionally scopes the row to one playground variant of a multi-app crate (unset
 * ⇒ every variant of the crate). Mirrors the TS discriminated union emitted for consumers as
 * `AssetDeliveryDeclarationV1` (see `emitPlaygroundsTypeScript`). */
export type AssetSpecRow = {
  readonly kind: "tile-proxy" | "static-dir" | "mesh-collection";
  readonly route: string;
  readonly app?: string;
  readonly upstream?: string;
  readonly cache?: string;
  readonly userAgent?: string;
  readonly root?: string;
  readonly catalog?: string;
};


/** 🎮️ One `[[package.metadata.semio.playground]]` row scoped to its owning plugin crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly catalogDefault?: boolean;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** 🏷️ Shell brand id (see `framework/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly launchNamePrefix?: string;
  readonly devContribution?: string;
  readonly nativeHost?: PlaygroundNativeHostV1;
  readonly mcpHost?: PlaygroundNativeHostV1;
  /** 📦️ Repo-root-relative CDN output directory for `build-<variant>-react-release` instead of framework-os-dev `dist/build-…`. */
  readonly distDir?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  /** 👥️ Extra per-user dev ports for a multi-user collaborative session (e.g. hub-backed `s`
   * studio dev launchers) — one port per concurrent user, over and above the single-user `ports` row. */
  readonly userPorts?: { readonly react: readonly number[]; readonly wgpu: readonly number[] };
  readonly examples: readonly string[];
  /** 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};


/** 🔢️ Every integer in a `key = [1, 2]` inline TOML array found inside `text` (used for
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
  const catalogDefault = declaredPlaygroundCatalogDefaultV1(block);
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const launchNamePrefix = declaredLaunchNamePrefix(block);
  const devContribution = block.match(/^devContribution\s*=\s*"([^"]+)"/m)?.[1];
  const distDir = block.match(/^distDir\s*=\s*"([^"]+)"/m)?.[1];
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
  const nativeHost = parsePlaygroundNativeHostV1(block) as PlaygroundNativeHostV1 | undefined;
  const mcpHost = parsePlaygroundNativeHostV1(block,"mcpHost") as PlaygroundNativeHostV1 | undefined;
  return { variant, ...(catalogDefault === undefined ? {} : { catalogDefault }), pluginId, cratePath, app, brand, ...(launchNamePrefix === undefined ? {} : { launchNamePrefix }), devContribution, distDir, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, ...(userPorts ? { userPorts } : {}), examples: [], engines, assets: [], ...(nativeHost ? {nativeHost} : {}),...(mcpHost ? {mcpHost} : {}) };
}


/** 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate manifest. */
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
    if (kind === "tile-proxy") {
      const authored = Bun.TOML.parse(block) as Record<string, unknown>;
      const { app, ...transport } = authored;
      const spec = parseTileProxyAssetSpecV1(transport);
      if (app !== undefined && (typeof app !== "string" || app.length === 0)) throw Error(`Invalid tile asset app in ${path}`);
      rows.push({ ...spec, ...(app === undefined ? {} : { app: app as string }) });
      continue;
    }
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


/** ✂️ The variation selector that closes an emoji identity in `exampleSlugPattern` — everything after it is the example id. */
export const EXAMPLE_SLUG_IDENTITY_SEPARATOR = "\uFE0F";


/** 🪪️ The example ids one owner descriptor declares for a playground's app — every `manifest.examples` row when the playground names no app. */
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
 * 🖼️ Example ids for one playground row: emoji-slug dirs under `🗿️artifacts/<a>/📚️examples/` and
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
    if (entry) entries.push({...entry,...(entry.nativeHost?{nativeHost:admitPlaygroundNativeHostV1(entry.nativeHost,view??nativeHostFilesystemViewV1(repoRoot))}:{}),...(entry.mcpHost?{mcpHost:admitPlaygroundNativeHostV1(entry.mcpHost,view??nativeHostFilesystemViewV1(repoRoot))}:{})});
  }
  return entries;
}


/** 🕹️ Scans every plugin/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PlaygroundEntry[] {
  const view = options.view ?? registryCatalogInputView(repoRoot, TAXONOMY);
  const entries = generatePluginRegistry(repoRoot, { ...options, view });
  const playgrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath, repoRoot, view);
    const descriptor = readDescriptorJson(repoRoot, entry.cratePath, view);
    for (const playground of parsePlaygroundsForCrate(manifestPath, entry.pluginId, entry.cratePath, repoRoot, view)) {
      const assets = crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app);
      const declared = declaredExampleIdsForPlayground(descriptor, playground.app);
      playgrounds.push({ ...playground, examples: discoverExamplesForPlayground(repoRoot, entry.cratePath, declared, view), assets });
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


/** 🚀️ Source-only playground rows of withheld plugins: launch rows stay stable while the dev catalog withholds a stale-channel plugin. */
export function generateWithheldPlaygroundRegistry(repoRoot: string, diagnostics: readonly RegistryChannelDiagnosticV1[], view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, TAXONOMY)): PlaygroundEntry[] {
  return diagnostics.flatMap(({ pluginId, cratePath }) => {
    const manifestPath = join(repoRoot, cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath, repoRoot, view);
    return parsePlaygroundsForCrate(manifestPath, pluginId, cratePath, repoRoot, view).map((playground) => ({ ...playground, examples: [], assets: crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app) }));
  });
}
