/** 🧬️ Jack diff schema — sparse field delta. */

export interface JackDiff {
  /** @state persistent */
  artifact?: JackArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  name?: string;
  /** @state persistent */
  manifestId?: string | null;
  /** @state persistent */
  manifest?: Manifest;
  /** @state persistent */
  camera?: Camera;
  /** @state persistent */
  nodes?: JackNodesDelta;
  /** @state persistent */
  edges?: JackEdgesDelta;
  /** @state persistent */
  rootNodeId?: string | null;
  /** @state shared-ui */
  selectedNodeIds?: JackStringList;
  /** @state shared-ui */
  activeFixtureId?: string;
  /** @state shared-ui */
  jackQuery?: string;
  /** @state shared-ui */
  lodModeByWindow?: Record<string, string | null>;
  /** @state local-ui */
  viewportCamera?: Camera;
  /** @state local-ui */
  jackResultJson?: string;
  /** @state local-ui */
  editorEngagementInput?: string;
  /** @state local-ui */
  graphEngagementInput?: string;
  /** @state local-ui */
  resultsEngagementInput?: string;
  /** @state local-ui */
  reorganizeEpoch?: number;
  /** @state local-ui */
  editorSelection?: JackEditorSelection | null;
  /** @state local-ui */
  revision?: number;
  /** @state local-ui */
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
