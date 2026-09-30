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

/** 🛠️ A window's in-flight select-tool gesture between dispatches — ephemeral tool state, never history. */
export interface Puzzle2dSelectToolState {
  states: string[];
  verb: string;
  authoringSeed: string;
  baseRevision: string;
  connect: boolean;
  transaction: { id: string; tool: string };
  entries: { key: string; mutation: { mutation: string } & Record<string, unknown> }[];
}

export interface Puzzle2dWindowTransient {
  engagementInput: string;
  brushCandidateIndex: number;
  brushCandidates: unknown[];
  brushCandidateSourceHandleId: string;
  suggestionMenu?: Puzzle2dSuggestionMenu | null;
  selectTool?: Puzzle2dSelectToolState | null;
}
