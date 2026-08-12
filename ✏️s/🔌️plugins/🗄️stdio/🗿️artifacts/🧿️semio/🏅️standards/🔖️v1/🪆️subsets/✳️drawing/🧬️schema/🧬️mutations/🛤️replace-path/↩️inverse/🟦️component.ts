/** ↩️ inverse for `ReplacePath` — always `ReplacePath` with the captured old segments. */
export interface ReplacePathInverseReplacePath {
  at: { layer: number; path: number[] };
  newSegments: unknown[];
}
