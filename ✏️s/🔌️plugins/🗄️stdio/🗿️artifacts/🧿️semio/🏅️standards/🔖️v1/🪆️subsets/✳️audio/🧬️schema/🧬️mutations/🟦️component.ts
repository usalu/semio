/** 🧬️ SemioAudioMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAudioMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAudioMutationEntry {
  key: string;
  value: string;
}
export interface SemioAudioMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAudioMutationEntry[];
}
