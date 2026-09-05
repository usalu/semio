// #region 🗂️PluginCatalog
/** @emoji 🗂️ `@semio-tech/framework-os` plugin package — builds the framework kernel's injected
 * `PluginCatalog` from this product's generated plugin/playground registry output
 * (`📇️registry/🤖️generated/🧩️plugins.ts` + `🤖️generated/🎮️playgrounds.ts`). This is the ONE place in
 * the codebase allowed to import that generated output on the kernel's behalf — the generic
 * `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` module must never import a specific product's build
 * artifacts directly; every caller of its `PluginCatalog`-taking resolvers imports `PLUGIN_CATALOG`
 * (or calls `buildPluginCatalog()`) from here instead. */
import type { PlaygroundCatalogTarget, PluginCatalog, PluginCatalogTarget } from "@semio-tech/framework";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, extensionModuleUrl, pluginModuleUrl } from "./🤖️generated/🧩️plugins.ts";
import { PLAYGROUND_BUILD_TARGETS } from "./🤖️generated/🎮️playgrounds.ts";

function toCatalogTarget(target: {
  readonly pluginId: string;
  readonly wasmOut: string;
  readonly role: "plugin" | "extension";
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  readonly dependsOn?: readonly string[];
}): PluginCatalogTarget {
  return { pluginId: target.pluginId, wasmOut: target.wasmOut, role: target.role, contributes: target.contributes, consumes: target.consumes, dependsOn: target.dependsOn };
}

function toPlaygroundCatalogTarget(target: { readonly variant: string; readonly pluginId: string; readonly app?: string; readonly aliases: readonly string[] }): PlaygroundCatalogTarget {
  return { variant: target.variant, pluginId: target.pluginId, app: target.app, aliases: target.aliases };
}

/** 🗂️ Builds a fresh {@link PluginCatalog} from this product's generated plugin/playground registry. */
export function buildPluginCatalog(): PluginCatalog {
  return {
    plugins: PLUGIN_BUILD_TARGETS.map(toCatalogTarget),
    extensions: EXTENSION_TARGETS.map(toCatalogTarget),
    hosts: PLUGIN_HOST_CONFIGS,
    playgrounds: PLAYGROUND_BUILD_TARGETS.map(toPlaygroundCatalogTarget),
    moduleUrl: pluginModuleUrl,
    extensionModuleUrl,
  };
}

/** 🗂️ Ready-built singleton — every caller in this product shares the same generated catalog rows. */
export const PLUGIN_CATALOG: PluginCatalog = buildPluginCatalog();
// #endregion 🗂️PluginCatalog

// #region 🏠️HostPlaygroundFilter
/**
 * 🏠️ Whether a raw playground filter (a variant, one of its aliases, or a bare crate pluginId)
 * resolves to a plugin crate declaring `[package.metadata.semio].host` — the studio-hub case every
 * consumer treats as "unfiltered". Pure lookup over the two generated registry modules, with the
 * identical resolution order as `projectedHostPluginFilter` in `📇️registry/📜️script.ts` (variant or
 * alias first, else the filter read as a bare plugin id) and the identical host predicate
 * (`host !== undefined` on the plugin row); `🧫️fixtures/🏠️host-filter.json` is the shared vector
 * pinning the two together. `⚙️vite.config.ts` mounts this one because it costs two array scans over
 * already-generated rows, where the script twin costs the repository walk behind `getWorkspaceRoot`.
 */
export function isHostPlaygroundFilter(
  pluginFilter?: string,
  playgrounds: readonly { readonly variant: string; readonly pluginId: string; readonly aliases: readonly string[] }[] = PLAYGROUND_BUILD_TARGETS,
  targets: readonly { readonly pluginId: string; readonly host?: unknown }[] = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS],
): boolean {
  if (!pluginFilter) return true;
  const variantRow = playgrounds.find((row) => row.variant === pluginFilter || row.aliases.includes(pluginFilter));
  const pluginId = variantRow?.pluginId ?? pluginFilter;
  return targets.some((target) => target.pluginId === pluginId && target.host !== undefined);
}
// #endregion 🏠️HostPlaygroundFilter
