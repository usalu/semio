/** 🧬️ SemioPresentationMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioPresentationMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioPresentationMutationEntry {
  key: string;
  value: string;
}
export interface SemioPresentationMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioPresentationMutationEntry[];
}
