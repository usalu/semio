/** 🔺️ rewriting drag-working-nodes/🔺️diff — mirror of the node moves of the working graph. */
import type { DragWorkingNodes } from "../🟦️.ts";

export function diff(payload: DragWorkingNodes, base: readonly { id: string; x: number; y: number }[]): { id: string; x: number; y: number }[] {
  return base.map((node) => (payload.targets.includes(node.id) ? { ...node, x: node.x + payload.dx, y: node.y + payload.dy } : node));
}
