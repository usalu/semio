/** 💡️ Puzzle2d inference schema — flatPosition (BFS-resolved node position) per node. */

export interface Puzzle2dFlatPositionXy {
  x: number;
  y: number;
}

export interface Puzzle2dFlatPosition {
  positions: Record<string, Puzzle2dFlatPositionXy>;
}

export interface Puzzle2dInference {
  /** @state inferred */
  flatPosition: Puzzle2dFlatPosition;
}
