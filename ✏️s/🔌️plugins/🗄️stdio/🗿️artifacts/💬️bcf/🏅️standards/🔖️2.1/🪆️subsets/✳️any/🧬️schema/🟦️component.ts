/** 🧬️ BcfArtifact schema. */
export interface BcfEntry {
  name: string;
  data: number[];
}
export interface BcfArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: BcfEntry[];
}
