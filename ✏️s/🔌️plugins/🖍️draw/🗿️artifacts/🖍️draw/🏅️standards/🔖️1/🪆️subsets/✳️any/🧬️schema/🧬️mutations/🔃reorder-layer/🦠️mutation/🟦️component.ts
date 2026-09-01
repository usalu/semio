/** 🔃 Draw mutation — `ReorderLayer` payload mirror: repositions (and optionally re-parents) an
 * existing layer to a FINAL-state `(parentId, index)` address — never spatial. */
export interface ReorderLayer {
  layerId: string;
  parentId?: string;
  index: number;
}
