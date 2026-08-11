/** 🧬️ AviDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the AviDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface AviDiffEntry {
  key: string;
  value: string;
}
export interface AviDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: AviDiffEntry[];
}
