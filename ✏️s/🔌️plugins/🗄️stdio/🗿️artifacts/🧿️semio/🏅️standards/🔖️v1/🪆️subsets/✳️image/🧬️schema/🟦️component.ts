/** 🧬️ SemioImageArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioImageArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioImageArtifactEntry {
  key: string;
  value: string;
}
export interface SemioImageArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioImageArtifactEntry[];
}
