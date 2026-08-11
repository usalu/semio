/** 🧬️ SemioMeshSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioMeshSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioMeshSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioMeshSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioMeshSnapshotEntry[];
}
