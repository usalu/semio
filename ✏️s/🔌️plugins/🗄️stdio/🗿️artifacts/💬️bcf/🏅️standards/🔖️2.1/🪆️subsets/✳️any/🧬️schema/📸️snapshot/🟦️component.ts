/** 🧬️ BcfSnapshot schema. */
export interface BcfEntry {
  name: string;
  data: number[];
}
export interface BcfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: BcfEntry[];
}
