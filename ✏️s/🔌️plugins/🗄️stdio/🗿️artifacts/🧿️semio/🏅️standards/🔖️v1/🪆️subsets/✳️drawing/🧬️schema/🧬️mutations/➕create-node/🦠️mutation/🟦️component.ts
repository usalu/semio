/** mutation payload — mirrors `CreateNode`. `NodePath{layer,path}` addresses the parent group. */
export interface CreateNode {
  parent: { layer: number; path: number[] };
  index: number;
  node: unknown;
}
