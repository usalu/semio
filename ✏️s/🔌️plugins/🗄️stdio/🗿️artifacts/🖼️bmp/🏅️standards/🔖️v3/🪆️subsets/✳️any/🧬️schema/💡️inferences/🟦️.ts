/** 💡️ Bmp inference schema — BITMAPINFOHEADER-derived raster geometry. */

export interface BmpDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface BmpInference {
  /** @derived */
  dimensions: BmpDimensions;
}
