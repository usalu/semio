/** 🧬️ SemioFlowSnapshot schema — real facet mirror of `📸️snapshot/🦀️.rs`; that Rust
 * file is the source of truth. */
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export interface SemioPoint2 {
  x: Binary64;
  y: Binary64;
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
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: FlowNode[];
  /** @state artifact */ edges: FlowEdge[];
}
