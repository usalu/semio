/** 🧬️ Remodel artifact schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelArtifact {
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
  selection: RemodelUiSelection;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  reportTable: string;
  /** @state presence */
  frameCursor: RemodelUiFrameCursor;
  /** @state config */
  camera: RemodelUiCamera;
  /** @state config */
  layers: RemodelUiLayers;
  /** @state config */
  locale: string;
}

export interface RemodelUiCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

export interface RemodelUiSelection {
  mode: string;
  ids: string[];
}

export interface RemodelUiLayers {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

export interface RemodelUiFrameCursor {
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
