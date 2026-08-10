/** 🧬️ PngArtifact schema. */
export interface PngEntry {
  name: string;
  data: number[];
}
export interface PngArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PngEntry[];
}
