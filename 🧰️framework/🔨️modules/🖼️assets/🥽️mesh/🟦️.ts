import delivery from "./📇️catalog.json" with { type: "json" };
import metabolism from "../🌱️metabolism/🎨️representation/📇️catalog.json" with { type: "json" };

export type MeshAsset = { readonly url: string; readonly source: string; readonly path: string };
export type MeshDeliveryCatalog = readonly MeshAsset[];

function object(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Mesh catalog object required");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== keys.length || keys.some(key => !(key in row))) throw new Error("Mesh catalog fields do not match the schema");
  return row;
}

function path(value: unknown, extension = ""): string {
  if (typeof value !== "string" || !value.endsWith(extension)) throw new Error("Mesh catalog path required");
  const stem = extension ? value.slice(0, -extension.length) : value;
  if (!stem || stem.split("/").some(part => !part || /[.\\%?#\u0000-\u001f]/u.test(part))) throw new Error(`Unsafe mesh catalog path: ${value}`);
  return value;
}

function publicUrl(value: unknown): string {
  if (typeof value !== "string" || !value.startsWith("/mesh/")) throw new Error("Mesh public URL required");
  const leaf = path(value.slice("/mesh/".length), ".glb");
  if (leaf.includes("/")) throw new Error("Mesh public identity must be a single explicit URL key");
  return value;
}

function rows(value: unknown): readonly unknown[] {
  if (!Array.isArray(value)) throw new Error("Mesh catalog rows required");
  return value;
}

/** 🧭️ Resolves schema-owned public identities to explicit source and delivery paths without aliases. */
export function parseMeshDeliveryCatalog(input: unknown, readCatalog: (path: string) => unknown): MeshDeliveryCatalog {
  const authority = object(input, ["$schema", "version", "collections", "entries"]);
  if (authority.version !== 1 || typeof authority.$schema !== "string") throw new Error("Unsupported mesh delivery schema");
  const result: MeshAsset[] = [];
  const urls = new Set<string>();
  const sources = new Set<string>();
  const paths = new Set<string>();
  const catalogs = new Set<string>();
  const admit = (entry: MeshAsset): void => {
    if (urls.has(entry.url) || sources.has(entry.source) || paths.has(entry.path)) throw new Error(`Duplicate mesh identity: ${entry.url}`);
    urls.add(entry.url);
    sources.add(entry.source);
    paths.add(entry.path);
    result.push(Object.freeze(entry));
  };
  for (const value of rows(authority.collections)) {
    const collection = object(value, ["catalog", "root", "output"]);
    const catalogPath = path(collection.catalog, ".json");
    if (catalogs.has(catalogPath)) throw new Error(`Duplicate mesh source catalog: ${catalogPath}`);
    catalogs.add(catalogPath);
    const root = path(collection.root);
    const output = path(collection.output);
    const source = object(readCatalog(catalogPath), ["$schema", "version", "entries"]);
    if (source.version !== 1 || typeof source.$schema !== "string" || rows(source.entries).length === 0) throw new Error("Unsupported mesh source schema");
    for (const value of rows(source.entries)) {
      const entry = object(value, ["url", "path"]);
      const leaf = path(entry.path, ".glb");
      admit({ url: publicUrl(entry.url), source: `${root}/${leaf}`, path: `${output}/${leaf}` });
    }
  }
  for (const value of rows(authority.entries)) {
    const entry = object(value, ["url", "source", "path"]);
    admit({ url: publicUrl(entry.url), source: path(entry.source, ".glb"), path: path(entry.path, ".glb") });
  }
  return Object.freeze(result);
}

export const MESH_DELIVERY_CATALOG = parseMeshDeliveryCatalog(delivery, path => {
  if (path === "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json") return metabolism;
  throw new Error(`Unknown mesh source catalog: ${path}`);
});

const indexes = new WeakMap<MeshDeliveryCatalog, ReadonlyMap<string, MeshAsset>>();

/** 🔎️ Unknown and corrupted public mesh IDs are errors, never filename fallbacks. */
export function resolveMeshAsset(url: string, catalog: MeshDeliveryCatalog = MESH_DELIVERY_CATALOG): MeshAsset {
  let index = indexes.get(catalog);
  if (!index) {
    index = new Map(catalog.map(entry => [entry.url, entry]));
    indexes.set(catalog, index);
  }
  const entry = index.get(url);
  if (!entry) throw new Error(`Unknown mesh asset: ${url}`);
  return entry;
}

/** 🌐️ Rewrites only the mesh namespace at the transport boundary; other asset domains retain ownership. */
export function meshAssetTransportUrl(url: string, catalog: MeshDeliveryCatalog = MESH_DELIVERY_CATALOG): string {
  return url.startsWith("/mesh/") ? `/mesh/${resolveMeshAsset(url, catalog).path}` : url;
}
