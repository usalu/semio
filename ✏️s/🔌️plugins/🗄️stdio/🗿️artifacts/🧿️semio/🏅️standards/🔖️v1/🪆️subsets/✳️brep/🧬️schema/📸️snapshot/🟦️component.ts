/** 🧬️ SemioBrepSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioBrepSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioBrepSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioBrepSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioBrepSnapshotEntry[];
}
