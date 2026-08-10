/** 🧬️ PngSnapshot schema. */
export interface PngEntry {
  name: string;
  data: number[];
}
export interface PngSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PngEntry[];
}
