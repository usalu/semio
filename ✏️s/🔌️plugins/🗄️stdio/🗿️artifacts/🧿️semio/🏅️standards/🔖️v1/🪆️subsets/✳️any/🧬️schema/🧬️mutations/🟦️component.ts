/** 🧬️ SemioMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioMutationEntry {
  key: string;
  value: string;
}
export interface SemioMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioMutationEntry[];
}
