/** 🧬️ SemioPresentationSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioPresentationSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioPresentationSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioPresentationSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioPresentationSnapshotEntry[];
}
