import catalog from "./🗺️catalog.json";
import schema from "./📐️schema.json";
import routes from "./🛣️routes.json";
import { installationDirectoryEmoji } from "../../../🧩️extension/🟦️.ts";

export type ModuleDirectory = { readonly pluginId: string; readonly directoryName: string };
export type ModuleRoutes = { readonly plugin: string; readonly extension: string };

const idSpec = schema.properties.modules.items.properties.pluginId;
const idPattern = new RegExp(idSpec.pattern, "u");

/** 🛣️Admits only the two explicitly selected distribution route owners. */
export function parseModuleRoutes(input: unknown): ModuleRoutes {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Invalid module routes");
  const value = input as Record<string, unknown>, properties = schema.definitions.moduleRoutes.properties;
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
  if (Object.keys(value).sort().join(",") !== "modules,version" || value.version !== 1 || !Array.isArray(value.modules) || value.modules.length < 1 || value.modules.length > 256) throw new Error("Invalid module deployment catalog fields");
  const ids = new Set<string>(), emojis = new Set<string>();
  return Object.freeze(value.modules.map((entry): ModuleDirectory => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).sort().join(",") !== "directoryName,pluginId") throw new Error("Invalid module deployment row");
    if (typeof entry.pluginId !== "string" || entry.pluginId.length > idSpec.maxLength || !idPattern.test(entry.pluginId)) throw new Error("Invalid public module identity");
    const emoji = installationDirectoryEmoji(entry.directoryName);
    if (ids.has(entry.pluginId) || emojis.has(emoji)) throw new Error("Duplicate module identity or sibling emoji");
    ids.add(entry.pluginId);
    emojis.add(emoji);
    return Object.freeze({ pluginId: entry.pluginId, directoryName: entry.directoryName });
  }));
}

export const MODULE_DIRECTORIES = parseModuleDirectories(catalog);
export const MODULE_BRIDGE_FILE = "🌉️bridge.js";
export const MODULE_VENDOR_DIRECTORY = "🪞️vendor";
export const MODULE_SHARD_DIRECTORY = "🧵️shard";
export const MODULE_HOT_SWAP_FILE = "♻️hot-swap.json";

/** 🚚️Selects only declared physical module directories for a production copy. */
export function moduleStaticDirectoryNames(pluginId: string, hostMode: boolean): readonly string[] | undefined {
  const directoryName = moduleDirectoryName(pluginId);
  return hostMode ? undefined : [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, directoryName];
}

/** 🧭️Resolves only an explicitly declared physical directory for a public plugin ID. */
export function moduleDirectoryName(pluginId: string): string {
  const row = MODULE_DIRECTORIES.find((entry) => entry.pluginId === pluginId);
  if (!row) throw new Error(`No hand-authored module directory for ${JSON.stringify(pluginId)}`);
  return row.directoryName;
}

/** 🔎️Maps a materialized basename back to its declared public identity. */
export function moduleIdForDirectoryName(directoryName: string): string | undefined {
  return MODULE_DIRECTORIES.find((entry) => entry.directoryName === directoryName)?.pluginId;
}
