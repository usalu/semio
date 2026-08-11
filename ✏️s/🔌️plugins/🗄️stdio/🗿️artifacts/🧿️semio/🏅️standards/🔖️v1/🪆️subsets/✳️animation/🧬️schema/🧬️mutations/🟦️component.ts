/** 🧬️ SemioAnimationMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAnimationMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAnimationMutationEntry {
  key: string;
  value: string;
}
export interface SemioAnimationMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAnimationMutationEntry[];
}
