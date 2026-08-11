/** 🧬️ SemioDocumentMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDocumentMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDocumentMutationEntry {
  key: string;
  value: string;
}
export interface SemioDocumentMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDocumentMutationEntry[];
}
