/** ◻️ Puzzle 5D viewer — Board2d window: typed twin of `🦀️.rs`'s view-model. Read-only mirror of the
 * `Board2dScene` payload `render()` produces — no selection, hover, utility or suggestion fields,
 * matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One rim grip of a part, as the board engine's handle record. */
export interface Puzzle5dViewBoard2dHandle {
  id: string;
  handleKind: string;
  angle: number;
  radius: number;
}

/** 👁️ One part, read straight off `Puzzle5dSnapshot.parts`, as the board engine's node record. */
export interface Puzzle5dViewBoard2dNode {
  id: string;
  nodeKind: string;
  shape: string;
  x: number;
  y: number;
  text: string;
  handles: Puzzle5dViewBoard2dHandle[];
  radius?: number;
  width?: number;
  height?: number;
  iconKind?: string;
  hidden?: boolean;
}

/** 👁️ One fastener, as the board engine's edge record. */
export interface Puzzle5dViewBoard2dEdge {
  id: string;
  edgeKind: string;
  source: string;
  target: string;
}

/** 👁️ The Board2d window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `Puzzle5dSnapshot`: a viewer has no runtime, config or utility state). */
export interface Puzzle5dViewBoard2dViewModel {
  windowKindId: "puzzle5d-view-2d";
  bodyKey: "puzzle.5d.view.2d";
  nodes: Puzzle5dViewBoard2dNode[];
  edges: Puzzle5dViewBoard2dEdge[];
  interactive: false;
}

export const PUZZLE5D_VIEW_BOARD2D_WINDOW_KIND_ID = "puzzle5d-view-2d" as const;
export const PUZZLE5D_VIEW_BOARD2D_BODY_KEY = "puzzle.5d.view.2d" as const;
