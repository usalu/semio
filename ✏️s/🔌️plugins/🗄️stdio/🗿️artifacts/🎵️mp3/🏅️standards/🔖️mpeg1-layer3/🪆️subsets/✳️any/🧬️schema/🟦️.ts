/** 🧬️ Mp3Artifact schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Artifact
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3ArtifactEntry {
  key: string;
  value: string;
}
export interface Mp3Artifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3ArtifactEntry[];
}
