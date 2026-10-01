/** ↩️ rewriting drag-rule-nodes/↩️inverse — mirror of the one-row exact undo. */
import type { DragRuleNodes } from "../🟦️.ts";
import type { SetRuleLayoutPoints } from "../../📍️set-rule-layout/🟦️.ts";

export function inverse(payload: DragRuleNodes, baseLayout: Readonly<Record<string, { x: number; y: number }>>): SetRuleLayoutPoints[] {
  return [{ points: payload.targets.filter((key) => baseLayout[key] !== undefined).map((key) => ({ key, ...baseLayout[key]! })), cleared: payload.targets.filter((key) => baseLayout[key] === undefined) }];
}
