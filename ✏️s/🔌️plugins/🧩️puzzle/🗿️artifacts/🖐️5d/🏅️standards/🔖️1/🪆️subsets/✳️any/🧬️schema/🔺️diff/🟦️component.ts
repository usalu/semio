/** 🧬️ Puzzle5d nested schema types (design-parity). */

/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🧬️ Puzzle5d diff schema — sparse field delta. */

export interface Puzzle5dDiff {
  /** @state artifact */
  artifact?: Puzzle5dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  label?: string | null;
  /** @state artifact */
  meta?: Puzzle5dMeta;
  /** @state artifact */
  kindCatalogs?: Puzzle5dKindCatalogs | null;
  /** @state artifact */
  kindCompatibility?: Puzzle5dKindCompatibilityList;
  /** @state artifact */
  parts?: Puzzle5dPartsDelta;
  /** @state artifact */
  fasteners?: Puzzle5dFastenersDelta;
  /** @state presence */
  selectedPartIds?: Puzzle5dStringList;
  /** @state presence */
  selectedGripIds?: Puzzle5dStringList;
  /** @state presence */
  selectedFastenerIds?: Puzzle5dStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  camera2dX?: number;
  /** @state config */
  camera2dY?: number;
  /** @state config */
  camera2dZoom?: number;
  /** @state config */
  camera3dPositionX?: number;
  /** @state config */
  camera3dPositionY?: number;
  /** @state config */
  camera3dPositionZ?: number;
  /** @state config */
  camera3dTargetX?: number;
  /** @state config */
  camera3dTargetY?: number;
  /** @state config */
  camera3dTargetZ?: number;
  /** @state config */
  camera3dZoom?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridFactor?: number;
  /** @state config */
  suggestionOffset?: number;
  /** @state config */
  overlapBudget?: number;
  /** @state config */
  fillCount?: number;
  /** @state config */
  brushCandidateIndex?: number;
  /** @state config */
  lodMode?: string;
  /** @state config */
  locale?: string;
  /** @state config */
  runtimeExtrasJson?: string;
  /** @state artifact */
  hoveredPartId?: string | null;
  /** @state artifact */
  previewSeq?: number;
}

export interface Puzzle5dStringList { values: string[]; }
export interface Puzzle5dPartsDelta { added: Puzzle5dPart[]; removed: string[]; patched: Puzzle5dPartPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dPartPatchEntry { id: string; patch: Puzzle5dPartPatch; }
export interface Puzzle5dPartPatch { replacement?: Puzzle5dPart; }
export interface Puzzle5dPart { id: string; partKind?: string; anchor?: Puzzle5dPartAnchor; [key: string]: unknown; }
export interface Puzzle5dFastenersDelta { added: Puzzle5dFastener[]; removed: string[]; patched: Puzzle5dFastenerPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dFastenerPatchEntry { id: string; patch: Puzzle5dFastenerPatch; }
export interface Puzzle5dFastenerPatch { replacement?: Puzzle5dFastener; }
export interface Puzzle5dFastener { id: string; source?: string; target?: string; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; x?: number; y?: number; [key: string]: unknown; }
export interface Puzzle5dKindCompatibility { source?: string; target?: string; bidirectional?: boolean; important?: boolean; specificity?: Puzzle5dCompatSpecificity; [key: string]: unknown; }
export interface Puzzle5dArtifact { [key: string]: unknown; }
export interface Puzzle5dMeta { [key: string]: unknown; }
export interface Puzzle5dKindCatalogs { [key: string]: unknown; }

export interface Puzzle5dKindCompatibilityList { values: Puzzle5dKindCompatibility[]; }
export interface Puzzle5dKindCompatibility { [key: string]: unknown; }
