/** 🔺️ SemioFlowDiff schema — real facet mirror of `🔺️diff/🦀️.rs`; that Rust file is
 * the source of truth. Built on the shared name-keyed triple shape (`engine::triples`). */
export interface NamedModified<K, D> {
  key: K;
  diff: D;
}
export interface NamedTripleDiff<K, D, T> {
  removed: K[];
  modified: NamedModified<K, D>[];
  added: T[];
}
import type {SemioPoint2,PortRef,FlowParam,FlowNode,FlowEdge} from "../📸️snapshot/🟦️.ts";
export type {SemioPoint2,PortRef,FlowParam,FlowNode,FlowEdge} from "../📸️snapshot/🟦️.ts";
export interface FlowParamDiff {
  value?: string;
}
export type FlowParamsDiff = NamedTripleDiff<string, FlowParamDiff, FlowParam>;
export interface FlowNodeDiff {
  kind?: string;
  label?: string;
  params?: FlowParamsDiff;
  position?: SemioPoint2;
}
export type FlowNodesDiff = NamedTripleDiff<string, FlowNodeDiff, FlowNode>;
export interface FlowEdgeDiff {
  from?: PortRef;
  to?: PortRef;
  kind?: string;
}
export type FlowEdgesDiff = NamedTripleDiff<string, FlowEdgeDiff, FlowEdge>;
export interface SemioFlowDiff {
  /** @state artifact */ nodes?: FlowNodesDiff;
  /** @state artifact */ edges?: FlowEdgesDiff;
}
