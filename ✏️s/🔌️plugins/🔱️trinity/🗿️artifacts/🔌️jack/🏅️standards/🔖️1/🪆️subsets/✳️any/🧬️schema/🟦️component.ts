/** 🧬️ Jack artifact schema — every field with its state class. */

export interface JackArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  name: string;
  /** @state artifact */
  manifestId?: string;
  /** @state artifact */
  manifest: Manifest;
  /** @state artifact */
  camera: Camera;
  /** @state artifact */
  nodes: Node[];
  /** @state artifact */
  edges: Edge[];
  /** @state artifact */
  rootNodeId?: string;
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  activeFixtureId: string;
  /** @state presence */
  jackQuery: string;
  /** @state presence */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  viewportCamera: Camera;
  /** @state config */
  jackResultJson: string;
  /** @state config */
  editorEngagementInput: string;
  /** @state config */
  graphEngagementInput: string;
  /** @state config */
  resultsEngagementInput: string;
  /** @state config */
  reorganizeEpoch: number;
  /** @state config */
  editorSelection?: JackEditorSelection;
  /** @state config */
  revision: number;
  /** @state config */
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
