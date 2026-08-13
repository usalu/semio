/** 💡️ EnergyModel inference schema — opaque-container census of the persisted `modelJson` body. */

export interface EnergyModelEntries {
  entryCount: number;
  byteSize: number;
  contentDigest: string;
}

export interface EnergyModelInference {
  /** @derived */
  entries: EnergyModelEntries;
}
