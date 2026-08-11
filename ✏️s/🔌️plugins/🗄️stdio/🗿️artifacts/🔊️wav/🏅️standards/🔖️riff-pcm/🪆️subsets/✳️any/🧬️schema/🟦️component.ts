/** 🧬️ WavArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the WavArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavArtifactEntry {
  key: string;
  value: string;
}
export interface WavArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: WavArtifactEntry[];
}
