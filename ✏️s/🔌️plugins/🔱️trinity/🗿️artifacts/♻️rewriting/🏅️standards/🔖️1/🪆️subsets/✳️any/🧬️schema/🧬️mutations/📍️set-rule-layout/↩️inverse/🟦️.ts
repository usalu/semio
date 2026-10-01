/** ↩️ rewriting set-rule-layout-points/↩️inverse — mirror of the one-row exact undo. */
import type { SetRuleLayoutPoints } from "../🟦️.ts";

export function inverse(payload: SetRuleLayoutPoints, baseLayout: Readonly<Record<string, { x: number; y: number }>>): SetRuleLayoutPoints[] {
  const keys = [...payload.points.map((point) => point.key), ...payload.cleared];
  return [{ points: keys.filter((key) => baseLayout[key] !== undefined).map((key) => ({ key, ...baseLayout[key]! })), cleared: keys.filter((key) => baseLayout[key] === undefined) }];
}
