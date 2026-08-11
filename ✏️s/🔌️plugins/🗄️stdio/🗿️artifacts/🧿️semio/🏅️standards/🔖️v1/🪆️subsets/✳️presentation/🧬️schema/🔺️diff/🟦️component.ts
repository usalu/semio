/** 🧬️ SemioPresentationDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioPresentationDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioPresentationDiffEntry {
  key: string;
  value: string;
}
export interface SemioPresentationDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioPresentationDiffEntry[];
}
