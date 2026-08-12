/** mutation payload — mirrors `GroupNodes`. `indices` MUST be a contiguous ascending run of
 * child indices under `parent` (so `ungroup` can restore the exact original membership). */
export interface GroupNodes {
  parent: { layer: number; path: number[] };
  indices: number[];
  transform: { translation: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number; w: number }; scale: { x: number; y: number; z: number } };
}
