/** 🔺️ rewriting drag-rule-nodes/🔺️diff — mirror of the per-key layout-point upserts. */
import type { DragRuleNodes } from "../🟦️.ts";

export function diff(payload: DragRuleNodes, basePositions: Readonly<Record<string, { x: number; y: number }>>): { ruleLayout: Record<string, { x: number; y: number }> } {
  const ruleLayout: Record<string, { x: number; y: number }> = {};
  for (const id of payload.targets) {
    const base = basePositions[id];
    if (base !== undefined) ruleLayout[id] = { x: base.x + payload.dx, y: base.y + payload.dy };
  }
  return { ruleLayout };
}
