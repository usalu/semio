/** 🧬️ AviSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the AviSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface AviSnapshotEntry {
  key: string;
  value: string;
}
export interface AviSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: AviSnapshotEntry[];
}
