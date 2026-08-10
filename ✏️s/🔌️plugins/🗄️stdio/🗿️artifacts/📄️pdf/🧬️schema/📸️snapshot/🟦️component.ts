/** 🧬️ PdfSnapshot schema. */
export interface PdfEntry {
  name: string;
  data: number[];
}
export interface PdfSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PdfEntry[];
}
