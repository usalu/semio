/** 🧬️ SemioWorkflowDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioWorkflowDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioWorkflowDiffEntry {
  key: string;
  value: string;
}
export interface SemioWorkflowDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioWorkflowDiffEntry[];
}
