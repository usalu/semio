/** 🧬️ WavSnapshot schema. 🚧 scaffolded by W1b — generic facet mirror; the WavSnapshot
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface WavSnapshotEntry {
  key: string;
  value: string;
}
export interface WavSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: WavSnapshotEntry[];
}
