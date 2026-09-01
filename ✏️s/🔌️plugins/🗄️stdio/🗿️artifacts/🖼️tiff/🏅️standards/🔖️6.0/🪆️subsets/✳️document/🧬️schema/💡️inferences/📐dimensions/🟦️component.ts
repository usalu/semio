/** 📐 `dimensions` — the TIFF raster's baseline-tag-derived geometry (IFD 0). */

export interface TiffDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
