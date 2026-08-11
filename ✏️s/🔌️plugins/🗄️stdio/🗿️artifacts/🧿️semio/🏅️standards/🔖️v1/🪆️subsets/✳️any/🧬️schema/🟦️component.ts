/** 🧬️ SemioArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioArtifactEntry {
  key: string;
  value: string;
}
export interface SemioArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioArtifactEntry[];
}
