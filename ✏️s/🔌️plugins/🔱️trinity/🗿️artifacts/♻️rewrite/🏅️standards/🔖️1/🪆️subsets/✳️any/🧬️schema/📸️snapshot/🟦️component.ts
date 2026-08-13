/** 🧬️ Rewrite snapshot schema — artifact-lane fields only. */

export interface RewriteSnapshot {
  /** @state artifact */
  beforeFixtureJson: string;
  /** @state artifact */
  lhsJson: string;
  /** @state artifact */
  rhsJson: string;
  /** @state artifact */
  parameterBindings: Record<string, PropertyValue>;
  /** @state artifact */
  ruleLayout: Record<string, LayoutPoint>;
}

export type PropertyValue =
  | null
  | boolean
  | number
  | string
  | PropertyValue[]
  | { [key: string]: PropertyValue };

export interface LayoutPoint {
  x: number;
  y: number;
}
