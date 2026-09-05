/** 🧬️ SemioFlowArtifact schema — real facet mirror of `🧬️schema/🦀️component.rs`; that Rust
 * file is the source of truth (mirrors `SemioFlowSnapshot` field for field). */
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
export interface SemioFlowArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: FlowNode[];
  /** @state artifact */ edges: FlowEdge[];
}
