/** 📍️ Relative rewriting `set-rule-layout-points` payload mirror of `SetRuleLayoutPoints`. */
export interface RuleLayoutPlacement {
  key: string;
  x: number;
  y: number;
}

export interface SetRuleLayoutPoints {
  points: RuleLayoutPlacement[];
  cleared: string[];
}
