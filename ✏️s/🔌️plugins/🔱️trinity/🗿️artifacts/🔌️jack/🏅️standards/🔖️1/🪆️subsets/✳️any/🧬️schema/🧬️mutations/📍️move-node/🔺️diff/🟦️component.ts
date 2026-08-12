/** 🔺️ jack move-node/🔺️diff — mirror of the x/y node patch delta builder. */
import type { MoveNode } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: MoveNode): { nodes: { patched: Array<{ id: string; patch: { x: number; y: number } }> } } {
  return { nodes: { patched: [{ id: payload.id, patch: { x: payload.x, y: payload.y } }] } };
}
