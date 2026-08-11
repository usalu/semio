/** 🧬️ SemioCadSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioCadSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioCadSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioCadSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioCadSnapshotEntry[];
}
