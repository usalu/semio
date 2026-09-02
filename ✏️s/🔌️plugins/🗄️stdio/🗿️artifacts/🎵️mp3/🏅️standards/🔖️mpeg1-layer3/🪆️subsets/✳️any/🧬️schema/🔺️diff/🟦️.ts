/** 🧬️ Mp3Diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Diff
 * `🦀️.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3DiffEntry {
  key: string;
  value: string;
}
export interface Mp3Diff {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: Mp3DiffEntry[];
}
