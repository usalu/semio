/** 🧬️ Puzzle3d artifact schema — every field with its state class. */

export interface Puzzle3dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  domain: string;
  /** @state persistent */
  meta: Puzzle3dMeta;
  /** @state persistent */
  objects: Puzzle3dObject[];
  /** @state persistent */
  attractions: Puzzle3dAttraction[];
  /** @state persistent */
  targetVolumes: Puzzle3dTargetVolume[];
  /** @state persistent */
  references: Puzzle3dReference[];
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
  activeUtilityId: string;
  /** @state local-ui */
  cameraPositionX: number;
  /** @state local-ui */
  cameraPositionY: number;
  /** @state local-ui */
  cameraPositionZ: number;
  /** @state local-ui */
  cameraTargetX: number;
  /** @state local-ui */
  cameraTargetY: number;
  /** @state local-ui */
  cameraTargetZ: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  gridVisible: boolean;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridSpacing: number;
  /** @state local-ui */
  overlapBudget: number;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  lodAutomatic: boolean;
  /** @state local-ui */
  lodDepthVariable: boolean;
  /** @state local-ui */
  lodManual: number;
  /** @state local-ui */
  proximityRadius: number;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  runtimeExtrasJson: string;
  /** @state preview */
  hoveredObjectId?: string;
  /** @state preview */
  hoveredVortexFullId?: string;
  /** @state preview */
  hoveredKindId?: string;
  /** @state preview */
  previewSeq: number;
}

