/** 🧬️ Remodel artifact schema — TypeScript mirror of the normative JSON Schema. */

export interface RemodelArtifact {
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
  /** @state shared-ui */
  selection: RemodelUiSelection;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  reportTable: string;
  /** @state shared-ui */
  frameCursor: RemodelUiFrameCursor;
  /** @state local-ui */
  camera: RemodelUiCamera;
  /** @state local-ui */
  layers: RemodelUiLayers;
  /** @state local-ui */
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
