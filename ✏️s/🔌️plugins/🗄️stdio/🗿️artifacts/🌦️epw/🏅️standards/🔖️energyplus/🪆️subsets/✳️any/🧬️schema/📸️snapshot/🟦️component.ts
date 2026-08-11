/** 🧬️ EpwSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the EpwSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface EpwSnapshotEntry {
  key: string;
  value: string;
}
export interface EpwSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: EpwSnapshotEntry[];
}
