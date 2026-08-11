/** 🧬️ SemioWorkflowArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioWorkflowArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioWorkflowArtifactEntry {
  key: string;
  value: string;
}
export interface SemioWorkflowArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioWorkflowArtifactEntry[];
}
