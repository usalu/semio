/** 🔺️ rewriting patch-working-nodes/🔺️diff — mirror of the field patch of the working graph. */
import type { PatchWorkingNodes } from "../🟦️.ts";

export function diff(payload: PatchWorkingNodes, base: readonly { id: string; name: string; kind: string }[]): { id: string; name: string; kind: string }[] {
  return base.map((node) => (payload.targets.includes(node.id) ? { ...node, [payload.field]: payload.value.trim() } : node));
}
