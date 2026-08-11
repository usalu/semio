/** 🧬️ SemioModelArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioModelArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioModelArtifactEntry {
  key: string;
  value: string;
}
export interface SemioModelArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioModelArtifactEntry[];
}
