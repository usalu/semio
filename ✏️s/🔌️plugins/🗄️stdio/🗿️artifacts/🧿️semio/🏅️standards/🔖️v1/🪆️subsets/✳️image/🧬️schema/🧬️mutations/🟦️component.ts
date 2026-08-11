/** 🧬️ SemioImageMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioImageMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioImageMutationEntry {
  key: string;
  value: string;
}
export interface SemioImageMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioImageMutationEntry[];
}
