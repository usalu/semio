/** 🧬️ Remodel diff schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelDiff {
  /** @state persistent */
  artifact?: RemodelArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  streams?: RemodelMediaStreamList;
  /** @state persistent */
  assets?: Record<string, ImageAsset>;
  /** @state persistent */
  calibration?: CalibrationState;
  /** @state persistent */
  params?: ReconstructionParams;
  /** @state persistent */
  gcps?: RemodelGcpList;
  /** @state persistent */
  job?: ReconstructionJob;
  /** @state persistent */
  results?: ReconstructionResults;
  /** @state shared-ui */
  selection?: RemodelUiSelection;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state shared-ui */
  reportTable?: string;
  /** @state shared-ui */
  frameCursor?: RemodelUiFrameCursor;
  /** @state local-ui */
  camera?: RemodelUiCamera;
  /** @state local-ui */
  layers?: RemodelUiLayers;
  /** @state local-ui */
  locale?: string;
}

export interface RemodelArtifact { [key: string]: unknown }
export interface RemodelMediaStreamList { values: MediaStream[] }
export interface RemodelGcpList { values: GroundControlPoint[] }
export interface RemodelUiCamera { [key: string]: unknown }
export interface RemodelUiSelection { [key: string]: unknown }
export interface RemodelUiLayers { [key: string]: unknown }
export interface RemodelUiFrameCursor { [key: string]: unknown }
export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
