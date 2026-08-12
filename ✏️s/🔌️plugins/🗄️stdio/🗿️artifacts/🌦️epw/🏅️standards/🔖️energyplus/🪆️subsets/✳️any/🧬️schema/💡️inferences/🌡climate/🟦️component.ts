/** 🌡️ `climate` — epw's hourly dry-bulb temperature min/max/avg, derived from `records`. */

export interface EpwClimateSummary {
  recordCount: number;
  parsedTempCount: number;
  minDryBulbC: number;
  maxDryBulbC: number;
  avgDryBulbC: number;
}
