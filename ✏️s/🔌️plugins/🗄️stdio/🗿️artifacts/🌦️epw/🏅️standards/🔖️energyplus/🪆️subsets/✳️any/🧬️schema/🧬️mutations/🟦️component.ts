/** 🧬️ EpwMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the EpwMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface EpwMutationEntry {
  key: string;
  value: string;
}
export interface EpwMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: EpwMutationEntry[];
}
