/** 🧬️ TiffSnapshot schema. */
export interface TiffEntry {
  name: string;
  data: number[];
}
export interface TiffSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TiffEntry[];
}
