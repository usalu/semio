/** 🧬️ EpwArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the EpwArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface EpwArtifactEntry {
  key: string;
  value: string;
}
export interface EpwArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: EpwArtifactEntry[];
}
