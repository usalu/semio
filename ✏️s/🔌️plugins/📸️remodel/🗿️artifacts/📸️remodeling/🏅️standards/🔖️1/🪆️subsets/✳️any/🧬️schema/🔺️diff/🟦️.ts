/** 🧬️ Remodeling diff schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelingDiff {
  /** @state artifact */
  artifact?: RemodelingArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  streams?: RemodelingMediaStreamList;
  /** @state artifact */
  assets?: Record<string, ImageAsset>;
  /** @state artifact */
  calibration?: CalibrationState;
  /** @state artifact */
  params?: ReconstructionParams;
  /** @state artifact */
  gcps?: RemodelingGcpList;
  /** @state artifact */
  job?: ReconstructionJob;
  /** @state artifact */
  results?: ReconstructionResults;
  /** @state presence */
  selection?: RemodelingUiSelection;
  /** @state presence */
  activeUtilityId?: string;
  /** @state presence */
  reportTable?: string;
  /** @state presence */
  frameCursor?: RemodelingUiFrameCursor;
  /** @state config */
  camera?: RemodelingUiCamera;
  /** @state config */
  layers?: RemodelingUiLayers;
  /** @state config */
  locale?: string;
}

export interface RemodelingArtifact { [key: string]: unknown }
export interface RemodelingMediaStreamList { values: MediaStream[] }
export interface RemodelingGcpList { values: GroundControlPoint[] }
export interface RemodelingUiCamera { [key: string]: unknown }
export interface RemodelingUiSelection { [key: string]: unknown }
export interface RemodelingUiLayers { [key: string]: unknown }
export interface RemodelingUiFrameCursor { [key: string]: unknown }
export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
