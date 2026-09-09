export interface Puzzle5dCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle5dCamera3d {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
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
  fillCount: number;
  lodMode: string;
  suggestionOffset: number;
  gridSnapEnabled: boolean;
  gridFactor: number;
}

export interface Puzzle5dWorldWindowConfig {
  camera3d: Puzzle5dCamera3d;
  sun: Puzzle5dWorldSunConfig;
}

export interface Puzzle5dWindowTransient {
  engagementInput: string;
  brushCandidateIndex: number;
}
