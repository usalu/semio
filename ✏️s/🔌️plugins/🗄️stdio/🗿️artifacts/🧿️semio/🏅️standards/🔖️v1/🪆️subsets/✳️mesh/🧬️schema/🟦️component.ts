/** 🧬️ SemioMeshArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioMeshArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioMeshArtifactEntry {
  key: string;
  value: string;
}
export interface SemioMeshArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioMeshArtifactEntry[];
}
