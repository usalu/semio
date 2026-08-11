/** 🧬️ SemioAudioArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAudioArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAudioArtifactEntry {
  key: string;
  value: string;
}
export interface SemioAudioArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAudioArtifactEntry[];
}
