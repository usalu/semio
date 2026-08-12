/** 📐 `dimensions` — the JPEG raster's canonical geometry (never alpha: JPEG has none). */

export interface JpgDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
