/** 🎛 `flat-position` — one named inference: each node's flattened `(u, v)` position. */

export interface JackFlatPositionUv {
  u: number;
  v: number;
}

export interface JackFlatPosition {
  positions: Record<string, JackFlatPositionUv>;
}
