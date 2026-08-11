/** 🧬️ SemioDocumentArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDocumentArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDocumentArtifactEntry {
  key: string;
  value: string;
}
export interface SemioDocumentArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDocumentArtifactEntry[];
}
