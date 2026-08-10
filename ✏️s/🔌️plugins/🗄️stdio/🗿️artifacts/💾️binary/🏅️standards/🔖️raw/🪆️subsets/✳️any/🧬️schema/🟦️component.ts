/** 🧬️ BinaryArtifact schema. */
export interface BinaryArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ bytes: number[];
}
