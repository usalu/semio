/** 🖼️ Raster editor — Composite window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(document: &RasterDocument, config: &RasterConfig)` boundary — the paint-2d scene
 * payload plus the brush/eraser chrome measures a mutation-capable surface carries (absent from the
 * viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

export * as brushOptions from "./🎚️options/🖌️brush/🟦️";
export * as eraserOptions from "./🎚️options/🧽️eraser/🟦️";

/** ✏️ The Composite window's typed view-model — mirrors the Rust `render()` boundary's inputs plus
 * its config-derived `window_measures()` chrome. */
export interface RasterCompositeViewModel {
  windowKindId: "raster-composite";
  bodyKey: "raster.play.composite";
  surfaceId: "raster.play.composite";
  viewMode: "composite";
  activeUtilityId: string;
  brushSize: number;
  brushOpacity: number;
}

export const RASTER_PLAY_WINDOW_COMPOSITE = "raster-composite" as const;
export const RASTER_PLAY_BODY_COMPOSITE = "raster.play.composite" as const;
export const RASTER_PLAY_SURFACE_COMPOSITE = "raster.play.composite" as const;
