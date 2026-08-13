/** 🧬️ Puzzle3dPresence */
export interface Puzzle3dPresence {
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedVortexIds: string[];
  /** @state presence */
  selectedAttractionIds: string[];
  /** @state presence */
  selectedTargetVolumeIds: string[];
  /** @state presence */
  selectedReferenceIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredVortexFullId?: string;
  /** @state presence */
  cameraPosition: number[];
  /** @state presence */
  cameraTarget: number[];
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  activeToolId?: string;
}
