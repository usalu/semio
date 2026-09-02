/** 🧬️ Remodeling artifact schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelingArtifact {
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
  /** @state presence */
  selection: RemodelingUiSelection;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  reportTable: string;
  /** @state presence */
  frameCursor: RemodelingUiFrameCursor;
  /** @state config */
  camera: RemodelingUiCamera;
  /** @state config */
  layers: RemodelingUiLayers;
  /** @state config */
  locale: string;
}

export interface RemodelingUiCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

export interface RemodelingUiSelection {
  mode: string;
  ids: string[];
}

export interface RemodelingUiLayers {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

export interface RemodelingUiFrameCursor {
  streamId?: string;
  frameIndex: number;
}

export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
