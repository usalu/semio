/** 🧬️ Jack artifact schema — every field with its state class. */

export interface JackArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  name: string;
  /** @state persistent */
  manifestId?: string;
  /** @state persistent */
  manifest: Manifest;
  /** @state persistent */
  camera: Camera;
  /** @state persistent */
  nodes: Node[];
  /** @state persistent */
  edges: Edge[];
  /** @state persistent */
  rootNodeId?: string;
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  activeFixtureId: string;
  /** @state shared-ui */
  jackQuery: string;
  /** @state shared-ui */
  lodModeByWindow: Record<string, string>;
  /** @state local-ui */
  viewportCamera: Camera;
  /** @state local-ui */
  jackResultJson: string;
  /** @state local-ui */
  editorEngagementInput: string;
  /** @state local-ui */
  graphEngagementInput: string;
  /** @state local-ui */
  resultsEngagementInput: string;
  /** @state local-ui */
  reorganizeEpoch: number;
  /** @state local-ui */
  editorSelection?: JackEditorSelection;
  /** @state local-ui */
  revision: number;
  /** @state local-ui */
  locale: string;
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
  x: number;
  y: number;
  width: number;
  height: number;
  ports: Port[];
}

export interface Port {
  id: string;
  kind: string;
  direction: string;
}

export interface Edge {
  id: string;
  kind: string;
  source: string;
  target: string;
}

export interface Manifest {
  nodeKinds: ManifestKind[];
  edgeKinds: ManifestKind[];
  portKinds: ManifestPortKind[];
}

export interface ManifestKind {
  name: string;
}

export interface ManifestPortKind {
  name: string;
  direction: string;
}
