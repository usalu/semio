/** mutation payload — mirrors `ReorderNodes`. */
export interface ReorderNodes {
  parent: { layer: number; path: number[] };
  from: number;
  to: number;
}
