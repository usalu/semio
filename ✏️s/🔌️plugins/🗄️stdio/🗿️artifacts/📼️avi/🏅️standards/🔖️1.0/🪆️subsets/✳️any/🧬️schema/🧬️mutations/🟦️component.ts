/** 🧬️ AviMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the AviMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface AviMutationEntry {
  key: string;
  value: string;
}
export interface AviMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: AviMutationEntry[];
}
