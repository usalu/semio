/** 🧬️ EpwDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the EpwDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface EpwDiffEntry {
  key: string;
  value: string;
}
export interface EpwDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: EpwDiffEntry[];
}
