/** 🧬️ SemioVideoMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioVideoMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioVideoMutationEntry {
  key: string;
  value: string;
}
export interface SemioVideoMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioVideoMutationEntry[];
}
