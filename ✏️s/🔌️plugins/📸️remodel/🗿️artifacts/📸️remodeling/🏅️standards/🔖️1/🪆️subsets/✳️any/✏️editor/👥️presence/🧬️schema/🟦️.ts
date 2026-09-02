/** 🧬️ RemodelingPresence */
export interface RemodelingPresence {
  /** @state presence */
  worldCameraPosition: number[];
  /** @state presence */
  worldCameraTarget: number[];
  /** @state presence */
  worldCameraFov: number;
  /** @state presence */
  frameStreamId?: string;
  /** @state presence */
  frameIndex: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  reportTable: string;
}
