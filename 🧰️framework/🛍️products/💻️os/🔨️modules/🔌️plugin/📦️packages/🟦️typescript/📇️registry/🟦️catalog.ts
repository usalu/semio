// #region 🗂️PluginCatalog
/** @emoji 🗂️ `@semio-tech/framework-os` plugin package — builds the framework kernel's injected
 * `PluginCatalog` from this product's generated plugin/playground registry output
 * (`📇️registry/🤖️generated/🟦️plugins.ts` + `🤖️generated/🟦️playgrounds.ts`). This is the ONE place in
 * the codebase allowed to import that generated output on the kernel's behalf — the generic
 * `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` module must never import a specific product's build
 * artifacts directly; every caller of its `PluginCatalog`-taking resolvers imports `PLUGIN_CATALOG`
 * (or calls `buildPluginCatalog()`) from here instead. */
import type { PlaygroundCatalogTarget, PluginCatalog, PluginCatalogTarget } from "@semio-tech/framework";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, extensionModuleUrl, pluginModuleUrl } from "./🤖️generated/🟦️plugins.ts";
import { PLAYGROUND_BUILD_TARGETS } from "./🤖️generated/🟦️playgrounds.ts";

function toCatalogTarget(target: { readonly pluginId: string; readonly wasmOut: string; readonly role: "plugin" | "extension"; readonly contributes: readonly string[]; readonly consumes: readonly string[] }): PluginCatalogTarget {
  return { pluginId: target.pluginId, wasmOut: target.wasmOut, role: target.role, contributes: target.contributes, consumes: target.consumes };
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
