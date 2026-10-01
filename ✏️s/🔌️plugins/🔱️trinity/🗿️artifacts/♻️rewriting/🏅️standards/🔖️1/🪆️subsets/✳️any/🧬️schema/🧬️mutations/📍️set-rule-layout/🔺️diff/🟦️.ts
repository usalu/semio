/** 🔺️ rewriting set-rule-layout-points/🔺️diff — mirror of the per-key set/clear delta. */
import type { SetRuleLayoutPoints } from "../🟦️.ts";

export function diff(payload: SetRuleLayoutPoints, base: Readonly<Record<string, { x: number; y: number }>>): { ruleLayout: Record<string, { x: number; y: number } | null> } {
  const ruleLayout: Record<string, { x: number; y: number } | null> = {};
  for (const point of payload.points) if (base[point.key]?.x !== point.x || base[point.key]?.y !== point.y) ruleLayout[point.key] = { x: point.x, y: point.y };
  for (const key of payload.cleared) if (base[key] !== undefined) ruleLayout[key] = null;
  return { ruleLayout };
}
