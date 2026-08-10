/** 🧬️ DocxArtifact schema. */
export interface DocxEntry {
  name: string;
  data: number[];
}
export interface DocxArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: DocxEntry[];
}
