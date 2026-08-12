/** mutation payload — mirrors `UnflattenNode`. `original` is a captured subtree (usually from
 * `flattenNode`'s own inverse). */
export interface UnflattenNode {
  at: { layer: number; path: number[] };
  original: unknown;
}
