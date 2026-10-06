/** 🧬️ SemioGraphSnapshot schema — real facet mirror of the Rust `🦀️.rs` sibling. */
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type {SemioValue} from "../../../🔢️value/🧬️schema/📸️snapshot/🟦️.ts";
export type SemioGraphPortKind = "in" | "out" | "inOut";

export interface SemioGraphPort {
  name: string;
  kind: SemioGraphPortKind;
  category: string;
  properties: { key: string; value: SemioValue }[];
}

export interface GraphNodeId { value: string }
export interface GraphEdgeId { value: string }

export interface SemioGraphNode {
  id: GraphNodeId;
  /** freeform node-type tag, mirrors flow's FlowNode.kind */
  kind: string;
  label: string;
  position: { x: Binary64; y: Binary64 };
  width: Binary64;
  height: Binary64;
  ports: SemioGraphPort[];
  properties: { key: string; value: SemioValue }[];
}

/** edges are id-keyed ENTITIES — source/target are ordinary data fields, not an attach handle */
export interface SemioGraphEdge {
  id: GraphEdgeId;
  source: GraphNodeId;
  target: GraphNodeId;
  sourcePort?: string;
  targetPort?: string;
  kind: string;
  label: string;
  properties: { key: string; value: SemioValue }[];
}

export interface SemioGraphSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: SemioGraphNode[];
  /** @state artifact */ edges: SemioGraphEdge[];
}
