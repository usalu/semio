export interface Puzzle2dWindowConfig {
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  lodMode: string;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  suggestionOffset: number;
  proximityRadius: number;
  areaBrushWidth: number;
  areaBrushHeight: number;
  transformMove: boolean;
  transformRotate: boolean;
  selectableNodes: boolean;
  selectableHandles: boolean;
  selectableEdges: boolean;
}

export interface Puzzle2dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
  handleId: string;
}

export interface Puzzle2dWindowTransient {
  engagementInput: string;
  brushCandidateIndex: number;
  brushCandidates: unknown[];
  brushCandidateSourceHandleId: string;
  suggestionMenu?: Puzzle2dSuggestionMenu | null;
}
