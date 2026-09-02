/** 🔺️ Sparse diff builder for `ReorderLayer` — a real handcrafted remove+insert at the new
 * address, never apply-then-capture. `layer` is the source layer's own subtree, resolved by the
 * caller (this mirror does no tree lookups of its own). */
import type { ReorderLayer } from "../🦠️mutation/🟦️.ts";
import type { DrawLayerNode } from "../../../../../✳️any/🧬️schema/🟦️.ts";

export function diff(payload: ReorderLayer, layer: DrawLayerNode): { layers: { removed: string[]; added: Array<{ parentId?: string; index: number; layer: DrawLayerNode }> } } {
  return { layers: { removed: [payload.layerId], added: [{ ...(payload.parentId !== undefined ? { parentId: payload.parentId } : {}), index: payload.index, layer }] } };
}
