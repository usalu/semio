/** mutation payload — mirrors `ReplacePath`. No-op unless `at` resolves to a `Path` node. */
export interface ReplacePath {
  at: { layer: number; path: number[] };
  newSegments: unknown[];
}
