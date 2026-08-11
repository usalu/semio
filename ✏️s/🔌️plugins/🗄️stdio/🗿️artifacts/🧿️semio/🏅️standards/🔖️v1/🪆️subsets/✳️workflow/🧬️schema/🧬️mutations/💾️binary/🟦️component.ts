/** 🧬️ SemioWorkflowMutation schema (binary facet mirror) — real, matches the text facet's shape;
 * the wire ENCODING is the text op line's UTF-8 bytes verbatim (see
 * 🧬️mutations/💾️binary/🥋️component.ksy). */
export interface SemioPoint2 {
  x: number;
  y: number;
}
export interface PortRef {
  node: string;
  port: string;
}
export interface WorkflowParam {
  key: string;
  value: string;
}
export interface WorkflowNode {
  id: string;
  kind: string;
  label: string;
  params: WorkflowParam[];
  position: SemioPoint2;
}
export interface WorkflowEdge {
  id: string;
  from: PortRef;
  to: PortRef;
  kind: string;
}
export interface SemioWorkflowSnapshot {
  schema: string;
  nodes: WorkflowNode[];
  edges: WorkflowEdge[];
}
export type SemioWorkflowMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioWorkflowSnapshot }
  | { mutation: "insertNode"; node: WorkflowNode }
  | { mutation: "removeNode"; id: string }
  | { mutation: "setNodeKind"; id: string; kind: string }
  | { mutation: "setNodeLabel"; id: string; label: string }
  | { mutation: "setNodePosition"; id: string; position: SemioPoint2 }
  | { mutation: "setNodeParam"; id: string; key: string; value: string }
  | { mutation: "removeNodeParam"; id: string; key: string }
  | { mutation: "insertEdge"; edge: WorkflowEdge }
  | { mutation: "removeEdge"; id: string }
  | { mutation: "setEdgeEndpoints"; id: string; from: PortRef; to: PortRef }
  | { mutation: "setEdgeKind"; id: string; kind: string };
