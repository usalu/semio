/** 🧬️ TsvDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the TsvDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface TsvDiffEntry {
  key: string;
  value: string;
}
export interface TsvDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TsvDiffEntry[];
}
