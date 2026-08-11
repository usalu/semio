/** 🧬️ SemioWorkflowSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioWorkflowSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioWorkflowSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioWorkflowSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioWorkflowSnapshotEntry[];
}
