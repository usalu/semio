/** ↩️ rewriting remove-rule-layout-point/↩️inverse — mirror of the BASE-lookup restore inverse. */
import type { RemoveRuleLayoutPoint } from "../🟦️.ts";
import type { ChangeRuleLayoutPoint, LayoutPoint } from "../../📐️change-rule-layout-point/🟦️.ts";

export function inverse(payload: RemoveRuleLayoutPoint, basePoint: LayoutPoint | undefined): ChangeRuleLayoutPoint[] {
  return basePoint === undefined ? [] : [{ key: payload.key, newPoint: basePoint }];
}
