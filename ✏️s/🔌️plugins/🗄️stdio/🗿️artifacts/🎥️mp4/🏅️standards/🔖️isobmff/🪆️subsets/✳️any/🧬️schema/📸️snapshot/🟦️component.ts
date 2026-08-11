/** 🧬️ Mp4Snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp4Snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp4SnapshotEntry {
  key: string;
  value: string;
}
export interface Mp4Snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp4SnapshotEntry[];
}
