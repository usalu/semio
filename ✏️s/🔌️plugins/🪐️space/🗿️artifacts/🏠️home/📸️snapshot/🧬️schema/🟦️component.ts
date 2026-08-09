/** 🧬️ S Home snapshot schema — persistent fields only. */

export interface SHomeSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  catalogGeneration: number;
}
