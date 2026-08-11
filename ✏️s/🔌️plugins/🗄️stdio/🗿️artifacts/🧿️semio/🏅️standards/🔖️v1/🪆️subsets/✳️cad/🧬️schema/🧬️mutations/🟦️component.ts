/** 🧬️ SemioCadMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioCadMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioCadMutationEntry {
  key: string;
  value: string;
}
export interface SemioCadMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioCadMutationEntry[];
}
