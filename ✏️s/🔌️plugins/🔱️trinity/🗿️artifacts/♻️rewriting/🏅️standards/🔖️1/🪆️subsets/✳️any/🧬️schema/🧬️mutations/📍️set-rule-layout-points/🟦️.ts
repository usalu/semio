/** 📍️ Relative rewriting `set-rule-layout-points` payload mirror of `SetRuleLayoutPoints`. */
import type {Binary64} from "../../🟦️.ts";
export interface RuleLayoutPlacement {
  key: string;
  x: Binary64;
  y: Binary64;
}

export interface SetRuleLayoutPoints {
  points: RuleLayoutPlacement[];
  cleared: string[];
}
