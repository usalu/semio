/** 🧬️ WavDiff schema. 🚧 scaffolded by W1b — generic facet mirror; the WavDiff
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavDiffEntry {
  key: string;
  value: string;
}
export interface WavDiff {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: WavDiffEntry[];
}
