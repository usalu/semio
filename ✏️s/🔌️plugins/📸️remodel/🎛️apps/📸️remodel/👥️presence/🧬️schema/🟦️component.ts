/** 🧬️ RemodelPresence */
export interface RemodelPresence {
  /** @state presence */
  selectionMode: string;
  /** @state presence */
  selectionIds: string[];
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
