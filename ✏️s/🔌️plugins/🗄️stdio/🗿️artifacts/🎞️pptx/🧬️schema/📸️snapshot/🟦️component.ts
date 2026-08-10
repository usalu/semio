/** 🧬️ PptxSnapshot schema. */
export interface PptxEntry {
  name: string;
  data: number[];
}
export interface PptxSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PptxEntry[];
}
