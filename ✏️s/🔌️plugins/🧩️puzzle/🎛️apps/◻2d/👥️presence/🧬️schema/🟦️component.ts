/** 🧬️ Puzzle2dPresence */
export interface Puzzle2dPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  cameraX: number;
  /** @state shared-ui */
  cameraY: number;
  /** @state shared-ui */
  cameraZoom: number;
  /** @state shared-ui */
  selectionMethod: string;
  /** @state shared-ui */
  activeUtilityId: string;
}
