/** 🧬️ TiffArtifact schema. */
export interface TiffEntry {
  name: string;
  data: number[];
}
export interface TiffArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TiffEntry[];
}
