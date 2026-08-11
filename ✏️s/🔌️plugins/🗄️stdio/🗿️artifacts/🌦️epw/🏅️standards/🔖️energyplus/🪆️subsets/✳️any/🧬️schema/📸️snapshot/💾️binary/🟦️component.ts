/** 🧬️ Semio_epw_snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_epw_snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_epw_snapshotEntry {
  key: string;
  value: string;
}
export interface Semio_epw_snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_epw_snapshotEntry[];
}
