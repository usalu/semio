/** 🧬️ SemioPresentationArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioPresentationArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioPresentationArtifactEntry {
  key: string;
  value: string;
}
export interface SemioPresentationArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioPresentationArtifactEntry[];
}
