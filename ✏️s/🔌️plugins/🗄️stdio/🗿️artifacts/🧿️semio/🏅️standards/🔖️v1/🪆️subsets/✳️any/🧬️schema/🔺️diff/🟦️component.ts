/** 🧬️ SemioDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDiffEntry {
  key: string;
  value: string;
}
export interface SemioDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDiffEntry[];
}
