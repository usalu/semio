/** 🧬️ SemioModelSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioModelSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioModelSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioModelSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioModelSnapshotEntry[];
}
