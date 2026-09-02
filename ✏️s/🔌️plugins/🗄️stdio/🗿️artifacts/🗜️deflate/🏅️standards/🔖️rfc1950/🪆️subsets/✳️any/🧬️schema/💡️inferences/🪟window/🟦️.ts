/** 🪟 `window` — the deflate snapshot's real RFC1950 zlib header semantics (CMF window size,
 * FLG.FLEVEL, FDICT), plus a real payload byte-size + content digest. */

export interface DeflateWindow {
  windowSize: number;
  compressionLevelHint: string;
  hasPresetDictionary: boolean;
  payloadSize: number;
  contentDigest: string;
}
