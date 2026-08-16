/** 🧽️ Composite-window option — the `paintEraser` utility's size/opacity sliders. Typed twin of
 * `🦀️component.rs`'s `measure(config: &RasterConfig) -> WindowMeasure` — eraser reuses the same
 * `RasterConfig.brushSize`/`.brushOpacity` fields as the brush (one shared brush model, two utilities
 * that read it), so this interface shape mirrors `RasterBrushOptions` field-for-field. */
export interface RasterEraserOptions {
  size: number;
  opacity: number;
}

export const RASTER_PLAY_ERASER_OPTIONS_GROUP_ID = "raster-utility-options-paintEraser" as const;
