/** 🧬️ SemioObjectDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioObjectDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioObjectDiffEntry {
  key: string;
  value: string;
}
export interface SemioObjectDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioObjectDiffEntry[];
}
