/** 🧬️ BinarySnapshot schema. */
export interface BinarySnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ bytes: number[];
}
