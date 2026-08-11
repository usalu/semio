/** 🧬️ Mp4Diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp4Diff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp4DiffEntry {
  key: string;
  value: string;
}
export interface Mp4Diff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp4DiffEntry[];
}
