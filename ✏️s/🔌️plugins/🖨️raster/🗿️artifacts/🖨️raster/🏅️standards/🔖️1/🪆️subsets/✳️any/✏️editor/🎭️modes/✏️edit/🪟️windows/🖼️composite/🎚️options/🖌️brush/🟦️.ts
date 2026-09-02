/** 🖌️ Composite-window option — the `paintBrush` utility's size/opacity sliders. Typed twin of
 * `🦀️.rs`'s `measure(config: &RasterConfig) -> WindowMeasure` — the two slider values it
 * reads off `RasterConfig.brushSize`/`.brushOpacity`, not a persisted struct of its own. */
export interface RasterBrushOptions {
  size: number;
  opacity: number;
}

export const RASTER_PLAY_BRUSH_OPTIONS_GROUP_ID = "raster-utility-options-paintBrush" as const;
