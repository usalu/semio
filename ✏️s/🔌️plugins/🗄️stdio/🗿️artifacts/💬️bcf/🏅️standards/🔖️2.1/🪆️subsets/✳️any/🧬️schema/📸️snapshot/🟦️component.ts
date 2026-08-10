/** 🧬️ BcfSnapshot schema. */
export interface BcfEntry {
  name: string;
  data: number[];
}
export interface BcfSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: BcfEntry[];
}
