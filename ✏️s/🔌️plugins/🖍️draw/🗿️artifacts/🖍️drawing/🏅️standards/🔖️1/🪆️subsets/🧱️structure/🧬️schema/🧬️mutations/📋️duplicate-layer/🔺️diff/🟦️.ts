/** 🔺️ Sparse diff builder for `DuplicateLayer` — one insertion of the cloned subtree right after its source,
 * never apply-then-capture. The source, its location and occupied key census are decoded domain facts.
 * The explicit assignment set controls cloning; no commitment is recomputed here. */
import type { DuplicateLayer } from "../🦠️mutation/🟦️.ts";
import type { DrawingLayerNode } from "../../../../../✳️any/🧬️schema/🟦️.ts";
import {cloneDrawingLayerNode} from "../../../../../✳️any/🧬️schema/🪪️identity/🟦️.ts";

export function diff(
  payload: DuplicateLayer,
  source: DrawingLayerNode,
  sourceLocation: { parentId?: string; index: number } | undefined,
  rootLength: number,
  occupiedKeys:readonly string[],
): { layers: { removed: never[]; inserted: Array<{ parentId?: string; index: number; layer: DrawingLayerNode }>; moved: never[]; modified: never[] } } {
  if(source.id!==payload.layerId)throw Error("Drawing duplicate source differs");
  const duplicate=cloneDrawingLayerNode(source," copy",payload.identities);
  if(payload.identities.some(assignment=>occupiedKeys.includes(assignment.target)))throw Error("Drawing duplicate target already exists");
  const target = sourceLocation ? { parentId: sourceLocation.parentId, index: sourceLocation.index + 1 } : { parentId: undefined, index: rootLength };
  return { layers: { removed: [], inserted: [{ ...(target.parentId !== undefined ? { parentId: target.parentId } : {}), index: target.index, layer: duplicate }], moved: [], modified: [] } };
}
