/** 🧬️ WavMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the WavMutation
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavMutationEntry {
  key: string;
  value: string;
}
export interface WavMutation {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: WavMutationEntry[];
}
