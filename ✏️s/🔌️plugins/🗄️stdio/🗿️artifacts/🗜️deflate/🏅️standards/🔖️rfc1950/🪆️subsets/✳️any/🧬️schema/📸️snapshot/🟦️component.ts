/** 🧬️ DeflateSnapshot schema — typed RFC1950 zlib container. */
export type DeflateLevelHint = "fastest" | "fast" | "default" | "maximum";

export interface DeflateSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact — CMF low nibble (CM); RFC1950 defines only 8 (deflate) */
  compressionMethod: number;
  /** @state artifact — CMF high nibble (CINFO); window = 2^(cinfo+8) */
  windowBits: number;
  /** @state artifact — FLG.FLEVEL */
  compressionLevelHint: DeflateLevelHint;
  /** @state artifact — FLG.FDICT + DICTID; present only when a preset dictionary is declared */
  dictId?: number;
  /** @state artifact — decompressed payload */
  payload: number[];
}
