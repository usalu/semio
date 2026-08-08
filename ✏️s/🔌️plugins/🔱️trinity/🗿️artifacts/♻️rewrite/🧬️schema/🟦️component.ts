/** 🧬️ Rewrite artifact schema — every field with its state class. */

export interface RewriteArtifact {
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
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  activeHoverVar: string;
  /** @state shared-ui */
  activeSelectVar: string;
  /** @state shared-ui */
  lodModeByWindow: Record<string, string>;
  /** @state local-ui */
  beforePaneCamera: Camera;
  /** @state local-ui */
  reorganizeEpoch: number;
  /** @state local-ui */
  hoverEpoch: number;
  /** @state local-ui */
  selectEpoch: number;
  /** @state local-ui */
  locale: string;
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
