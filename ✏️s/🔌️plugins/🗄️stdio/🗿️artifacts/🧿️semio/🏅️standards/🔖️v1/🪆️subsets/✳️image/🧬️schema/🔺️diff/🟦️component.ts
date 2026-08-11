/** 🧬️ SemioImageDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioImageDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioImageDiffEntry {
  key: string;
  value: string;
}
export interface SemioImageDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioImageDiffEntry[];
}
