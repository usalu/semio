/** 🧬️ Mp3Mutation schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Mutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3MutationEntry {
  key: string;
  value: string;
}
export interface Mp3Mutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp3MutationEntry[];
}
