/** 🧬️ Puzzle5d diff schema — sparse field delta. */

export interface Puzzle5dDiff {
  /** @state persistent */
  artifact?: Puzzle5dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  domain?: string;
  /** @state persistent */
  label?: string | null;
  /** @state persistent */
  meta?: Puzzle5dMeta;
  /** @state persistent */
  kindCatalogs?: Puzzle5dKindCatalogs | null;
  /** @state persistent */
  kindCompatibility?: Puzzle5dKindCompatibilityList;
  /** @state persistent */
  parts?: Puzzle5dPartsDelta;
  /** @state persistent */
  fasteners?: Puzzle5dFastenersDelta;
  /** @state shared-ui */
  selectedPartIds?: Puzzle5dStringList;
  /** @state shared-ui */
  selectedGripIds?: Puzzle5dStringList;
  /** @state shared-ui */
  selectedFastenerIds?: Puzzle5dStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  camera2dX?: number;
  /** @state local-ui */
  camera2dY?: number;
  /** @state local-ui */
  camera2dZoom?: number;
  /** @state local-ui */
  camera3dPositionX?: number;
  /** @state local-ui */
  camera3dPositionY?: number;
  /** @state local-ui */
  camera3dPositionZ?: number;
  /** @state local-ui */
  camera3dTargetX?: number;
  /** @state local-ui */
  camera3dTargetY?: number;
  /** @state local-ui */
  camera3dTargetZ?: number;
  /** @state local-ui */
  camera3dZoom?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridFactor?: number;
  /** @state local-ui */
  suggestionOffset?: number;
  /** @state local-ui */
  overlapBudget?: number;
  /** @state local-ui */
  fillCount?: number;
  /** @state local-ui */
  brushCandidateIndex?: number;
  /** @state local-ui */
  lodMode?: string;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  runtimeExtrasJson?: string;
  /** @state preview */
  hoveredPartId?: string | null;
  /** @state preview */
  previewSeq?: number;
}

export interface Puzzle5dStringList { values: string[]; }
export interface Puzzle5dPartsDelta { added: Puzzle5dPart[]; removed: string[]; patched: Puzzle5dPartPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dPartPatchEntry { id: string; patch: Puzzle5dPartPatch; }
export interface Puzzle5dPartPatch { replacement?: Puzzle5dPart; }
export interface Puzzle5dPart { id: string; [key: string]: unknown; }
export interface Puzzle5dFastenersDelta { added: Puzzle5dFastener[]; removed: string[]; patched: Puzzle5dFastenerPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dFastenerPatchEntry { id: string; patch: Puzzle5dFastenerPatch; }
export interface Puzzle5dFastenerPatch { replacement?: Puzzle5dFastener; }
export interface Puzzle5dFastener { id: string; [key: string]: unknown; }
export interface Puzzle5dKindCompatibility { id: string; [key: string]: unknown; }
export interface Puzzle5dArtifact { [key: string]: unknown; }
export interface Puzzle5dMeta { [key: string]: unknown; }
export interface Puzzle5dKindCatalogs { [key: string]: unknown; }

export interface Puzzle5dKindCompatibilityList { values: Puzzle5dKindCompatibility[]; }
export interface Puzzle5dKindCompatibility { [key: string]: unknown; }
