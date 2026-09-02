/** 💡️ Tiff inference schema — IFD 0 baseline-tag-derived raster geometry. */

export interface TiffDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface TiffInference {
  /** @derived */
  dimensions: TiffDimensions;
}
