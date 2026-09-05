/** 🔺️ Sparse diff builder for `DuplicateLayer` — real handcrafted insert of the cloned subtree
 * right after its source, never apply-then-capture. `duplicate` and `sourceLocation` are resolved
 * by the caller (id hashing and tree lookup are not mirrored here). */
import type { DuplicateLayer } from "../🦠️mutation/🟦️.ts";
import type { DrawingLayerNode } from "../../../../../✳️any/🧬️schema/🟦️.ts";

export function diff(
  payload: DuplicateLayer,
  duplicate: DrawingLayerNode,
  sourceLocation: { parentId?: string; index: number } | undefined,
  rootLayerCount: number,
): { layers: { added: Array<{ parentId?: string; index: number; layer: DrawingLayerNode }> } } {
  const target = sourceLocation ? { parentId: sourceLocation.parentId, index: sourceLocation.index + 1 } : { parentId: undefined, index: rootLayerCount };
  return { layers: { added: [{ ...(target.parentId !== undefined ? { parentId: target.parentId } : {}), index: target.index, layer: duplicate }] } };
}
