/** 🧬️ Remodel diff schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelDiff {
  /** @state artifact */
  artifact?: RemodelArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  streams?: RemodelMediaStreamList;
  /** @state artifact */
  assets?: Record<string, ImageAsset>;
  /** @state artifact */
  calibration?: CalibrationState;
  /** @state artifact */
  params?: ReconstructionParams;
  /** @state artifact */
  gcps?: RemodelGcpList;
  /** @state artifact */
  job?: ReconstructionJob;
  /** @state artifact */
  results?: ReconstructionResults;
  /** @state presence */
  selection?: RemodelUiSelection;
  /** @state presence */
  activeUtilityId?: string;
  /** @state presence */
  reportTable?: string;
  /** @state presence */
  frameCursor?: RemodelUiFrameCursor;
  /** @state config */
  camera?: RemodelUiCamera;
  /** @state config */
  layers?: RemodelUiLayers;
  /** @state config */
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
