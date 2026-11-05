import {readFileSync} from "node:fs";
import {join} from "node:path";

/** 🎮️ The catalog row fields the playground port table reads; the registry owns the full row. */
export type PlaygroundCatalogRow = { readonly variant: string; readonly ports: { readonly react: number; readonly wgpu: number } };

/** 📚️ Reads the caller-supplied publication root; absence admits empty, configured missing or malformed files refuse. */
export function loadFrameworkOsPlaygroundCatalog<Variant extends PlaygroundCatalogRow = PlaygroundCatalogRow>(catalogRoot=process.env.SEMIO_PLUGIN_CATALOG): readonly Variant[] {
  if(catalogRoot===undefined)return [];
  const catalogPath = join(catalogRoot, "🚀️playgrounds.json");
  return JSON.parse(readFileSync(catalogPath, "utf8")) as readonly Variant[];
}

