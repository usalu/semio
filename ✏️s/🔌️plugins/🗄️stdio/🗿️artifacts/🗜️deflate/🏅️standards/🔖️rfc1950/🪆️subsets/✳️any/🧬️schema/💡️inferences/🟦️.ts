/** 💡️ deflate inference schema — real RFC1950 zlib header semantics (CMF window size, FLG.FLEVEL,
 * FDICT), not a forced multi-entry shape (RFC1950 wraps exactly one deflate stream). */

export interface DeflateWindow {
  windowSize: number;
  compressionLevelHint: string;
  hasPresetDictionary: boolean;
  payloadSize: number;
  contentDigest: string;
}

export interface DeflateInference {
  /** @derived */
  window: DeflateWindow;
}
