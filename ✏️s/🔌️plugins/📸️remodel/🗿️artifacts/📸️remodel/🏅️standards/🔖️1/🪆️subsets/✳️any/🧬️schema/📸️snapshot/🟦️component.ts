/** 🧬️ Remodel snapshot schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  streams: MediaStream[];
  /** @state artifact */
  assets: Record<string, ImageAsset>;
  /** @state artifact */
  calibration: CalibrationState;
  /** @state artifact */
  params: ReconstructionParams;
  /** @state artifact */
  gcps: GroundControlPoint[];
  /** @state artifact */
  job: ReconstructionJob;
  /** @state artifact */
  results: ReconstructionResults;
}

export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
