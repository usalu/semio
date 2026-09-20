export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: { playGisMapTileServeMode: (command: "build" | "serve") => string; PLAY_STATIC_MAP_TILE_Z_MAX: number },
): Promise<void> {
  const { playGisMapTileServeMode, PLAY_STATIC_MAP_TILE_Z_MAX } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayMapTileTests
  describe("playGisMapTileServeMode", () => {
    it("bundles map tiles for production Vite builds", () => {
      expect(playGisMapTileServeMode("build")).toBe("bundle");
    });

    it("caps the static ship prefetch below full offline-play depth", () => {
      expect(PLAY_STATIC_MAP_TILE_Z_MAX).toBeLessThanOrEqual(10);
    });

    it("keeps dev serve on fetch unless GIS_MAP_TILE_SERVE_MODE overrides", () => {
      const previous = process.env.GIS_MAP_TILE_SERVE_MODE;
      delete process.env.GIS_MAP_TILE_SERVE_MODE;
      expect(playGisMapTileServeMode("serve")).toBe("fetch");
      process.env.GIS_MAP_TILE_SERVE_MODE = "bundle";
      expect(playGisMapTileServeMode("serve")).toBe("bundle");
      if (previous === undefined) delete process.env.GIS_MAP_TILE_SERVE_MODE;
      else process.env.GIS_MAP_TILE_SERVE_MODE = previous;
    });
  });
  //#endregion 🧪️PlayMapTileTests
}
