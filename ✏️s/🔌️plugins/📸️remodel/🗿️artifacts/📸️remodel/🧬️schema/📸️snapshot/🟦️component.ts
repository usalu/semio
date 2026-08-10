/** 🧬️ Remodel snapshot schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  streams: MediaStream[];
  /** @state persistent */
  assets: Record<string, ImageAsset>;
  /** @state persistent */
  calibration: CalibrationState;
  /** @state persistent */
  params: ReconstructionParams;
  /** @state persistent */
  gcps: GroundControlPoint[];
  /** @state persistent */
  job: ReconstructionJob;
  /** @state persistent */
  results: ReconstructionResults;
}

export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
