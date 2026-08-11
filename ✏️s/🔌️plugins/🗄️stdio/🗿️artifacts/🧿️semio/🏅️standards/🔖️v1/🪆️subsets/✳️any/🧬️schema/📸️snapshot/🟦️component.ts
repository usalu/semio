/** 🧬️ SemioSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioSnapshotEntry[];
}
