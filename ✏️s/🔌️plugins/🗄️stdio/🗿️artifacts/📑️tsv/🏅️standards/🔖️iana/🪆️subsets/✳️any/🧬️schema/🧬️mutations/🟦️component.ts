/** 🧬️ TsvMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the TsvMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface TsvMutationEntry {
  key: string;
  value: string;
}
export interface TsvMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: TsvMutationEntry[];
}
