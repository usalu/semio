/** 🧬️ Mp3Snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3SnapshotEntry {
  key: string;
  value: string;
}
export interface Mp3Snapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3SnapshotEntry[];
}
