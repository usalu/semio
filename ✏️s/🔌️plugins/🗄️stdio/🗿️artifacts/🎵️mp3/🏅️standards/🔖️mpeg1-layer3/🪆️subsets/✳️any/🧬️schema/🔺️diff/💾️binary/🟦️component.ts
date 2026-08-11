/** 🧬️ Semio_mp3_diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_mp3_diff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_mp3_diffEntry {
  key: string;
  value: string;
}
export interface Semio_mp3_diff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_mp3_diffEntry[];
}
