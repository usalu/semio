/** 🧬️ SemioModelMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioModelMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioModelMutationEntry {
  key: string;
  value: string;
}
export interface SemioModelMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioModelMutationEntry[];
}
