/** 🧬️ GifSnapshot schema. */
export interface GifEntry {
  name: string;
  data: number[];
}
export interface GifSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: GifEntry[];
}
