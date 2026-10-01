import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import type { DeployedRegistryEntryV1 } from "../🔎️discovery/🟦️.ts";
import { parseDeployedRegistryEntryV1 } from "../🔎️discovery/🧬️schema/🟦️.ts";
import type { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";
import { parseModuleDirectories, type ModuleDirectory } from "../📦️deployment/🟦️.ts";

export type GeneratedCatalogProjection = { readonly entries: readonly DeployedRegistryEntryV1[]; readonly playgrounds: readonly PlaygroundEntry[] };

export const GENERATED_PLUGINS_PROJECTION = "🔌️plugins.json";

export const GENERATED_PLAYGROUNDS_PROJECTION = "🎠️playgrounds.json";

/** 📦️Admits only deployment directories declared by the supplied registry owners. */
export function registryModuleDirectories(entries: readonly DeployedRegistryEntryV1[]): readonly ModuleDirectory[] {
  return parseModuleDirectories({ version: 1, modules: entries.map(({ pluginId, directoryName }) => ({ pluginId, directoryName })) });
}

/** 📖️ Reads the rows `generate` just projected into `🤖️generated` so one dev boot walks the repository once instead of once per consumer; the projection is the language-agnostic twin of [[generatePluginRegistry]] and [[generatePlaygroundRegistry]]. */
export function readGeneratedCatalogProjection(generatedDir = join(import.meta.dir, "..", "🤖️generated")): GeneratedCatalogProjection {
  const read = <T>(name: string): readonly T[] => {
    const parsed: unknown = JSON.parse(readFileSync(join(generatedDir, name), "utf8"));
    if (!Array.isArray(parsed)) throw new Error(`📇️registry: ${name} is not a projected row array`);
    return parsed as T[];
  };
  const entries = read<unknown>(GENERATED_PLUGINS_PROJECTION).map(parseDeployedRegistryEntryV1);
  registryModuleDirectories(entries);
  return { entries, playgrounds: read<PlaygroundEntry>(GENERATED_PLAYGROUNDS_PROJECTION) };
}

/** 🏠️ Host detection of projected rows: a variant, alias or bare plugin id whose crate declares
 * `[package.metadata.semio].host` and whose playground row names no `app`. A row that names one boots
 * that single artifact app standalone, even on the host crate (`🪐️space`'s Home and Space). */
export function projectedHostPluginFilter(projection: GeneratedCatalogProjection, pluginFilter?: string): boolean {
  if (!pluginFilter) return true;
  const variantRow = projection.playgrounds.find((row) => row.variant === pluginFilter || row.aliases.includes(pluginFilter));
  if (variantRow?.app !== undefined) return false;
  const pluginId = variantRow?.pluginId ?? pluginFilter;
  return projection.entries.some((entry) => entry.pluginId === pluginId && entry.host !== undefined);
}

/** 🎯️ Applies registry filter semantics to already projected rows. */
export function filterProjectedPluginRegistry(projection: GeneratedCatalogProjection, filterPlaygroundPlugin?: string): DeployedRegistryEntryV1[] {
  const entries = [...projection.entries].sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  if (!filterPlaygroundPlugin || projectedHostPluginFilter(projection, filterPlaygroundPlugin)) return entries;
  const { resolveRegistryPluginIdsForFilter } = loadDiscovery();
  const ids = new Set(resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin, projection.entries, projection.playgrounds));
  return entries.filter((entry) => ids.has(entry.pluginId));
}

const requireDiscovery = createRequire(import.meta.url);
function loadDiscovery(): { resolveRegistryPluginIdsForFilter: (filter: string, entries: readonly DeployedRegistryEntryV1[], playgrounds: GeneratedCatalogProjection["playgrounds"]) => readonly string[] } {
  return requireDiscovery('../🔎️discovery/🟦️.ts');
}
