/** 💡️ Semio image inference schema — header-derived raster geometry. */

export interface SemioImageDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
  frameCount: number;
}

export interface SemioImageInference {
  /** @derived */
  dimensions: SemioImageDimensions;
}
