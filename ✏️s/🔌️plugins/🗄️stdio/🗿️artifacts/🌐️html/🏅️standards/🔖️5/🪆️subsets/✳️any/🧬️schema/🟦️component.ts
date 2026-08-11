/** 🧬️ HtmlArtifact schema. 🚧 scaffolded by W1b — generic facet mirror; the HtmlArtifact
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface HtmlArtifactEntry {
  key: string;
  value: string;
}
export interface HtmlArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: HtmlArtifactEntry[];
}
