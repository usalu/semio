/** 🧬️ JpgArtifact schema. */
export interface JpgEntry {
  name: string;
  data: number[];
}
export interface JpgArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: JpgEntry[];
}
