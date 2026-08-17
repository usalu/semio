/** 🧬️ RemodelWorldCamera */
export interface RemodelWorldCamera {
  /** @state config */
  position: number[];
  /** @state config */
  target: number[];
  /** @state config */
  fov: number;
}

/** 🧬️ RemodelLayerVisibility */
export interface RemodelLayerVisibility {
  /** @state config */
  mesh: boolean;
  /** @state config */
  dense: boolean;
  /** @state config */
  sparse: boolean;
  /** @state config */
  cameras: boolean;
  /** @state config */
  gcps: boolean;
}

/** 🧬️ RemodelFrameCursor */
export interface RemodelFrameCursor {
  /** @state config */
  streamId?: string;
  /** @state config */
  frameIndex: number;
}

/** 🧬️ RemodelConfig */
export interface RemodelConfig {
  /** @state config */
  camera: RemodelWorldCamera;
  /** @state config */
  layers: RemodelLayerVisibility;
  /** @state config */
  frameCursor: RemodelFrameCursor;
  /** @state config */
  reportTable: string;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}
