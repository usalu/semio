/** 🧬️ RemodelWorldCamera */
export interface RemodelWorldCamera {
  /** @state local-ui */
  position: number[];
  /** @state local-ui */
  target: number[];
  /** @state local-ui */
  fov: number;
}

/** 🧬️ RemodelSelection */
export interface RemodelSelection {
  /** @state local-ui */
  mode: string;
  /** @state local-ui */
  ids: string[];
}

/** 🧬️ RemodelLayerVisibility */
export interface RemodelLayerVisibility {
  /** @state local-ui */
  mesh: boolean;
  /** @state local-ui */
  dense: boolean;
  /** @state local-ui */
  sparse: boolean;
  /** @state local-ui */
  cameras: boolean;
  /** @state local-ui */
  gcps: boolean;
}

/** 🧬️ RemodelFrameCursor */
export interface RemodelFrameCursor {
  /** @state local-ui */
  streamId?: string;
  /** @state local-ui */
  frameIndex: number;
}

/** 🧬️ RemodelConfig */
export interface RemodelConfig {
  /** @state local-ui */
  camera: RemodelWorldCamera;
  /** @state local-ui */
  selection: RemodelSelection;
  /** @state local-ui */
  layers: RemodelLayerVisibility;
  /** @state local-ui */
  frameCursor: RemodelFrameCursor;
  /** @state local-ui */
  reportTable: string;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}
