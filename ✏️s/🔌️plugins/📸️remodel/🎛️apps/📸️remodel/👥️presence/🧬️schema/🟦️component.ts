/** 🧬️ RemodelPresence */
export interface RemodelPresence {
  /** @state shared-ui */
  selectionMode: string;
  /** @state shared-ui */
  selectionIds: string[];
  /** @state shared-ui */
  worldCameraPosition: number[];
  /** @state shared-ui */
  worldCameraTarget: number[];
  /** @state shared-ui */
  worldCameraFov: number;
  /** @state shared-ui */
  frameStreamId?: string;
  /** @state shared-ui */
  frameIndex: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  reportTable: string;
}
