/** 🧬️ SemioBrepArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioBrepArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioBrepArtifactEntry {
  key: string;
  value: string;
}
export interface SemioBrepArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioBrepArtifactEntry[];
}
