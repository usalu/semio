/** 💡️ Jpg inference schema — canonical raster geometry. */

export interface JpgDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface JpgInference {
  /** @state inferred */
  dimensions: JpgDimensions;
}
