/** 🧬️ Semio_wav_mutations schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_wav_mutations
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_wav_mutationsEntry {
  key: string;
  value: string;
}
export interface Semio_wav_mutations {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_wav_mutationsEntry[];
}
