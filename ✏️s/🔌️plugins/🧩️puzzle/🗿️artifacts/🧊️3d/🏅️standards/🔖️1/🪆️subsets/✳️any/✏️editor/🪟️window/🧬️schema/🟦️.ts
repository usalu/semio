export interface Puzzle3dSelectableKinds {
  objects: boolean;
  vortices: boolean;
  attractions: boolean;
}

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
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  up?: [number, number, number] | null;
  projection: WorldProjectionConfig;
}

export interface Puzzle3dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
  vortexFullId: string;
}

export interface Puzzle3dWindowConfig {
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  gridVisible: boolean;
  lodManual: number;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  selectableKinds: Puzzle3dSelectableKinds;
  proximityRadius: number;
  chunkSize: number;
  voxelDims: [number, number, number];
  transformMove: boolean;
  transformRotate: boolean;
  vortexShow: string;
  vortexDirection: string;
  sun: WorldSunConfig;
  camera: Puzzle3dCamera;
}

export interface Puzzle3dWindowTransient {
  suggestionMenu?: Puzzle3dSuggestionMenu | null;
  engagementInput: string;
  brushCandidateIndex: number;
}
