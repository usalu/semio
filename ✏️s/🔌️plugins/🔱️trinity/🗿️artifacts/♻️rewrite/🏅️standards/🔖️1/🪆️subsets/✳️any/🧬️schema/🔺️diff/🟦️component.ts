/** 🧬️ Rewrite diff schema — sparse field delta. */

export interface RewriteDiff {
  /** @state artifact */
  artifact?: RewriteArtifact;
  /** @state artifact */
  beforeFixtureJson?: string;
  /** @state artifact */
  lhsJson?: string;
  /** @state artifact */
  rhsJson?: string;
  /** @state artifact */
  parameterBindings?: Record<string, PropertyValue | null>;
  /** @state artifact */
  ruleLayout?: Record<string, LayoutPoint | null>;
  /** @state presence */
  lodModeByWindow?: Record<string, string | null>;
  /** @state config */
  beforePaneCamera?: Camera;
  /** @state config */
  reorganizeEpoch?: number;
  /** @state config */
  locale?: string;
}

export interface RewriteArtifact {
  beforeFixtureJson: string;
  lhsJson: string;
  rhsJson: string;
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

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
