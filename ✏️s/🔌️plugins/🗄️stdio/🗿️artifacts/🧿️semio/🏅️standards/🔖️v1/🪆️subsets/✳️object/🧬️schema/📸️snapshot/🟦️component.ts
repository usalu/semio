/** 🧬️ SemioObjectSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioObjectSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioObjectSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioObjectSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioObjectSnapshotEntry[];
}
