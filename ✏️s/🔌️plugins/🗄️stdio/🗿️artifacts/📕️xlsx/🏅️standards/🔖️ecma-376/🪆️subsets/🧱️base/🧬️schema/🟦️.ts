/** 🧬️ XlsxArtifact schema. */
export interface XlsxEntry {
  name: string;
  data: number[];
}
export interface XlsxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: XlsxEntry[];
}
