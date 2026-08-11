/** 🧬️ SemioImageSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioImageSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioImageSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioImageSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioImageSnapshotEntry[];
}
