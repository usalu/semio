/** 🧬️ SemioGraphSnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export type SemioGraphPortKind = "in" | "out" | "inOut";

export interface SemioGraphPort {
  name: string;
  kind: SemioGraphPortKind;
}

export interface GraphNodeId { value: string }
export interface GraphEdgeId { value: string }

export interface SemioGraphNode {
  id: GraphNodeId;
  /** freeform node-type tag, mirrors flow's FlowNode.kind */
  kind: string;
  label: string;
  position: { x: number; y: number };
  ports: SemioGraphPort[];
  properties: { key: string; value: unknown }[];
}

/** edges are id-keyed ENTITIES — source/target are ordinary data fields, not an attach handle */
export interface SemioGraphEdge {
  id: GraphEdgeId;
  source: GraphNodeId;
  target: GraphNodeId;
  kind: string;
  label: string;
}

export interface SemioGraphSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: SemioGraphNode[];
  /** @state persistent */ edges: SemioGraphEdge[];
}
