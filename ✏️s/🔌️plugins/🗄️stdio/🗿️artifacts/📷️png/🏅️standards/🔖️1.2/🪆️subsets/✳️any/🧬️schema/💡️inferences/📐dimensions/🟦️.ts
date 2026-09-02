/** 📐 `dimensions` — the PNG raster's IHDR-derived header geometry. */

export interface PngDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
