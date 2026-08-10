/** 🧬️ GifArtifact schema. */
export interface GifEntry {
  name: string;
  data: number[];
}
export interface GifArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: GifEntry[];
}
