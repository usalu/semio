/** 🧬️ AviArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the AviArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface AviArtifactEntry {
  key: string;
  value: string;
}
export interface AviArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: AviArtifactEntry[];
}
