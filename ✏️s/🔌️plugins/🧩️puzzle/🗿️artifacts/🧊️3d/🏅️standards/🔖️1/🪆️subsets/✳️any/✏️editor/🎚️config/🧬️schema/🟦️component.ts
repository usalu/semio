/** 🧬️ Puzzle3dConfig */
export interface Puzzle3dConfig {
  /** @state config */
  selection: Puzzle3dSelection;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredVortexFullId?: string;
  /** @state config */
  suggestionMenu?: Puzzle3dSuggestionMenu;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  fillCount: number;
  /** @state config */
  fillCheckpoint: number[];
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  objectKindWeights: Record<string, number>;
  /** @state config */
  vortexKindWeights: Record<string, number>;
  /** @state config */
  lodAutomatic: boolean;
  /** @state config */
  lodDepthVariable: boolean;
  /** @state config */
  gridVisible: boolean;
  /** @state config */
  lodManual: number;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridSpacing: number;
  /** @state config */
  selectableKinds: Puzzle3dSelectableKinds;
  /** @state config */
  hoveredKindId?: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  proximityRadius: number;
  /** @state config */
  chunkSize: number;
  /** @state config */
  voxelDims: number[];
  /** @state config */
  transformMove: boolean;
  /** @state config */
  transformRotate: boolean;
  /** @state config */
  vortexShow: string;
  /** @state config */
  vortexDirection: string;
  /** @state config */
  sun: WorldSunConfig;
  /** @state config */
  camera: Puzzle3dCamera;
  /** @state config */
  windowOptions: Record<string, Puzzle3dWindowOptions>;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  activeToolId?: string;
  /** @state config */
  terminology: string;
  /** @state config */
  locale: string;
  /** @state config */
  windowIds: string[];
}

export type SelectionSet = string[];

export interface WorldSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface WorldProjectionConfig {
  kind: string;
  orthographicView: string;
  axonometricVariant: string;
  axonometricAngleA: number;
  axonometricAngleB: number;
  axonometricQuadrant: string;
  obliqueVariant: string;
  obliqueAngle: number;
  obliqueDepth: number;
  onePointAxis: string;
  fov: number;
  twoPointShift: number;
  curvilinearFov: number;
  curvilinearStrength: number;
  curvilinearMapping: string;
}

export interface Puzzle3dCamera {
  position: number[];
  target: number[];
  zoom: number;
  up?: number[];
  projection: WorldProjectionConfig;
}

export interface Puzzle3dSelection {
  objectIds: SelectionSet;
  vortexIds: SelectionSet;
  attractionIds: SelectionSet;
  targetVolumeIds: SelectionSet;
  referenceIds: SelectionSet;
}

export interface Puzzle3dSelectableKinds {
  objects: boolean;
  vortices: boolean;
  attractions: boolean;
}

export interface Puzzle3dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
}

export interface Puzzle3dWindowOptions {
  selectionMethod: string;
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  gridVisible: boolean;
  lodManual: number;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  selectableKinds: Puzzle3dSelectableKinds;
  engagementInput: string;
  selectionModeDefault: string;
  proximityRadius: number;
  chunkSize: number;
  voxelDims: number[];
  transformMove: boolean;
  transformRotate: boolean;
  vortexShow: string;
  vortexDirection: string;
  sun: WorldSunConfig;
  camera: Puzzle3dCamera;
}
