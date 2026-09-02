/** 🧬️ DocxArtifact schema. */
export interface DocxEntry {
  name: string;
  data: number[];
}
export interface DocxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: DocxEntry[];
}
