/** 🧬️ Rewrite artifact schema — every field with its state class. */

export interface RewriteArtifact {
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
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  activeHoverVar: string;
  /** @state presence */
  activeSelectVar: string;
  /** @state presence */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  beforePaneCamera: Camera;
  /** @state config */
  reorganizeEpoch: number;
  /** @state config */
  hoverEpoch: number;
  /** @state config */
  selectEpoch: number;
  /** @state config */
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
