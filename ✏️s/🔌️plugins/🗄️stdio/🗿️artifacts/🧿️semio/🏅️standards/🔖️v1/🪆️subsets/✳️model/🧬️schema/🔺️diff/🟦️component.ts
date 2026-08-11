/** 🧬️ SemioModelDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioModelDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioModelDiffEntry {
  key: string;
  value: string;
}
export interface SemioModelDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioModelDiffEntry[];
}
