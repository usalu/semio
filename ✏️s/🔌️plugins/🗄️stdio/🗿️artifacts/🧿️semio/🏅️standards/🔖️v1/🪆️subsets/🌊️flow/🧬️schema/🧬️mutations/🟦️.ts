/** 🧬️ SemioFlowMutation schema — real facet mirror of `🧬️mutations/🦀️.rs`; that Rust
 * file is the source of truth. Discriminated union on the `mutation` tag. */
import type {SemioPoint2,PortRef,FlowParam,FlowNode,FlowEdge,SemioFlowSnapshot} from "../📸️snapshot/🟦️.ts";
export type {SemioPoint2,PortRef,FlowParam,FlowNode,FlowEdge,SemioFlowSnapshot} from "../📸️snapshot/🟦️.ts";

export type SemioFlowMutation =
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
  | { mutation: "setEdgeKind"; id: string; kind: string }
  | { mutation: "dragNodes"; targets: string[]; dx: SemioPoint2["x"]; dy: SemioPoint2["y"] };
