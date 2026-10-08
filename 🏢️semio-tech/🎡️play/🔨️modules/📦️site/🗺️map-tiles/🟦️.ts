import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { GIS_MAP_DEFAULT_PREFETCH_BOUNDS, mapTileCacheRoots, prefetchMapTiles } from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts";
import { PLAY_STATIC_MAP_TILE_Z_MAX } from "../🗺️tile-serve-mode/🟦️.ts";

/** ⬇️ Ensures the map's default viewport has raster and vector tiles before the build copies them. */
export async function prefetchPlayMapTiles(repoRoot: string, signal?: AbortSignal): Promise<void> {
  const result = await prefetchMapTiles({ repoRoot, bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS, raster: true, vector: true, zMinRaster: 0, zMaxRaster: PLAY_STATIC_MAP_TILE_Z_MAX, zMinVector: 0, zMaxVector: PLAY_STATIC_MAP_TILE_Z_MAX, skipExisting: true, signal, progress: (completed, total) => console.log(`Play map tiles ${completed}/${total}`), log: line => console.log(line) });
  signal?.throwIfAborted();
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  if (!existsSync(resolve(vt, "3/1/2.pbf")) || !existsSync(resolve(osm, "0/0/0.png"))) throw new Error(`[play:site] map tile cache incomplete after prefetch (failed=${result.failed})`);
  if (result.failed > 0) throw new Error(`[play:site] map tile prefetch failed for ${result.failed} tile(s)`);
}
