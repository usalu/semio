/** 🧬️ RemodelingWorldCamera */
export interface RemodelingWorldCamera {
  /** @state config */
  position: number[];
  /** @state config */
  target: number[];
  /** @state config */
  fov: number;
}

/** 🧬️ RemodelingLayerVisibility */
export interface RemodelingLayerVisibility {
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

/** 🧬️ RemodelingFrameCursor */
export interface RemodelingFrameCursor {
  /** @state config */
  streamId?: string;
  /** @state config */
  frameIndex: number;
}

/** 🧬️ RemodelingConfig */
export interface RemodelingConfig {
  /** @state config */
  camera: RemodelingWorldCamera;
  /** @state config */
  layers: RemodelingLayerVisibility;
  /** @state config */
  frameCursor: RemodelingFrameCursor;
  /** @state config */
  reportTable: string;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}
