/** 🧬️ PptxArtifact schema. */
export interface PptxEntry {
  name: string;
  data: number[];
}
export interface PptxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: PptxEntry[];
}
