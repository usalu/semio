/** 🧬️ XlsxArtifact schema. */
export interface XlsxEntry {
  name: string;
  data: number[];
}
export interface XlsxArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: XlsxEntry[];
}
