/** 🧬️ SemioFlowSnapshot schema (TEXT representation facet mirror) — real, matches the facet
 * root's shape; the wire ENCODING is envelope + hex-encoded JSON (see 📝️text/📖️component.grammar.semio). */
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
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: FlowNode[];
  /** @state persistent */ edges: FlowEdge[];
}
