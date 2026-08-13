/** 🧬️ Jack diff schema — sparse field delta. */

export interface JackDiff {
  /** @state artifact */
  artifact?: JackArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  name?: string;
  /** @state artifact */
  manifestId?: string | null;
  /** @state artifact */
  manifest?: Manifest;
  /** @state artifact */
  camera?: Camera;
  /** @state artifact */
  nodes?: JackNodesDelta;
  /** @state artifact */
  edges?: JackEdgesDelta;
  /** @state artifact */
  rootNodeId?: string | null;
  /** @state presence */
  selectedNodeIds?: JackStringList;
  /** @state presence */
  activeFixtureId?: string;
  /** @state presence */
  jackQuery?: string;
  /** @state presence */
  lodModeByWindow?: Record<string, string | null>;
  /** @state config */
  viewportCamera?: Camera;
  /** @state config */
  jackResultJson?: string;
  /** @state config */
  editorEngagementInput?: string;
  /** @state config */
  graphEngagementInput?: string;
  /** @state config */
  resultsEngagementInput?: string;
  /** @state config */
  reorganizeEpoch?: number;
  /** @state config */
  editorSelection?: JackEditorSelection | null;
  /** @state config */
  revision?: number;
  /** @state config */
  locale?: string;
}

export interface JackStringList {
  values: string[];
}

export interface JackNodesDelta {
  added: Node[];
  removed: string[];
  patched: JackNodePatchEntry[];
  reordered?: string[];
}

export interface JackNodePatchEntry {
  id: string;
  patch: JackNodePatch;
}

export interface JackNodePatch {
  name?: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

export interface JackEdgesDelta {
  added: Edge[];
  removed: string[];
  patched: JackEdgePatchEntry[];
  reordered?: string[];
}

export interface JackEdgePatchEntry {
  id: string;
  patch: JackEdgePatch;
}

export interface JackEdgePatch {
  key?: string;
  valueJson?: string | null;
}

export interface JackArtifact {
  schema: string;
  name: string;
  nodes: Node[];
  edges: Edge[];
}

export interface JackEditorSelection {
  start: number;
  end: number;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}

export interface Node {
  id: string;
  kind: string;
  name: string;
}

export interface Edge {
  id: string;
  kind: string;
  source: string;
  target: string;
}

export interface Manifest {
  nodeKinds: { name: string }[];
}
