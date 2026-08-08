/** 🧬️ Puzzle5d artifact schema — every field with its state class. */

export interface Puzzle5dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  domain: string;
  /** @state persistent */
  label?: string;
  /** @state persistent */
  meta: Puzzle5dMeta;
  /** @state persistent */
  kindCatalogs?: Puzzle5dKindCatalogs;
  /** @state persistent */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state persistent */
  parts: Puzzle5dPart[];
  /** @state persistent */
  fasteners: Puzzle5dFastener[];
  /** @state shared-ui */
  selectedPartIds: string[];
  /** @state shared-ui */
  selectedGripIds: string[];
  /** @state shared-ui */
  selectedFastenerIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  camera2dX: number;
  /** @state local-ui */
  camera2dY: number;
  /** @state local-ui */
  camera2dZoom: number;
  /** @state local-ui */
  camera3dPositionX: number;
  /** @state local-ui */
  camera3dPositionY: number;
  /** @state local-ui */
  camera3dPositionZ: number;
  /** @state local-ui */
  camera3dTargetX: number;
  /** @state local-ui */
  camera3dTargetY: number;
  /** @state local-ui */
  camera3dTargetZ: number;
  /** @state local-ui */
  camera3dZoom: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  suggestionOffset: number;
  /** @state local-ui */
  overlapBudget: number;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  runtimeExtrasJson: string;
  /** @state preview */
  hoveredPartId?: string;
  /** @state preview */
  previewSeq: number;
}

