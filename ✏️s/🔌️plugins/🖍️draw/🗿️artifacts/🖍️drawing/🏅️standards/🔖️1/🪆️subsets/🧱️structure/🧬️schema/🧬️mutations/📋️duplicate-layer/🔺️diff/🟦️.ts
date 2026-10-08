/** 🔺️ Sparse diff builder for `DuplicateLayer` — one insertion of the cloned subtree right after its source,
 * never apply-then-capture. `duplicate`, `sourceLocation` and `rootLength` are resolved by the caller (id hashing
 * and tree lookup are not mirrored here). */
import type { DuplicateLayer } from "../🦠️mutation/🟦️.ts";
import type { DrawingLayerNode } from "../../../../../✳️any/🧬️schema/🟦️.ts";

export function diff(
  _payload: DuplicateLayer,
  duplicate: DrawingLayerNode,
  sourceLocation: { parentId?: string; index: number } | undefined,
  rootLength: number,
): { layers: { removed: never[]; inserted: Array<{ parentId?: string; index: number; layer: DrawingLayerNode }>; moved: never[]; modified: never[] } } {
  const target = sourceLocation ? { parentId: sourceLocation.parentId, index: sourceLocation.index + 1 } : { parentId: undefined, index: rootLength };
  return { layers: { removed: [], inserted: [{ ...(target.parentId !== undefined ? { parentId: target.parentId } : {}), index: target.index, layer: duplicate }], moved: [], modified: [] } };
}
