/** 📐 `dimensions` — the semio image's header-derived raster geometry. */

export interface SemioImageDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
  frameCount: number;
}
