/** 🧬️ TsvSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the TsvSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface TsvSnapshotEntry {
  key: string;
  value: string;
}
export interface TsvSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TsvSnapshotEntry[];
}
