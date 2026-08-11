/** 🧬️ SemioDocumentSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDocumentSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDocumentSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioDocumentSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDocumentSnapshotEntry[];
}
