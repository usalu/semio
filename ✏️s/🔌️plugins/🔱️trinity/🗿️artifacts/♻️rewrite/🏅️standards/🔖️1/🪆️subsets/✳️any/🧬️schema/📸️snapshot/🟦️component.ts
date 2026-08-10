/** 🧬️ Rewrite snapshot schema — persistent fields only. */

export interface RewriteSnapshot {
  /** @state persistent */
  beforeFixtureJson: string;
  /** @state persistent */
  lhsJson: string;
  /** @state persistent */
  rhsJson: string;
  /** @state persistent */
  parameterBindings: Record<string, PropertyValue>;
  /** @state persistent */
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
