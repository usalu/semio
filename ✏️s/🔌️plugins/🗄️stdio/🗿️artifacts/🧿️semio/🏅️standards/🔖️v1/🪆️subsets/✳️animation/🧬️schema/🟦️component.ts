/** 🧬️ SemioAnimationArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAnimationArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAnimationArtifactEntry {
  key: string;
  value: string;
}
export interface SemioAnimationArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAnimationArtifactEntry[];
}
