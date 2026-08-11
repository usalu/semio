/** 🧬️ SemioMeshDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioMeshDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioMeshDiffEntry {
  key: string;
  value: string;
}
export interface SemioMeshDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioMeshDiffEntry[];
}
