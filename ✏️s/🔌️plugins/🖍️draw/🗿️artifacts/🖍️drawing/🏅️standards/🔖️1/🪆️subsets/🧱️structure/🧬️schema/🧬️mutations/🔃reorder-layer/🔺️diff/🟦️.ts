/** 🔺️ Sparse diff builder for `ReorderLayer` — one tree-aware move row from the layer's BASE address to its
 * AFTER address, never the layer itself. `from` is the layer's base `(parentId, index)`, resolved by the caller
 * (this mirror does no tree lookups). */
import type { ReorderLayer } from "../🦠️mutation/🟦️.ts";

export interface DrawingLayerAddress { parentId?: string; index: number }

export function diff(payload: ReorderLayer, from: DrawingLayerAddress): { layers: { removed: never[]; inserted: never[]; moved: Array<{ id: string; from: DrawingLayerAddress; to: DrawingLayerAddress }>; modified: never[] } } {
  const to: DrawingLayerAddress = { ...(payload.parentId !== undefined ? { parentId: payload.parentId } : {}), index: payload.index };
  return { layers: { removed: [], inserted: [], moved: [{ id: payload.layerId, from, to }], modified: [] } };
}
