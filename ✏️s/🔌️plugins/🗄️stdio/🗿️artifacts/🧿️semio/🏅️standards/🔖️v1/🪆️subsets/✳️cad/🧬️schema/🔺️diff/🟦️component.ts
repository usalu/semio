/** 🧬️ SemioCadDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioCadDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioCadDiffEntry {
  key: string;
  value: string;
}
export interface SemioCadDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioCadDiffEntry[];
}
