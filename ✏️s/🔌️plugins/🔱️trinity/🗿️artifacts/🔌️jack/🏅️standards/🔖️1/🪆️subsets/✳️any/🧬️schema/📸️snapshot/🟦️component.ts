/** 🧬️ Jack snapshot schema — persistent fields only. */

export interface JackSnapshot {
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
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}

export interface Port {
  id: string;
  kind: string;
  direction: string;
  properties: Record<string, PropertyValue>;
}

export interface Node {
  id: string;
  kind: string;
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  properties: Record<string, PropertyValue>;
  ports: Port[];
}

export interface Edge {
  id: string;
  kind: string;
  source: string;
  target: string;
  properties: Record<string, PropertyValue>;
}

export type PropertyValue =
  | null
  | boolean
  | number
  | string
  | PropertyValue[]
  | { [key: string]: PropertyValue };

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
