/** 🧬️ Semio_avi_snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_avi_snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_avi_snapshotEntry {
  key: string;
  value: string;
}
export interface Semio_avi_snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_avi_snapshotEntry[];
}
