/** 🧬️ PptxArtifact schema. */
export interface PptxEntry {
  name: string;
  data: number[];
}
export interface PptxArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PptxEntry[];
}
