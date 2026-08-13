/** 🧬️ Puzzle3d diff schema — sparse field delta. */

export interface Puzzle3dDiff {
  /** @state artifact */
  artifact?: Puzzle3dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  meta?: Puzzle3dMeta;
  /** @state artifact */
  objects?: Puzzle3dObjectsDelta;
  /** @state artifact */
  attractions?: Puzzle3dAttractionsDelta;
  /** @state artifact */
  targetVolumes?: Puzzle3dTargetVolumesDelta;
  /** @state artifact */
  references?: Puzzle3dReferencesDelta;
  /** @state presence */
  selectedObjectIds?: Puzzle3dStringList;
  /** @state presence */
  selectedVortexIds?: Puzzle3dStringList;
  /** @state presence */
  selectedAttractionIds?: Puzzle3dStringList;
  /** @state presence */
  selectedTargetVolumeIds?: Puzzle3dStringList;
  /** @state presence */
  selectedReferenceIds?: Puzzle3dStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  cameraPositionX?: number;
  /** @state config */
  cameraPositionY?: number;
  /** @state config */
  cameraPositionZ?: number;
  /** @state config */
  cameraTargetX?: number;
  /** @state config */
  cameraTargetY?: number;
  /** @state config */
  cameraTargetZ?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  selectionModeDefault?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  gridVisible?: boolean;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridSpacing?: number;
  /** @state config */
  overlapBudget?: number;
  /** @state config */
  fillCount?: number;
  /** @state config */
  brushCandidateIndex?: number;
  /** @state config */
  lodAutomatic?: boolean;
  /** @state config */
  lodDepthVariable?: boolean;
  /** @state config */
  lodManual?: number;
  /** @state config */
  proximityRadius?: number;
  /** @state config */
  locale?: string;
  /** @state config */
  runtimeExtrasJson?: string;
  /** @state artifact */
  hoveredObjectId?: string | null;
  /** @state artifact */
  hoveredVortexFullId?: string | null;
  /** @state artifact */
  hoveredKindId?: string | null;
  /** @state artifact */
  previewSeq?: number;
}

export interface Puzzle3dStringList { values: string[]; }
export interface Puzzle3dObjectsDelta { added: Puzzle3dObject[]; removed: string[]; patched: Puzzle3dObjectPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dObjectPatchEntry { id: string; patch: Puzzle3dObjectPatch; }
export interface Puzzle3dObjectPatch { replacement?: Puzzle3dObject; }
export interface Puzzle3dAttractionsDelta { added: Puzzle3dAttraction[]; removed: string[]; patched: Puzzle3dAttractionPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dAttractionPatchEntry { id: string; patch: Puzzle3dAttractionPatch; }
export interface Puzzle3dAttractionPatch { replacement?: Puzzle3dAttraction; }
export interface Puzzle3dTargetVolumesDelta { added: Puzzle3dTargetVolume[]; removed: string[]; patched: Puzzle3dTargetVolumePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dTargetVolumePatchEntry { id: string; patch: Puzzle3dTargetVolumePatch; }
export interface Puzzle3dTargetVolumePatch { replacement?: Puzzle3dTargetVolume; }
export interface Puzzle3dTargetVolume { id: string; [key: string]: unknown; }
export interface Puzzle3dReferencesDelta { added: Puzzle3dReference[]; removed: string[]; patched: Puzzle3dReferencePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dReferencePatchEntry { id: string; patch: Puzzle3dReferencePatch; }
export interface Puzzle3dReferencePatch { replacement?: Puzzle3dReference; }
export interface Puzzle3dReference { id: string; [key: string]: unknown; }
export interface Puzzle3dArtifact { [key: string]: unknown; }

export type Puzzle3dObjectAnchor = "fixed" | "derived";
export type Puzzle3dCompatSpecificity = "general" | "object" | "attraction" | "vortex" | "cable";

export interface Puzzle3dVortex {
  id: string;
  vortexKind?: string;
  label?: string;
  position: [number, number, number];
  direction?: [number, number, number];
  radius?: number;
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dObject {
  id: string;
  label?: string;
  objectKind?: string;
  anchor?: Puzzle3dObjectAnchor;
  origin: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: number | [number, number, number];
  meshUrl?: string;
  vortices?: Puzzle3dVortex[];
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dAttraction {
  id?: string;
  attracting: string;
  attracted: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}

export interface Puzzle3dRepresentation {
  id: string;
  name: string;
  url: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

export interface Puzzle3dCatalogVortexTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  vortexKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle3dCatalogObjectKind {
  id: string;
  name: string;
  label: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle3dRepresentation[];
  vortices?: Puzzle3dCatalogVortexTemplate[];
  attributes?: Array<{ id?: string; key: string; value: string; definition?: string }>;
  authors?: Array<{ id?: string; name: string; email?: string; role?: string; rank?: number }>;
}

export interface Puzzle3dCatalogVortexKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultCableKind?: string;
}

export interface Puzzle3dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle3dCompatSpecificity;
}

export interface Puzzle3dMeta {
  kindCatalogs?: {
    objects?: Puzzle3dCatalogObjectKind[];
    vortices?: Puzzle3dCatalogVortexKind[];
    cables?: unknown[];
    attractions?: unknown[];
  };
  kindCompatibility?: Puzzle3dKindCompatibility[];
}

export interface Puzzle3dTargetVolume { id: string; [key: string]: unknown; }
export interface Puzzle3dReference { id: string; [key: string]: unknown; }
