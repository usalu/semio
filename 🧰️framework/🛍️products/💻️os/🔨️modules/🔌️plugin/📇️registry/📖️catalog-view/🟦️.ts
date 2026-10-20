import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import type { ComponentSourceOwnerV1, DeployedRegistryEntryV1 } from "../🔎️discovery/🟦️.ts";
import {parsePluginBuildTargetV1, parseDeployedRegistryEntryV1 } from "../🔎️discovery/🧬️schema/🟦️.ts";
import type { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";
import { parseModuleDirectories, type ModuleDirectory } from "../📦️deployment/🟦️.ts";

export type GeneratedCatalogProjection = { readonly entries: readonly DeployedRegistryEntryV1[]; readonly playgrounds: readonly PlaygroundEntry[] };

/** 🧭️ Rows plus playground declarations under one filter, whether the rows are deployed or only discovered as build sources. */
export type CatalogSelection<T extends ComponentSourceOwnerV1 = DeployedRegistryEntryV1> = { readonly entries: readonly T[]; readonly playgrounds: readonly PlaygroundEntry[] };

export const GENERATED_PLUGINS_PROJECTION = "🔌️plugins.json";

export const GENERATED_PLAYGROUNDS_PROJECTION = "🎠️playgrounds.json";

/** 📦️Admits only deployment directories declared by the supplied registry owners. */
export function registryModuleDirectories(entries: readonly ModuleDirectory[]): readonly ModuleDirectory[] {
  return parseModuleDirectories({ version: 1, modules: entries.map(({ pluginId, directoryName }) => ({ pluginId, directoryName })) });
}

/** 📖️ Reads the rows `generate` just projected into `🤖️generated` so one dev boot walks the repository once instead of once per consumer; the projection is the language-agnostic twin of [[generatePluginRegistry]] and [[generatePlaygroundRegistry]]. */
export function readGeneratedCatalogProjection(generatedDir = runtimeCatalogProjectionRootV1()): GeneratedCatalogProjection {
  if(generatedDir===undefined)return {entries:[],playgrounds:[]};
  const read = <T>(name: string): readonly T[] => {
    const parsed: unknown = JSON.parse(readFileSync(join(generatedDir, name), "utf8"));
    if (!Array.isArray(parsed)) throw new Error(`📇️registry: ${name} is not a projected row array`);
    return parsed as T[];
  };
  return parseGeneratedCatalogProjectionV1({ entries: read<unknown>(GENERATED_PLUGINS_PROJECTION), playgrounds: read<unknown>(GENERATED_PLAYGROUNDS_PROJECTION) });
}

/** 📖️ Admits one projected catalog document: both row arrays are parsed row by row and the module directories must stay unique. */
export function parseGeneratedCatalogProjectionV1(value: unknown): GeneratedCatalogProjection {
  if (typeof value !== "object" || value === null || !("entries" in value) || !("playgrounds" in value) || !Array.isArray(value.entries) || !Array.isArray(value.playgrounds)) throw new Error("📇️registry: catalog projection requires entries and playgrounds row arrays");
  const entries = value.entries.map(parseDeployedRegistryEntryV1);
  registryModuleDirectories(entries);
  return { entries, playgrounds: value.playgrounds.map(parseProjectedPlaygroundV1) };
}

/** 🏠️ Host detection of projected rows: a variant, alias or bare plugin id whose crate declares
 * `[package.metadata.semio].host` and whose playground row names no `app`. A row that names one boots
 * that single artifact app standalone, even on the host crate (`🪐️space`'s Home and Space). */
export function projectedHostPluginFilter(projection: CatalogSelection<ComponentSourceOwnerV1>, pluginFilter?: string): boolean {
  if (!pluginFilter) return true;
  const variantRow = projection.playgrounds.find((row) => row.variant === pluginFilter || row.aliases.includes(pluginFilter));
  if (variantRow?.app !== undefined) return false;
  const pluginId = variantRow?.pluginId ?? pluginFilter;
  return projection.entries.some((entry) => entry.pluginId === pluginId && entry.host !== undefined);
}

/** 🎯️ Applies registry filter semantics to already projected rows. */
export function filterProjectedPluginRegistry<T extends ComponentSourceOwnerV1>(projection: CatalogSelection<T>, filterPlaygroundPlugin?: string): T[] {
  const entries = [...projection.entries].sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  if (!filterPlaygroundPlugin || projectedHostPluginFilter(projection, filterPlaygroundPlugin)) return entries;
  const ids = new Set(resolveProjectedPluginIdsV1(filterPlaygroundPlugin, projection.entries, projection.playgrounds));
  return entries.filter((entry) => ids.has(entry.pluginId));
}

/** 🎯️ Resolves one filter within the supplied rows alone; the playground row decides the target and whether it is app scoped. */
export function resolveProjectedPluginIdsV1(filter: string, entries: readonly ComponentSourceOwnerV1[], playgrounds: readonly { readonly variant: string; readonly aliases: readonly string[]; readonly pluginId: string; readonly app?: string }[]): readonly string[] {
  return closeProjectedPluginV1(filter, entries, playgrounds.find((p) => p.variant === filter || p.aliases.includes(filter)));
}

/** 🧮️ The runtime closure of one target plugin: empty when the rows do not declare it. */
export function closeProjectedPluginV1(filter: string, entries: readonly ComponentSourceOwnerV1[], row: { readonly pluginId: string; readonly app?: string } | undefined): readonly string[] {
  const target = row?.pluginId ?? filter;
  return entries.some((entry) => entry.pluginId === target) ? runtimeComponentClosure(entries, [{ id: target, appScoped: row?.app !== undefined }]) : [];
}


/** 📍️ A process receives the publication it operates on from its caller. */
export function runtimeCatalogProjectionRootV1():string|undefined{
  const path=process.env.SEMIO_PLUGIN_CATALOG;
  if(path===undefined)return undefined;
  if(!path||path.includes("\0"))throw Error("Explicit catalog root required");
  return path;
}
/** 🧩️ Derives deployment inventory from supplied rows without application constants. */
export function catalogBuildInventoryV1(projection:GeneratedCatalogProjection){
  const plugins=projection.entries.filter(row=>row.role==="plugin"),extensions=projection.entries.filter(row=>row.role==="extension");
  const hosts=plugins.flatMap(row=>row.host?[{pluginId:row.pluginId,...row.host}]:[]);
  const defaults=projection.playgrounds.filter(row=>row.catalogDefault===true);
  if(defaults.length>1)throw Error("Catalog declares multiple defaults");
  return {plugins,extensions,hosts,playgrounds:projection.playgrounds,moduleDirectories:registryModuleDirectories(projection.entries),defaultVariant:defaults[0]?.variant};
}
export function runtimeCatalogBuildInventoryV1(){return catalogBuildInventoryV1(readGeneratedCatalogProjection());}
export type PluginBuildTarget=import("../🔎️discovery/🧬️schema/🟦️.ts").PluginBuildTargetV1;
export type PlaygroundBuildTarget=PlaygroundEntry;

/** 🎮️ Rejects malformed configured playground rows before a host resolves them. */
export function parseProjectedPlaygroundV1(value:unknown):PlaygroundEntry{
  if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Malformed projected playground");
  const row=value as Record<string,unknown>,keys=["variant","pluginId","cratePath","catalogDefault","app","brand","devContribution","compositionConfigPath","nativeHost","mcpHost","distDir","aliases","ports","userPorts","hub","dataDir","userDataDir","localOnly","viewerPath","examples","engines","assets"];
  if(Object.keys(row).some(key=>!keys.includes(key)))throw Error("Unknown projected playground field");
  for(const key of ["variant","pluginId","cratePath"])if(typeof row[key]!=="string"||!row[key])throw Error("Invalid projected playground "+key);
  for(const key of ["aliases","examples","engines"])if(!Array.isArray(row[key])||(row[key] as unknown[]).some(item=>typeof item!=="string"))throw Error("Invalid projected playground "+key);
  const ports=row.ports as Record<string,unknown>|undefined;
  if(!ports||typeof ports!=="object"||Array.isArray(ports)||Object.keys(ports).some(key=>!["react","wgpu"].includes(key))||![ports.react,ports.wgpu].every(port=>Number.isInteger(port)&&Number(port)>0&&Number(port)<=65535))throw Error("Invalid projected playground ports");
  if(!Array.isArray(row.assets))throw Error("Invalid projected playground assets");
  for(const key of ["catalogDefault","localOnly"])if(row[key]!==undefined&&typeof row[key]!=="boolean")throw Error("Invalid projected playground "+key);
  for(const key of ["app","brand","devContribution","compositionConfigPath","distDir","hub","dataDir","userDataDir","viewerPath"])if(row[key]!==undefined&&typeof row[key]!=="string")throw Error("Invalid projected playground "+key);
  return row as unknown as PlaygroundEntry;
}

/** 🏗️ Source inventory remains selectable while compiled deployment rows are withheld. */
export function readCatalogBuildTargetsV1(generatedDir=runtimeCatalogProjectionRootV1()){
  if(generatedDir===undefined)return [];
  const value:unknown=JSON.parse(readFileSync(join(generatedDir,"🗂️sources.json"),"utf8"));
  if(!Array.isArray(value))throw Error("Malformed configured source catalog");
  return value.flatMap(row=>{if(!row||typeof row!=="object")throw Error("Malformed source catalog row");return row.directoryName===undefined?[]:[parsePluginBuildTargetV1(row)];});
}
