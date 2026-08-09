/** 🧬️ Puzzle5dPresence */
export interface Puzzle5dPresence {
  /** @state shared-ui */
  selectedPartIds: string[];
  /** @state shared-ui */
  selectedGripIds: string[];
  /** @state shared-ui */
  selectedFastenerIds: string[];
  /** @state shared-ui */
  hoveredPartId?: string;
  /** @state shared-ui */
  camera2dX: number;
  /** @state shared-ui */
  camera2dY: number;
  /** @state shared-ui */
  camera2dZoom: number;
  /** @state shared-ui */
  camera3dPosition: number[];
  /** @state shared-ui */
  camera3dTarget: number[];
  /** @state shared-ui */
  camera3dZoom: number;
  /** @state shared-ui */
  activeUtilityId: string;
}
