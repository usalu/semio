import type { GisMapTileServeMode } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

/** @emoji 🎚️ The static site ships the map's default viewport through z10 (~400 tiles) instead of the full offline z13/z14 set. */
export const PLAY_STATIC_MAP_TILE_Z_MAX = 10;

/** @emoji 🗺️ A static host cannot run the tile-proxy middleware, so builds serve and copy cached tiles only. */
export function playGisMapTileServeMode(viteCommand: "build" | "serve"): GisMapTileServeMode {
  if (viteCommand === "build") return "bundle";
  return process.env.GIS_MAP_TILE_SERVE_MODE === "bundle" ? "bundle" : "fetch";
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playmaptiles/🟦️.ts");
  await registerTests1(import.meta.vitest, { playGisMapTileServeMode, PLAY_STATIC_MAP_TILE_Z_MAX });
}
