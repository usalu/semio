/** mutation payload — mirrors `DragNodes`. A separate PLURAL mutation, never a `Vec` arg bolted
 * onto `moveNode`. */
export interface DragNodes {
  ats: { layer: number; path: number[] }[];
  offset: { x: number; y: number };
}
