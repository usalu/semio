/** 🧬️ XlsxSnapshot schema. */
export interface XlsxEntry {
  name: string;
  data: number[];
}
export interface XlsxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: XlsxEntry[];
}
