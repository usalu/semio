/** 🧬️ Semio_mp3_snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_mp3_snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_mp3_snapshotEntry {
  key: string;
  value: string;
}
export interface Semio_mp3_snapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Semio_mp3_snapshotEntry[];
}
