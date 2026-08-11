/** 🧬️ Semio_semio_object_snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_semio_object_snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_semio_object_snapshotEntry {
  key: string;
  value: string;
}
export interface Semio_semio_object_snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_semio_object_snapshotEntry[];
}
