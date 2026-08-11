/** 🧬️ SemioBrepDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioBrepDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioBrepDiffEntry {
  key: string;
  value: string;
}
export interface SemioBrepDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioBrepDiffEntry[];
}
