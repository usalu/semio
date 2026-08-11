/** 🧬️ SemioObjectMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioObjectMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioObjectMutationEntry {
  key: string;
  value: string;
}
export interface SemioObjectMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioObjectMutationEntry[];
}
