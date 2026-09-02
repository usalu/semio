/** 📐 `dimensions` — the GIF89a logical screen's geometry (`hasAlpha` from any frame's GCE). */

export interface GifDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}
