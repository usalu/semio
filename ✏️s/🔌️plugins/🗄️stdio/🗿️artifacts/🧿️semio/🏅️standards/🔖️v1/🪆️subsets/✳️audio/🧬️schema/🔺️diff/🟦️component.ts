/** 🧬️ SemioAudioDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAudioDiff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAudioDiffEntry {
  key: string;
  value: string;
}
export interface SemioAudioDiff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAudioDiffEntry[];
}
