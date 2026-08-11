/** 🧬️ Semio_mp3_mutations schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_mp3_mutations
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_mp3_mutationsEntry {
  key: string;
  value: string;
}
export interface Semio_mp3_mutations {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_mp3_mutationsEntry[];
}
