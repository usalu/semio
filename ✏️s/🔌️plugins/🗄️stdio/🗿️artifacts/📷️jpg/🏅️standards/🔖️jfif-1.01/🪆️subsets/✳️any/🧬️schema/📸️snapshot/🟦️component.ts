/** 🧬️ JpgSnapshot schema. */
export interface JpgEntry {
  name: string;
  data: number[];
}
export interface JpgSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: JpgEntry[];
}
