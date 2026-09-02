/** 🧬️ Mp3Mutation schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Mutation
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3MutationEntry {
  key: string;
  value: string;
}
export interface Mp3Mutation {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3MutationEntry[];
}
