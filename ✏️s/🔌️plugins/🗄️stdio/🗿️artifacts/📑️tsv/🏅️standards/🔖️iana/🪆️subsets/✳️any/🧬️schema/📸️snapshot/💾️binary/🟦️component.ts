/** 🧬️ Semio_tsv_snapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_tsv_snapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_tsv_snapshotEntry {
  key: string;
  value: string;
}
export interface Semio_tsv_snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_tsv_snapshotEntry[];
}
