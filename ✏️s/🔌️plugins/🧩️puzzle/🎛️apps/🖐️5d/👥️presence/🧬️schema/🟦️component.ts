/** 🧬️ Puzzle5dPresence */
export interface Puzzle5dPresence {
  /** @state presence */
  selectedPartIds: string[];
  /** @state presence */
  selectedGripIds: string[];
  /** @state presence */
  selectedFastenerIds: string[];
  /** @state presence */
  hoveredPartId?: string;
  /** @state presence */
  camera2dX: number;
  /** @state presence */
  camera2dY: number;
  /** @state presence */
  camera2dZoom: number;
  /** @state presence */
  camera3dPosition: number[];
  /** @state presence */
  camera3dTarget: number[];
  /** @state presence */
  camera3dZoom: number;
  /** @state presence */
  activeUtilityId: string;
}
