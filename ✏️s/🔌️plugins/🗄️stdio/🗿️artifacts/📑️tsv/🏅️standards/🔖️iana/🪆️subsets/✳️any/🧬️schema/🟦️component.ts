/** 🧬️ TsvArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the TsvArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface TsvArtifactEntry {
  key: string;
  value: string;
}
export interface TsvArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TsvArtifactEntry[];
}
