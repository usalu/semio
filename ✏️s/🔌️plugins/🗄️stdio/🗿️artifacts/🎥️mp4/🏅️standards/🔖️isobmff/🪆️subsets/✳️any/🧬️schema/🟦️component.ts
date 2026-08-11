/** 🧬️ Mp4Artifact schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp4Artifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp4ArtifactEntry {
  key: string;
  value: string;
}
export interface Mp4Artifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp4ArtifactEntry[];
}
