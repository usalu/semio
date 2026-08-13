/** 🧬️ DocxSnapshot schema. */
export interface DocxEntry {
  name: string;
  data: number[];
}
export interface DocxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: DocxEntry[];
}
