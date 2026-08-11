/** 🧬️ SemioCadArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioCadArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioCadArtifactEntry {
  key: string;
  value: string;
}
export interface SemioCadArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioCadArtifactEntry[];
}
