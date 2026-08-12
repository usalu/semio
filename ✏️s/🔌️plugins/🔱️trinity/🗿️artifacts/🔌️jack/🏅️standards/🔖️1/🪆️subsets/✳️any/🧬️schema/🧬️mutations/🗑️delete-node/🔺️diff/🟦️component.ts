/** 🔺️ jack delete-node/🔺️diff — mirror of the node+cascade-edges removal delta builder. */
import type { DeleteNode } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: DeleteNode, severedEdgeIds: string[]): { nodes: { removed: string[] }; edges?: { removed: string[] } } {
  return { nodes: { removed: [payload.id] }, ...(severedEdgeIds.length > 0 ? { edges: { removed: severedEdgeIds } } : {}) };
}
