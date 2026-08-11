/** 🧬️ SemioAudioSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioAudioSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioAudioSnapshotEntry {
  key: string;
  value: string;
}
export interface SemioAudioSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAudioSnapshotEntry[];
}
