/** 🧬️ EnergyModel snapshot schema — persistent fields only. */

export interface EnergyModelSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  modelJson: string;
}
