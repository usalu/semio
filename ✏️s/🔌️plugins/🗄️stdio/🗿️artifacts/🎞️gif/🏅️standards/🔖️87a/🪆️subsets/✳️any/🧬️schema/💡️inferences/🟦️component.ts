/** 💡️ Gif (87a) inference schema — logical screen geometry (never alpha: 87a has none). */

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
