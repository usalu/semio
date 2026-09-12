import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type PluginRegistryEntry, resolveRegistryPluginIdsForFilter } from "../🔎️discovery/🟦️.ts";
import type { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";

export type GeneratedCatalogProjection = { readonly entries: readonly PluginRegistryEntry[]; readonly playgrounds: readonly PlaygroundEntry[] };

export const GENERATED_PLUGINS_PROJECTION = "🔌️plugins.json";

export const GENERATED_PLAYGROUNDS_PROJECTION = "🎠️playgrounds.json";

/** 📖️ Reads the rows `generate` just projected into `🤖️generated` so one dev boot walks the repository once instead of once per consumer; the projection is the language-agnostic twin of [[generatePluginRegistry]] and [[generatePlaygroundRegistry]]. */
export function readGeneratedCatalogProjection(generatedDir = join(import.meta.dir, "..", "🤖️generated")): GeneratedCatalogProjection {
  const read = <T>(name: string): readonly T[] => {
    const parsed: unknown = JSON.parse(readFileSync(join(generatedDir, name), "utf8"));
    if (!Array.isArray(parsed)) throw new Error(`📇️registry: ${name} is not a projected row array`);
    return parsed as T[];
  };
  return { entries: read<PluginRegistryEntry>(GENERATED_PLUGINS_PROJECTION), playgrounds: read<PlaygroundEntry>(GENERATED_PLAYGROUNDS_PROJECTION) };
}

/** 🏠️ Host detection of projected rows: a variant, alias or bare plugin id whose crate declares `[package.metadata.semio].host`. */
export function projectedHostPluginFilter(projection: GeneratedCatalogProjection, pluginFilter?: string): boolean {
  if (!pluginFilter) return true;
  const variantRow = projection.playgrounds.find((row) => row.variant === pluginFilter || row.aliases.includes(pluginFilter));
  const pluginId = variantRow?.pluginId ?? pluginFilter;
  return projection.entries.some((entry) => entry.pluginId === pluginId && entry.host !== undefined);
}

/** 🎯️ Applies registry filter semantics to already projected rows. */
export function filterProjectedPluginRegistry(projection: GeneratedCatalogProjection, filterPlaygroundPlugin?: string): PluginRegistryEntry[] {
  const entries = [...projection.entries].sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  if (!filterPlaygroundPlugin || projectedHostPluginFilter(projection, filterPlaygroundPlugin)) return entries;
  const ids = new Set(resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin, projection.entries, projection.playgrounds));
  return entries.filter((entry) => ids.has(entry.pluginId));
}
