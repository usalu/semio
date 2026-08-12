/** 📐 `dimensions` — the BMP raster's BITMAPINFOHEADER-derived geometry. */

export interface BmpDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
