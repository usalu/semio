import { admitPluginCatalogV1, pluginCatalogV1, type PluginCatalogAdmissionV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, pluginModuleUrl, extensionModuleUrl } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

/** 🧩️ Specific application assembly owns and admits its generated installed inventory. */
export function composeSpecificOsCatalogV1(baseUrl: string, control: PluginCatalogAdmissionV1) {
  const installed = new Set([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => row.pluginId));
  const rows = admitPluginCatalogV1({
    version: 1,
    targets: [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => ({ pluginId: row.pluginId, wasmOut: row.wasmOut, role: row.role, capabilities: row.capabilities, ...(row.role === "extension" ? { extends: row.extends } : {}), contributes: row.contributes, consumes: row.consumes, dependsOn: row.dependsOn, activationEvents: row.activationEvents, moduleUrl: new URL(row.role === "extension" ? extensionModuleUrl(row.pluginId) : pluginModuleUrl(row.pluginId), baseUrl).href })),
    hosts: PLUGIN_HOST_CONFIGS.map(row => ({ pluginId: row.pluginId, landingAppId: row.landingAppId, hostAppId: row.hostAppId })),
    playgrounds: PLAYGROUND_BUILD_TARGETS.filter(row => installed.has(row.pluginId)).map(row => ({ variant: row.variant, pluginId: row.pluginId, aliases: row.aliases, ...(row.app === undefined ? {} : { app: row.app }) })),
  }, control);
  return { rows, catalog: pluginCatalogV1(rows) };
}
