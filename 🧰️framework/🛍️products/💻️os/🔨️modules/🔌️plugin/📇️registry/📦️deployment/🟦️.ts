import schemaModule from "./🧬️schema/🔣️.json";
import routes from "./🛣️routes.json";
import { parseInstallationDirectoryV1, installationDirectoryEmoji, type InstallationDirectoryV1 } from "../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts";
import "../../../../../../🔨️modules/🖼️assets/🌐️published-page/🟦️.ts";

export type ModuleDirectory = { readonly pluginId: string; readonly directoryName: InstallationDirectoryV1 };
export type ModuleRoutes = { readonly plugin: string; readonly extension: string };

const schema = schemaModule.$defs.DeploymentCatalogV1;
const idSpec = schema.properties.modules.items.properties.pluginId;
const idPattern = new RegExp(idSpec.pattern, "u");

/** 🧩️ The owner of a component crate: a component crate sits at `<owner>/🧩️component/📦️packages/🦀️rust` beside its library at
 * `<owner>/📦️packages/🦀️rust`, so its sources (library, component root and committed descriptor pair) are everything under `<owner>`. */
export function componentOwnerRootV1(cratePath: string): string {
  const parts = cratePath.split("/");
  if (parts.length < 4 || parts.at(-1) !== "🦀️rust" || parts.at(-2) !== "📦️packages" || parts.at(-3) !== "🧩️component") throw new Error(`Component crate ${cratePath} lies outside a 🧩️component facet`);
  return parts.slice(0, -3).join("/");
}

/** 🛣️Admits only the two explicitly selected distribution route owners. */
export function parseModuleRoutes(input: unknown): ModuleRoutes {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid module routes");
  const value = input as Record<string, unknown>, properties = schemaModule.$defs.DeploymentModuleRoutesV1.properties;
  if (Object.keys(value).sort().join(",") !== "extension,plugin" || value.plugin !== properties.plugin.const || value.extension !== properties.extension.const) throw new Error("Module routes must match their exact schema authority");
  return Object.freeze({ plugin: value.plugin as string, extension: value.extension as string });
}

export const MODULE_ROUTES = parseModuleRoutes(routes);
export const MODULE_PLUGIN_ROUTE = MODULE_ROUTES.plugin;
export const MODULE_EXTENSION_ROUTE = MODULE_ROUTES.extension;

/** 🚏️Decodes one canonical module request path without accepting obsolete routes or traversal aliases. */
export function moduleRoutePath(rawUrl: string): string | null {
  const encoded = rawUrl.split(/[?#]/, 1)[0] ?? "";
  if (/%(?:2f|5c)/iu.test(encoded)) return null;
  let path: string;
  try { path = decodeURIComponent(encoded); } catch { return null; }
  if (/[\\\u0000-\u001F\u007F]/u.test(path) || path.split("/").slice(1).some((part) => part === "" || part === "." || part === "..")) return null;
  return Object.values(MODULE_ROUTES).some((route) => path === route || path.startsWith(route + "/")) ? path : null;
}

/** 📦️Validates the hand-authored deployment authority without deriving any name from an ID. */
export function parseModuleDirectories(input: unknown): readonly ModuleDirectory[] {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid module deployment catalog");
  const value = input as Record<string, unknown>;
  if (Object.keys(value).sort().join(",") !== "modules,version" || value.version !== 1 || !Array.isArray(value.modules) || value.modules.length > 256) throw new Error("Invalid module deployment catalog fields");
  const ids = new Set<string>(), emojis = new Set<string>();
  return Object.freeze(value.modules.map((entry): ModuleDirectory => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).sort().join(",") !== "directoryName,pluginId") throw new Error("Invalid module deployment row");
    if (typeof entry.pluginId !== "string" || entry.pluginId.length > idSpec.maxLength || !idPattern.test(entry.pluginId)) throw new Error("Invalid public module identity");
    const directoryName = parseInstallationDirectoryV1(entry.directoryName), emoji = installationDirectoryEmoji(directoryName);
    if (ids.has(entry.pluginId) || emojis.has(emoji)) throw new Error("Duplicate module identity or sibling emoji");
    ids.add(entry.pluginId);
    emojis.add(emoji);
    return Object.freeze({ pluginId: entry.pluginId, directoryName });
  }));
}

export const MODULE_BRIDGE_FILE = "🌉️bridge.js";
export const MODULE_VENDOR_DIRECTORY = "🪞️vendor";
export const MODULE_SHARD_DIRECTORY = "🧵️shard";
export const MODULE_HOT_SWAP_FILE = "♻️hot-swap.json";

/** 🚚️Selects only declared physical module directories for a production copy. */
export function moduleStaticDirectoryNames(pluginId: string, hostMode: boolean, inventory: readonly ModuleDirectory[]): readonly string[] | undefined {
  const directoryName = moduleDirectoryName(pluginId, inventory);
  return hostMode ? undefined : [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, directoryName];
}

/** 🧭️Resolves only an explicitly declared physical directory for a public plugin ID. */
export function moduleDirectoryName(pluginId: string, inventory: readonly ModuleDirectory[]): InstallationDirectoryV1 {
  const matches = inventory.filter((entry) => entry.pluginId === pluginId);
  if (matches.length !== 1) throw new Error(`Expected one declared module directory for ${JSON.stringify(pluginId)}`);
  const row = matches[0]!;
  installationDirectoryEmoji(row.directoryName);
  return row.directoryName;
}

/** 🔎️Maps a materialized basename back to its declared public identity. */
export function moduleIdForDirectoryName(directoryName: string, inventory: readonly ModuleDirectory[]): string | undefined {
  const matches = inventory.filter((entry) => entry.directoryName === directoryName);
  if (matches.length > 1) throw new Error(`Repeated declared module directory ${JSON.stringify(directoryName)}`);
  return matches[0]?.pluginId;
}

/** 🔑️ The one `stageRepositoryArtifacts` ownership key of a component's module directory, whichever profile produced it. */
export function componentModuleOwnerV1(cratePath: string): string {
  return `${cratePath.split(/[\\/]/u).join("/")}/Cargo.toml:browser`;
}
