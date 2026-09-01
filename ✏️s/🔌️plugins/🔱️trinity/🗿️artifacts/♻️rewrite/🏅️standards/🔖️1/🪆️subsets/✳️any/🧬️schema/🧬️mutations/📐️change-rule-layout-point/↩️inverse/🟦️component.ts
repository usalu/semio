/** ↩️ rewrite change-rule-layout-point/↩️inverse — mirror of the BASE-lookup old-point inverse. */
import type { ChangeRuleLayoutPoint, LayoutPoint } from "../🟦️component.ts";
import type { RemoveRuleLayoutPoint } from "../../🗑️remove-rule-layout-point/🟦️component.ts";

export function inverse(payload: ChangeRuleLayoutPoint, basePoint: LayoutPoint | undefined): [ChangeRuleLayoutPoint] | [RemoveRuleLayoutPoint] {
  return basePoint === undefined ? [{ key: payload.key }] : [{ key: payload.key, newPoint: basePoint }];
}
