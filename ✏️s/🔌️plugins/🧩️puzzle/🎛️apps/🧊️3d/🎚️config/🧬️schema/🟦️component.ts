/** 🧬️ Puzzle3dConfig */
export interface Puzzle3dConfig {
  /** @state local-ui */
  selection: Puzzle3dSelection;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  hoveredObjectId?: string;
  /** @state local-ui */
  hoveredVortexFullId?: string;
  /** @state local-ui */
  suggestionMenu?: Puzzle3dSuggestionMenu;
  /** @state local-ui */
  overlapBudget: number;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  objectKindWeights: Record<string, number>;
  /** @state local-ui */
  vortexKindWeights: Record<string, number>;
  /** @state local-ui */
  lodAutomatic: boolean;
  /** @state local-ui */
  lodDepthVariable: boolean;
  /** @state local-ui */
  gridVisible: boolean;
  /** @state local-ui */
  lodManual: number;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridSpacing: number;
  /** @state local-ui */
  selectableKinds: Puzzle3dSelectableKinds;
  /** @state local-ui */
  hoveredKindId?: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  proximityRadius: number;
  /** @state local-ui */
  chunkSize: number;
  /** @state local-ui */
  voxelDims: number[];
  /** @state local-ui */
  transformMove: boolean;
  /** @state local-ui */
  transformRotate: boolean;
  /** @state local-ui */
  vortexShow: string;
  /** @state local-ui */
  vortexDirection: string;
  /** @state local-ui */
  sun: WorldSunConfig;
  /** @state local-ui */
  camera: Puzzle3dCamera;
  /** @state local-ui */
  windowOptions: Record<string, Puzzle3dWindowOptions>;
  /** @state local-ui */
  activeUtilityByWindowId: Record<string, string>;
  /** @state local-ui */
  activeToolId?: string;
  /** @state local-ui */
  terminology: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
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
