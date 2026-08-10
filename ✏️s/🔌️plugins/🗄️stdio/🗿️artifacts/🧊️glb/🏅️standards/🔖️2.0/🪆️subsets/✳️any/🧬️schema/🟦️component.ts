/** 🧬️ GlbArtifact schema. */
export interface GlbEntry {
  name: string;
  data: number[];
}
export interface GlbArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: GlbEntry[];
}
