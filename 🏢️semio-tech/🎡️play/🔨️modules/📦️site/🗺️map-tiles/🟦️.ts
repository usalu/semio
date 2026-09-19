import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { GIS_MAP_DEFAULT_PREFETCH_BOUNDS, mapTileCacheRoots, prefetchMapTiles, type GisMapTileServeMode } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

/** @emoji 🎚️ The static site ships the map's default viewport through z10 (~400 tiles) instead of the full offline z13/z14 set. */
export const PLAY_STATIC_MAP_TILE_Z_MAX = 10;

/** @emoji 🗺️ A static host cannot run the tile-proxy middleware, so builds serve and copy cached tiles only. */
export function playGisMapTileServeMode(viteCommand: "build" | "serve"): GisMapTileServeMode {
  if (viteCommand === "build") return "bundle";
  return process.env.GIS_MAP_TILE_SERVE_MODE === "bundle" ? "bundle" : "fetch";
}

/** @emoji ⬇️ Ensures the map's default viewport has raster and vector tiles before the build copies them. */
export async function prefetchPlayMapTiles(repoRoot: string): Promise<void> {
  const result = await prefetchMapTiles({ repoRoot, bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS, raster: true, vector: true, zMinRaster: 0, zMaxRaster: PLAY_STATIC_MAP_TILE_Z_MAX, zMinVector: 0, zMaxVector: PLAY_STATIC_MAP_TILE_Z_MAX, skipExisting: true, log: line => console.log(line) });
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  if (!existsSync(resolve(vt, "3/1/2.pbf")) || !existsSync(resolve(osm, "0/0/0.png"))) throw new Error(`[play:site] map tile cache incomplete after prefetch (failed=${result.failed})`);
  if (result.failed > 0) throw new Error(`[play:site] map tile prefetch failed for ${result.failed} tile(s)`);
}
