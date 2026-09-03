/** 🧬️ Semio_semio_mutations schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_semio_mutations
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_semio_mutationsEntry {
  key: string;
  value: string;
}
export interface Semio_semio_mutations {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Semio_semio_mutationsEntry[];
}
