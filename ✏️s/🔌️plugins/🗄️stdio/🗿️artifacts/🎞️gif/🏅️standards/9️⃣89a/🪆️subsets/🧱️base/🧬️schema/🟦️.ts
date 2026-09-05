/** 🧬️ GifArtifact schema. */
export interface GifEntry {
  name: string;
  data: number[];
}
export interface GifArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: GifEntry[];
}
