/** 🧬️ WavMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the WavMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavMutationEntry {
  key: string;
  value: string;
}
export interface WavMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: WavMutationEntry[];
}
