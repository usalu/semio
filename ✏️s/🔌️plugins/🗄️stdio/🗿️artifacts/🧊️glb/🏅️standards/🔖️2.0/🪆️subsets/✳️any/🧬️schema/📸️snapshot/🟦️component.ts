/** 🧬️ GlbSnapshot schema. */
export interface GlbEntry {
  name: string;
  data: number[];
}
export interface GlbSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: GlbEntry[];
}
