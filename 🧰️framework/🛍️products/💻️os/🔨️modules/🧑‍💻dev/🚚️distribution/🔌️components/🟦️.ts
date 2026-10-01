import { join } from "node:path";
import { parseInstallationDirectoryV1 } from "../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts";
import { MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE, MODULE_SHARD_DIRECTORY, MODULE_VENDOR_DIRECTORY, moduleDirectoryName } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import type { BrowserArtifactSource } from "../../../🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";
import { browserArtifactVitePlugin } from "../../../🔌️plugin/🌐️browser-bundle/📦️distribution/⚡️vite/🟦️.ts";
import { pluginModulesRootIn } from "../../♻️activation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

export type ProductionBrowserComponents = {
  readonly version: 1;
  readonly profile: "dev" | "release";
  readonly components: readonly { readonly pluginId: string; readonly role: "plugin" | "extension"; readonly cratePath: string; readonly directoryName: string }[];
};
const componentSchema = schema.properties.components.items.properties;
const idPattern = new RegExp(componentSchema.pluginId.pattern, "u"), pathPattern = new RegExp(componentSchema.cratePath.pattern, "u"), directoryPattern = new RegExp(componentSchema.directoryName.pattern, "u");

/** 🧩️ Resolves the prepared session through the catalog's exact identities and role routes. */
export function selectProductionBrowserComponents(input: unknown, variant: string, registryPluginId: string, catalog: ProductionBrowserComponents["components"]): ProductionBrowserComponents {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Missing prepared production session");
  const session = input as { variant?: unknown; registryPluginId?: unknown; plugins?: unknown };
  if (session.variant !== variant || session.registryPluginId !== registryPluginId || !Array.isArray(session.plugins) || !session.plugins.length) throw new Error("Prepared production session identity mismatch");
  const admittedCatalog = catalog.map(component => ({ ...component, directoryName: parseInstallationDirectoryV1(component.directoryName) }));
  const selected = new Set<string>();
  const components = session.plugins.map((row: { pluginId?: string; moduleUrl?: string }) => {
    const component = row && admittedCatalog.find(entry => entry.pluginId === row.pluginId);
    if (!component || selected.has(component.pluginId) || row.moduleUrl !== `${component.role === "extension" ? MODULE_EXTENSION_ROUTE : MODULE_PLUGIN_ROUTE}/${moduleDirectoryName(component.pluginId, admittedCatalog)}/🌉️bridge.js`) throw new Error(`Invalid prepared production component: ${row?.pluginId}`);
    selected.add(component.pluginId);
    return { pluginId: component.pluginId, role: component.role, cratePath: component.cratePath, directoryName: component.directoryName };
  });
  return { version: 1, profile: "release", components };
}

/** 📦️ Resolves immutable component, browser-support and font owners for one production closure. */
export function productionBrowserSources(workspace: string, input: unknown): readonly BrowserArtifactSource[] {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid production browser components");
  const plan = input as ProductionBrowserComponents;
  if (Object.keys(plan).sort().join() !== "components,profile,version" || plan.version !== 1 || !schema.properties.profile.enum.includes(plan.profile) || !Array.isArray(plan.components) || plan.components.length < 1 || plan.components.length > 256) throw new Error("Invalid production browser component fields");
  const base = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules"), modules = pluginModulesRootIn(workspace, plan.profile);
  const pluginRoute = MODULE_PLUGIN_ROUTE.slice(1), extensionRoute = MODULE_EXTENSION_ROUTE.slice(1), shimDirectory = pluginRoute + "/" + PREVIEW2_VENDOR_RELATIVE;
  const sources: BrowserArtifactSource[] = [
    { root: join(modules, PREVIEW2_VENDOR_RELATIVE), destination: shimDirectory, owner: `browser-support:${plan.profile}:preview2` },
    { root: join(modules, MODULE_SHARD_DIRECTORY), destination: pluginRoute + "/" + MODULE_SHARD_DIRECTORY, owner: `browser-support:${plan.profile}:shard` },
    { root: join(base, "♾️infinite/📦️packages/🦀️rust/dist/fonts"), destination: pluginRoute + "/" + MODULE_VENDOR_DIRECTORY, owner: "infinite:fonts" },
  ];
  const admittedComponents = plan.components.map(component => ({ ...component, directoryName: parseInstallationDirectoryV1(component.directoryName) }));
  const ids = new Set<string>();
  for (const component of plan.components) {
    if (!component || typeof component !== "object" || Object.keys(component).sort().join() !== "cratePath,directoryName,pluginId,role") throw new Error("Invalid production browser component");
    if (typeof component.pluginId !== "string" || component.pluginId.length > componentSchema.pluginId.maxLength || !idPattern.test(component.pluginId) || !componentSchema.role.enum.includes(component.role)) throw new Error("Invalid production browser component identity");
    if (typeof component.cratePath !== "string" || !pathPattern.test(component.cratePath) || component.cratePath.normalize("NFC") !== component.cratePath) throw new Error("Invalid production browser component path");
    if (ids.has(component.pluginId)) throw new Error(`Duplicate production browser component: ${component.pluginId}`);
    ids.add(component.pluginId);
    if (typeof component.directoryName !== "string" || component.directoryName.length > componentSchema.directoryName.maxLength || !directoryPattern.test(component.directoryName) || component.directoryName.normalize("NFC") !== component.directoryName) throw new Error("Invalid production browser component directory");
    const directory = moduleDirectoryName(component.pluginId, admittedComponents);
    sources.push({ root: join(modules, directory), destination: (component.role === "extension" ? extensionRoute : pluginRoute) + "/" + directory, owner: component.cratePath + `/Cargo.toml:browser:${plan.profile}`, shimDirectory });
  }
  return sources;
}

/** 🚚️ Copies declared runtime files only when Vite writes its production bundle. */
export function productionBrowserArtifactsVitePlugin(workspace: string, plan: ProductionBrowserComponents) {
  return { ...browserArtifactVitePlugin(productionBrowserSources(workspace, plan)), name: "semio-production-browser-artifacts" };
}
