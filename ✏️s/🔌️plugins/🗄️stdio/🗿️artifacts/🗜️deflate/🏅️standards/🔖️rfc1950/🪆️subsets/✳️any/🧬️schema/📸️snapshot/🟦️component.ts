/** 🧬️ DeflateSnapshot schema — typed RFC1950 zlib container. */
export type DeflateLevelHint = "fastest" | "fast" | "default" | "maximum";

export interface DeflateSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent — CMF low nibble (CM); RFC1950 defines only 8 (deflate) */
  compressionMethod: number;
  /** @state persistent — CMF high nibble (CINFO); window = 2^(cinfo+8) */
  windowBits: number;
  /** @state persistent — FLG.FLEVEL */
  compressionLevelHint: DeflateLevelHint;
  /** @state persistent — FLG.FDICT + DICTID; present only when a preset dictionary is declared */
  dictId?: number;
  /** @state persistent — decompressed payload */
  payload: number[];
}
