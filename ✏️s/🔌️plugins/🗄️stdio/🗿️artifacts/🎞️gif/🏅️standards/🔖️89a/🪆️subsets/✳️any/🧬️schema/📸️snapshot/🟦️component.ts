/** 🧬️ GifSnapshot schema. */
export interface GifEntry {
  name: string;
  data: number[];
}
export interface GifSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: GifEntry[];
}
