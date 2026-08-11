/** 🧬️ HtmlDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the HtmlDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface HtmlDiffEntry {
  key: string;
  value: string;
}
export interface HtmlDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: HtmlDiffEntry[];
}
