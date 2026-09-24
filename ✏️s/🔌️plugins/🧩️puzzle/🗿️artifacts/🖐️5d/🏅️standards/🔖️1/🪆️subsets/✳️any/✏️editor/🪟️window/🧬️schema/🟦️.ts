export interface Puzzle5dCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle5dWorldProjectionConfig {
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

export interface Puzzle5dCamera3d {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  up: [number, number, number] | null;
  projection: Puzzle5dWorldProjectionConfig;
}

/** 🎯️ Which entity kinds a pick in this pane may reach. */
export interface Puzzle5dSelectableKinds {
  parts: boolean;
  grips: boolean;
  fasteners: boolean;
}

export interface Puzzle5dWorldSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface Puzzle5dBoardWindowConfig {
  camera2d: Puzzle5dCamera2d;
  lodMode: string;
  suggestionOffset: number;
  gridSnapEnabled: boolean;
  gridFactor: number;
  gridVisible: boolean;
  selectableKinds: Puzzle5dSelectableKinds;
}

export interface Puzzle5dWorldWindowConfig {
  camera3d: Puzzle5dCamera3d;
  sun: Puzzle5dWorldSunConfig;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  lodManual: number;
  selectableKinds: Puzzle5dSelectableKinds;
  gripShow: string;
  gripDirection: string;
  transformMove: boolean;
  transformRotate: boolean;
  voxelDims: [number, number, number];
}

/** The grip suggestion menu this window has open: a floating popup, or the context menu's "suggest" submenu. `vortexFullId` is the world host's name for the grip full id. */
export interface Puzzle5dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
  vortexFullId: string;
  submenu: boolean;
}

export interface Puzzle5dWindowTransient {
  suggestionMenu?: Puzzle5dSuggestionMenu | null;
  engagementInput: string;
  brushCandidateIndex: number;
}
