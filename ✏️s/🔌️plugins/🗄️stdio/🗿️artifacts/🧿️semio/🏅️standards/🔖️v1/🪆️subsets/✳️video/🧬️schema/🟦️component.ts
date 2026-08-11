/** 🧬️ SemioVideoArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioVideoArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioVideoArtifactEntry {
  key: string;
  value: string;
}
export interface SemioVideoArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioVideoArtifactEntry[];
}
