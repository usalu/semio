/** 🧬️ SemioObjectArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioObjectArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioObjectArtifactEntry {
  key: string;
  value: string;
}
export interface SemioObjectArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioObjectArtifactEntry[];
}
