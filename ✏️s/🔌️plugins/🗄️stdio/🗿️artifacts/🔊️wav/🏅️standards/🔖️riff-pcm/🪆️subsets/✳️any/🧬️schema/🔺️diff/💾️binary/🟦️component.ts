/** 🧬️ Semio_wav_diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_wav_diff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_wav_diffEntry {
  key: string;
  value: string;
}
export interface Semio_wav_diff {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Semio_wav_diffEntry[];
}
