/** 📐 `dimensions` — the GIF87a logical screen's geometry (never alpha: 87a has none). */

export interface GifDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
