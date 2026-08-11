/** 🧬️ SemioDrawingDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDrawingDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDrawingDiffEntry {
  key: string;
  value: string;
}
export interface SemioDrawingDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDrawingDiffEntry[];
}
