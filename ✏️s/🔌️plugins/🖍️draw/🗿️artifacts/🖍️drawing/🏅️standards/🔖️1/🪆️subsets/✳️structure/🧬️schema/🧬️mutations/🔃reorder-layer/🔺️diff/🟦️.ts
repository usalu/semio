/** 🔺️ Sparse diff builder for `ReorderLayer` — a real handcrafted remove+insert at the new
 * address, never apply-then-capture. `layer` is the source layer's own subtree, resolved by the
 * caller (this mirror does no tree lookups of its own). */
import type { ReorderLayer } from "../🦠️mutation/🟦️.ts";
import type { DrawingLayerNode } from "../../../../../✳️any/🧬️schema/🟦️.ts";

export function diff(payload: ReorderLayer, layer: DrawingLayerNode): { layers: { removed: string[]; added: Array<{ parentId?: string; index: number; layer: DrawingLayerNode }> } } {
  return { layers: { removed: [payload.layerId], added: [{ ...(payload.parentId !== undefined ? { parentId: payload.parentId } : {}), index: payload.index, layer }] } };
}
