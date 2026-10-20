/** ↩️ rewriting set-rule-layout-points/↩️inverse — mirror of the one-row exact undo. */
import type {LayoutPoint} from "../../../🟦️.ts";
import type { SetRuleLayoutPoints } from "../🟦️.ts";

export function inverse(payload: SetRuleLayoutPoints, baseLayout: Readonly<Record<string, LayoutPoint>>): SetRuleLayoutPoints[] {
  const keys = [...payload.points.map((point) => point.key), ...payload.cleared];
  return [{ points: keys.filter((key) => baseLayout[key] !== undefined).map((key) => ({ key, ...baseLayout[key]! })), cleared: keys.filter((key) => baseLayout[key] === undefined) }];
}
