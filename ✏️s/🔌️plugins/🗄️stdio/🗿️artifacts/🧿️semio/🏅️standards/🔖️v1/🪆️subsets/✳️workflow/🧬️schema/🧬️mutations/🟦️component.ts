/** 🧬️ SemioWorkflowMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioWorkflowMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioWorkflowMutationEntry {
  key: string;
  value: string;
}
export interface SemioWorkflowMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioWorkflowMutationEntry[];
}
