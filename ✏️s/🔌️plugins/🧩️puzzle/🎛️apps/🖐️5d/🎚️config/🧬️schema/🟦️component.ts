/** 🧬️ Puzzle5dConfig */
export interface Puzzle5dConfig {
  /** @state local-ui */
  camera2d: Puzzle5dCamera2d;
  /** @state local-ui */
  camera3d: Puzzle5dCamera3d;
  /** @state local-ui */
  selection: Puzzle5dSelection;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  hoveredPartId?: string;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  overlapBudget: number;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  suggestionOffset: number;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  engagementInputByWindow: Record<string, string>;
  /** @state local-ui */
  objectKindWeights: Record<string, number>;
  /** @state local-ui */
  vortexKindWeights: Record<string, number>;
  /** @state local-ui */
  sun: WorldSunConfig;
  /** @state local-ui */
  activeUtilityByWindowId: Record<string, string>;
  /** @state local-ui */
  terminology: string;
  /** @state local-ui */
  locale: string;
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

export interface Puzzle5dCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle5dCamera3d {
  position: number[];
  target: number[];
  zoom: number;
}

export interface Puzzle5dSelection {
  partIds: SelectionSet;
  gripIds: SelectionSet;
  fastenerIds: SelectionSet;
}
