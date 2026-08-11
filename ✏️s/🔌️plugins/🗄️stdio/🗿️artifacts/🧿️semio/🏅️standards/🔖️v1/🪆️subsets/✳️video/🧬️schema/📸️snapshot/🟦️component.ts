/** 🧬️ SemioVideoSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioVideoSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioVideoSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioVideoSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioVideoSnapshotEntry[];
}
