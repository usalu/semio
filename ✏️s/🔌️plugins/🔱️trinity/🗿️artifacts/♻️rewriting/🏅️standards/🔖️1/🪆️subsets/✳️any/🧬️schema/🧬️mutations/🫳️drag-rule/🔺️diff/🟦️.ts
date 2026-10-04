/** 🔺️ rewriting drag-rule-nodes/🔺️diff — mirror of the per-key layout-point upserts. */
import {binary64,binary64Value} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {LayoutPoint} from "../../../🟦️.ts";
import type { DragRuleNodes } from "../🟦️.ts";

export function diff(payload: DragRuleNodes, basePositions: Readonly<Record<string, LayoutPoint>>): { ruleLayout: Record<string, LayoutPoint> } {
  const ruleLayout: Record<string, LayoutPoint> = {};
  for (const id of payload.targets) {
    const base = basePositions[id];
    if (base !== undefined) ruleLayout[id] = { x: binary64(binary64Value(base.x) + binary64Value(payload.dx)), y: binary64(binary64Value(base.y) + binary64Value(payload.dy)) };
  }
  return { ruleLayout };
}
