/** 🧬️ SemioDrawingMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioDrawingMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioDrawingMutationEntry {
  key: string;
  value: string;
}
export interface SemioDrawingMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioDrawingMutationEntry[];
}
