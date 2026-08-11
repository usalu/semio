/** 🧬️ SemioDrawingArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDrawingArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDrawingArtifactEntry {
  key: string;
  value: string;
}
export interface SemioDrawingArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDrawingArtifactEntry[];
}
