/** 🧬️ HtmlSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the HtmlSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface HtmlSnapshotEntry {
  key: string;
  value: string;
}
export interface HtmlSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: HtmlSnapshotEntry[];
}
