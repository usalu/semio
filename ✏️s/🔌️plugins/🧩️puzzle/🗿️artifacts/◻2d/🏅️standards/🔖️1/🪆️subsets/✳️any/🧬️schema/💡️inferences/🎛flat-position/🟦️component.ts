/** 🎛 `flat-position` — one named inference: each node's resolved `(x, y)` position after the
 *  compose-parity fastened layout (`Fixed` nodes keep their stored coordinates, `Derived` nodes are
 *  BFS-walked from their edge's connection params). */

export interface Puzzle2dFlatPositionXy {
  x: number;
  y: number;
}

export interface Puzzle2dFlatPosition {
  positions: Record<string, Puzzle2dFlatPositionXy>;
}
