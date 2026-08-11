/** 🧬️ Mp4Mutation schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp4Mutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp4MutationEntry {
  key: string;
  value: string;
}
export interface Mp4Mutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp4MutationEntry[];
}
