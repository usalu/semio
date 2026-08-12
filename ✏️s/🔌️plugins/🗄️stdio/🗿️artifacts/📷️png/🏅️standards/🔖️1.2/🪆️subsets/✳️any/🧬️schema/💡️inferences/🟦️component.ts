/** 💡️ Png inference schema — IHDR-derived raster geometry. */

export interface PngDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface PngInference {
  /** @state inferred */
  dimensions: PngDimensions;
}
