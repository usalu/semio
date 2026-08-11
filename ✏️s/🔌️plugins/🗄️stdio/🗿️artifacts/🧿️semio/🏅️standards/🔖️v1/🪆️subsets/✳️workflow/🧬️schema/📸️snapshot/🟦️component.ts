/** 🧬️ SemioWorkflowSnapshot schema — real facet mirror of `📸️snapshot/🦀️component.rs`; that Rust
 * file is the source of truth. */
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
