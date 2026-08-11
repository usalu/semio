/** 🧬️ SemioAnimationSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAnimationSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAnimationSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioAnimationSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAnimationSnapshotEntry[];
}
