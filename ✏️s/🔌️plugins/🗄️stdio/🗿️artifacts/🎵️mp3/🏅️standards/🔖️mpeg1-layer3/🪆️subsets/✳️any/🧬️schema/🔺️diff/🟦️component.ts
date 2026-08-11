/** 🧬️ Mp3Diff schema. 🚧 scaffolded by W1b — generic facet mirror; the Mp3Diff
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface Mp3DiffEntry {
  key: string;
  value: string;
}
export interface Mp3Diff {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: Mp3DiffEntry[];
}
