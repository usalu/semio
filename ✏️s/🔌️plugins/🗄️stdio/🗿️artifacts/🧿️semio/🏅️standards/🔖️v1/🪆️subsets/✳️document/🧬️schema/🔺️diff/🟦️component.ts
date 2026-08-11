/** 🧬️ SemioDocumentDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDocumentDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDocumentDiffEntry {
  key: string;
  value: string;
}
export interface SemioDocumentDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDocumentDiffEntry[];
}
