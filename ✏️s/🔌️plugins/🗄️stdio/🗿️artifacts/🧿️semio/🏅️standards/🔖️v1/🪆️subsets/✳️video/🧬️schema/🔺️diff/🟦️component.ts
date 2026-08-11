/** 🧬️ SemioVideoDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioVideoDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioVideoDiffEntry {
  key: string;
  value: string;
}
export interface SemioVideoDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioVideoDiffEntry[];
}
