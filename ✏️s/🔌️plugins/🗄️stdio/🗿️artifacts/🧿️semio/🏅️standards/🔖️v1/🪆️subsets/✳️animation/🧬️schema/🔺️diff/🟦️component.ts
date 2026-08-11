/** 🧬️ SemioAnimationDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAnimationDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAnimationDiffEntry {
  key: string;
  value: string;
}
export interface SemioAnimationDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAnimationDiffEntry[];
}
