/** 🔺️ rewriting remove-rule-layout-point/🔺️diff — mirror of the per-key clear delta builder. */
import type { RemoveRuleLayoutPoint } from "../🟦️.ts";

export function diff(payload: RemoveRuleLayoutPoint): { ruleLayout: Record<string, null> } {
  return { ruleLayout: { [payload.key]: null } };
}
