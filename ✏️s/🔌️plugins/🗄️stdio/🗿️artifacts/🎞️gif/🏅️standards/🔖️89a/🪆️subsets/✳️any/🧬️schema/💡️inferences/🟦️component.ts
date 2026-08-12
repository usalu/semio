/** 💡️ Gif (89a) inference schema — logical screen geometry (`hasAlpha` from any frame's GCE). */

export interface GifDimensions {
  width: number;
  height: number;
  bitDepth: number;
  hasAlpha: boolean;
  pixelCount: number;
}

export interface GifInference {
  /** @state inferred */
  dimensions: GifDimensions;
}
