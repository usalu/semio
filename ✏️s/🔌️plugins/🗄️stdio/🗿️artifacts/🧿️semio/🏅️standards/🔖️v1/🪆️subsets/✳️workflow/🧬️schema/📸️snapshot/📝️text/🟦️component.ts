/** 🧬️ SemioWorkflowSnapshot schema (TEXT representation facet mirror) — real, matches the facet
 * root's shape; the wire ENCODING is envelope + hex-encoded JSON (see 📝️text/📖️component.grammar.semio). */
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
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: WorkflowNode[];
  /** @state persistent */ edges: WorkflowEdge[];
}
