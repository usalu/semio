/** 🧬️ SemioBrepMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioBrepMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioBrepMutationEntry {
  key: string;
  value: string;
}
export interface SemioBrepMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioBrepMutationEntry[];
}
