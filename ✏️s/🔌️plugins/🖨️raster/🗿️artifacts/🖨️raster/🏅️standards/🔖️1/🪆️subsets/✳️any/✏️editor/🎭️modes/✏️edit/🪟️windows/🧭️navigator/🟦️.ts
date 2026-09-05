/** 🧭️ Raster editor — Navigator window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(document: &RasterDocument, config: &RasterConfig)` boundary. No `☑️options` node:
 * the navigator has no live chrome measures of its own, matching the Rust file's own doc comment. */

/** ✏️ The Navigator window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RasterNavigatorViewModel {
  windowKindId: "raster-navigator";
  bodyKey: "raster.play.navigator";
  surfaceId: "raster.play.navigator";
  viewMode: "navigator";
  activeUtilityId: string;
  compositeViewportJson?: string;
}

export const RASTER_PLAY_WINDOW_NAVIGATOR = "raster-navigator" as const;
export const RASTER_PLAY_BODY_NAVIGATOR = "raster.play.navigator" as const;
export const RASTER_PLAY_SURFACE_NAVIGATOR = "raster.play.navigator" as const;
