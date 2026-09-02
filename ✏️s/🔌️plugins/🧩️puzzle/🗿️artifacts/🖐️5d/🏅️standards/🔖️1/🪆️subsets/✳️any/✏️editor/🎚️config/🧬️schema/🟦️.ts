/** 🧬️ Puzzle5dConfig */
export interface Puzzle5dConfig {
  /** @state config */
  camera2d: Puzzle5dCamera2d;
  /** @state config */
  camera3d: Puzzle5dCamera3d;
  /** @state config */
  selection: Puzzle5dSelection;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredPartId?: string;
  /** @state config */
  fillCount: number;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  overlapBudget: number;
  /** @state config */
  lodMode: string;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  engagementInputByWindow: Record<string, string>;
  /** @state config */
  objectKindWeights: Record<string, number>;
  /** @state config */
  vortexKindWeights: Record<string, number>;
  /** @state config */
  sun: WorldSunConfig;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  terminology: string;
  /** @state config */
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
