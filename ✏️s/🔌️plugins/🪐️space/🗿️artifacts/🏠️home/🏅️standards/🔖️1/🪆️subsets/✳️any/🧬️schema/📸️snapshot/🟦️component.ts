/** 🧬️ S Home snapshot schema — persistent fields only. */

export interface SHomeSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  catalogGeneration: number;
}
