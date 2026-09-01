/** 🔺️ rewrite change-rule-layout-point/🔺️diff — mirror of the per-key upsert delta builder. */
import type { ChangeRuleLayoutPoint, LayoutPoint } from "../🟦️component.ts";

export function diff(payload: ChangeRuleLayoutPoint): { ruleLayout: Record<string, LayoutPoint> } {
  return { ruleLayout: { [payload.key]: payload.newPoint } };
}
