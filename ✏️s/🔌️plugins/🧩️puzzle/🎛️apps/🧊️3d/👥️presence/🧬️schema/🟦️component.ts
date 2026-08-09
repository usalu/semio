/** 🧬️ Puzzle3dPresence */
export interface Puzzle3dPresence {
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  selectedVortexIds: string[];
  /** @state shared-ui */
  selectedAttractionIds: string[];
  /** @state shared-ui */
  selectedTargetVolumeIds: string[];
  /** @state shared-ui */
  selectedReferenceIds: string[];
  /** @state shared-ui */
  hoveredObjectId?: string;
  /** @state shared-ui */
  hoveredVortexFullId?: string;
  /** @state shared-ui */
  cameraPosition: number[];
  /** @state shared-ui */
  cameraTarget: number[];
  /** @state shared-ui */
  cameraZoom: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  activeToolId?: string;
}
