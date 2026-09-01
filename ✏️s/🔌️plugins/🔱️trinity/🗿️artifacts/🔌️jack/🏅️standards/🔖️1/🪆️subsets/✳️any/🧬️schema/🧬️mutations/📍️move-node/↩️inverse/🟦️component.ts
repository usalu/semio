/** ↩️ jack move-node/↩️inverse — mirror of the BASE-lookup old-position inverse builder. */
import type { MoveNode } from "../🟦️component.ts";

export function inverse(payload: MoveNode, basePosition: { x: number; y: number } | undefined): MoveNode[] {
  return basePosition === undefined ? [] : [{ id: payload.id, x: basePosition.x, y: basePosition.y }];
}
