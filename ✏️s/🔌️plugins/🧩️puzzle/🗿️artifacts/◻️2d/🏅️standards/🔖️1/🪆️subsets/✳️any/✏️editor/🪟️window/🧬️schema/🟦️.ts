export interface Puzzle2dWindowConfig {
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  lodMode: string;
  fillCount: number;
  gridSnapEnabled: boolean;
  gridFactor: number;
  suggestionOffset: number;
}

export interface Puzzle2dWindowTransient {
  engagementInput: string;
  brushCandidateIndex: number;
  brushCandidates: unknown[];
  brushCandidateSourceHandleId: string;
}
