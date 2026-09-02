/** 🧬️ BcfArtifact schema. */
export interface BcfEntry {
  name: string;
  data: number[];
}
export interface BcfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: BcfEntry[];
}
