/** 🧬️ PptxSnapshot schema. */
export interface PptxEntry {
  name: string;
  data: number[];
}
export interface PptxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: PptxEntry[];
}
