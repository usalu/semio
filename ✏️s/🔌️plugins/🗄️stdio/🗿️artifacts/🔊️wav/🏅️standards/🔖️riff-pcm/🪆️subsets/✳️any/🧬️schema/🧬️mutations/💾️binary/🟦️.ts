/** 🧬️ Semio_wav_mutations schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_wav_mutations
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_wav_mutationsEntry {
  key: string;
  value: string;
}
export interface Semio_wav_mutations {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Semio_wav_mutationsEntry[];
}
