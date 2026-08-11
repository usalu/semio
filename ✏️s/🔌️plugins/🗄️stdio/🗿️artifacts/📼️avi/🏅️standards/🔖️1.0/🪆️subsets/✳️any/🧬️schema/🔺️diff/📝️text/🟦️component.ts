/** 🧬️ Semio_avi_diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Semio_avi_diff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Semio_avi_diffEntry {
  key: string;
  value: string;
}
export interface Semio_avi_diff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Semio_avi_diffEntry[];
}
