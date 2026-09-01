/** 🧬️ SemioFlowMutation schema — real facet mirror of `🧬️mutations/🦀️component.rs`; that Rust
 * file is the source of truth. Discriminated union on the `mutation` tag. */
export interface SemioPoint2 {
  x: number;
  y: number;
}
export interface PortRef {
  node: string;
  port: string;
}
export interface FlowParam {
  key: string;
  value: string;
}
export interface FlowNode {
  id: string;
  kind: string;
  label: string;
  params: FlowParam[];
  position: SemioPoint2;
}
export interface FlowEdge {
  id: string;
  from: PortRef;
  to: PortRef;
  kind: string;
}
export interface SemioFlowSnapshot {
  schema: string;
  nodes: FlowNode[];
  edges: FlowEdge[];
}

export type SemioFlowMutation =
  | { mutation: "setSnapshot"; snapshot: SemioFlowSnapshot }
  | { mutation: "insertNode"; node: FlowNode }
  | { mutation: "removeNode"; id: string }
  | { mutation: "setNodeKind"; id: string; kind: string }
  | { mutation: "setNodeLabel"; id: string; label: string }
  | { mutation: "setNodePosition"; id: string; position: SemioPoint2 }
  | { mutation: "setNodeParam"; id: string; key: string; value: string }
  | { mutation: "removeNodeParam"; id: string; key: string }
  | { mutation: "insertEdge"; edge: FlowEdge }
  | { mutation: "removeEdge"; id: string }
  | { mutation: "setEdgeEndpoints"; id: string; from: PortRef; to: PortRef }
  | { mutation: "setEdgeKind"; id: string; kind: string };
