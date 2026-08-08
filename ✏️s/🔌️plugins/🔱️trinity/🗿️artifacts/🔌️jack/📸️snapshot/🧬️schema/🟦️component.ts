/** 🧬️ Jack snapshot schema — persistent fields only. */

export interface JackSnapshot {
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
