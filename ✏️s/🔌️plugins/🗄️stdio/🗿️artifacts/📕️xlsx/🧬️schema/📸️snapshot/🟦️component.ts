/** 🧬️ XlsxSnapshot schema. */
export interface XlsxEntry {
  name: string;
  data: number[];
}
export interface XlsxSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: XlsxEntry[];
}
