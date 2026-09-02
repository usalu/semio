/** 🔺️ jack delete-edge/🔺️diff — mirror of the id-only edges-removed delta builder. */
import type { DeleteEdge } from "../🟦️.ts";

export function diff(payload: DeleteEdge): { edges: { removed: string[] } } {
  return { edges: { removed: [payload.id] } };
}
