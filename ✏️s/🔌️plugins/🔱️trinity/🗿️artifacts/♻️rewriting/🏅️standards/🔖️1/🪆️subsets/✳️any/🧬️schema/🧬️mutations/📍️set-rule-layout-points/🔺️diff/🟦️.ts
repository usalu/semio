/** 🔺️ rewriting set-rule-layout-points/🔺️diff — mirror of the per-key set/clear delta. */
import type {LayoutPoint} from "../../../🟦️.ts";
import type { SetRuleLayoutPoints } from "../🟦️.ts";

export function diff(payload: SetRuleLayoutPoints, base: Readonly<Record<string, LayoutPoint>>): { ruleLayout: Record<string, LayoutPoint | null> } {
  const ruleLayout: Record<string, LayoutPoint | null> = {};
  for (const point of payload.points) if (base[point.key]?.x.bits !== point.x.bits || base[point.key]?.y.bits !== point.y.bits) ruleLayout[point.key] = { x: point.x, y: point.y };
  for (const key of payload.cleared) if (base[key] !== undefined) ruleLayout[key] = null;
  return { ruleLayout };
}
