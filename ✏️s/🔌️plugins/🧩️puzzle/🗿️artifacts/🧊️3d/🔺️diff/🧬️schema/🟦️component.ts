/** 🧬️ Puzzle3d diff schema — sparse field delta. */

export interface Puzzle3dDiff {
  /** @state persistent */
  artifact?: Puzzle3dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  domain?: string;
  /** @state persistent */
  meta?: Puzzle3dMeta;
  /** @state persistent */
  objects?: Puzzle3dObjectsDelta;
  /** @state persistent */
  attractions?: Puzzle3dAttractionsDelta;
  /** @state persistent */
  targetVolumes?: Puzzle3dTargetVolumesDelta;
  /** @state persistent */
  references?: Puzzle3dReferencesDelta;
  /** @state shared-ui */
  selectedObjectIds?: Puzzle3dStringList;
  /** @state shared-ui */
  selectedVortexIds?: Puzzle3dStringList;
  /** @state shared-ui */
  selectedAttractionIds?: Puzzle3dStringList;
  /** @state shared-ui */
  selectedTargetVolumeIds?: Puzzle3dStringList;
  /** @state shared-ui */
  selectedReferenceIds?: Puzzle3dStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  cameraPositionX?: number;
  /** @state local-ui */
  cameraPositionY?: number;
  /** @state local-ui */
  cameraPositionZ?: number;
  /** @state local-ui */
  cameraTargetX?: number;
  /** @state local-ui */
  cameraTargetY?: number;
  /** @state local-ui */
  cameraTargetZ?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  selectionModeDefault?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  gridVisible?: boolean;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridSpacing?: number;
  /** @state local-ui */
  overlapBudget?: number;
  /** @state local-ui */
  fillCount?: number;
  /** @state local-ui */
  brushCandidateIndex?: number;
  /** @state local-ui */
  lodAutomatic?: boolean;
  /** @state local-ui */
  lodDepthVariable?: boolean;
  /** @state local-ui */
  lodManual?: number;
  /** @state local-ui */
  proximityRadius?: number;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  runtimeExtrasJson?: string;
  /** @state preview */
  hoveredObjectId?: string | null;
  /** @state preview */
  hoveredVortexFullId?: string | null;
  /** @state preview */
  hoveredKindId?: string | null;
  /** @state preview */
  previewSeq?: number;
}

export interface Puzzle3dStringList { values: string[]; }
export interface Puzzle3dObjectsDelta { added: Puzzle3dObject[]; removed: string[]; patched: Puzzle3dObjectPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dObjectPatchEntry { id: string; patch: Puzzle3dObjectPatch; }
export interface Puzzle3dObjectPatch { replacement?: Puzzle3dObject; }
export interface Puzzle3dObject { id: string; [key: string]: unknown; }
export interface Puzzle3dAttractionsDelta { added: Puzzle3dAttraction[]; removed: string[]; patched: Puzzle3dAttractionPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dAttractionPatchEntry { id: string; patch: Puzzle3dAttractionPatch; }
export interface Puzzle3dAttractionPatch { replacement?: Puzzle3dAttraction; }
export interface Puzzle3dAttraction { id: string; [key: string]: unknown; }
export interface Puzzle3dTargetVolumesDelta { added: Puzzle3dTargetVolume[]; removed: string[]; patched: Puzzle3dTargetVolumePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dTargetVolumePatchEntry { id: string; patch: Puzzle3dTargetVolumePatch; }
export interface Puzzle3dTargetVolumePatch { replacement?: Puzzle3dTargetVolume; }
export interface Puzzle3dTargetVolume { id: string; [key: string]: unknown; }
export interface Puzzle3dReferencesDelta { added: Puzzle3dReference[]; removed: string[]; patched: Puzzle3dReferencePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dReferencePatchEntry { id: string; patch: Puzzle3dReferencePatch; }
export interface Puzzle3dReferencePatch { replacement?: Puzzle3dReference; }
export interface Puzzle3dReference { id: string; [key: string]: unknown; }
export interface Puzzle3dArtifact { [key: string]: unknown; }
export interface Puzzle3dMeta { [key: string]: unknown; }

