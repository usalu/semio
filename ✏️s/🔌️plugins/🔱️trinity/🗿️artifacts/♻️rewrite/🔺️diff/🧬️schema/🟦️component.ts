/** 🧬️ Rewrite diff schema — sparse field delta. */

export interface RewriteDiff {
  /** @state persistent */
  artifact?: RewriteArtifact;
  /** @state persistent */
  beforeFixtureJson?: string;
  /** @state persistent */
  lhsJson?: string;
  /** @state persistent */
  rhsJson?: string;
  /** @state persistent */
  parameterBindings?: Record<string, PropertyValue | null>;
  /** @state persistent */
  ruleLayout?: Record<string, LayoutPoint | null>;
  /** @state shared-ui */
  selectedNodeIds?: RewriteStringList;
  /** @state shared-ui */
  activeHoverVar?: string;
  /** @state shared-ui */
  activeSelectVar?: string;
  /** @state shared-ui */
  lodModeByWindow?: Record<string, string | null>;
  /** @state local-ui */
  beforePaneCamera?: Camera;
  /** @state local-ui */
  reorganizeEpoch?: number;
  /** @state local-ui */
  hoverEpoch?: number;
  /** @state local-ui */
  selectEpoch?: number;
  /** @state local-ui */
  locale?: string;
}

export interface RewriteStringList {
  values: string[];
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
