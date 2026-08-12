/** 📐️ rewrite change-rule-layout-point/🦠️mutation — payload mirror of `ChangeRuleLayoutPoint`. */
export interface LayoutPoint {
  x: number;
  y: number;
}

export interface ChangeRuleLayoutPoint {
  key: string;
  newPoint: LayoutPoint;
}
