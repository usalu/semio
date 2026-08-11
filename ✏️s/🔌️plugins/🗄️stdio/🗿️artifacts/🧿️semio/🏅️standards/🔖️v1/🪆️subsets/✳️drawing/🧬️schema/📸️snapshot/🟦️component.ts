/** 🧬️ SemioDrawingSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDrawingSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDrawingSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioDrawingSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDrawingSnapshotEntry[];
}
