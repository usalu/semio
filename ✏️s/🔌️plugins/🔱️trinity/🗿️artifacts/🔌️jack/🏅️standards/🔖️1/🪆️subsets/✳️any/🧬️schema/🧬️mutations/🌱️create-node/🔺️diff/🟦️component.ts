/** 🔺️ jack create-node/🔺️diff — mirror of the append-only nodes-added delta builder. */
import type { CreateNode, JackNode } from "../🟦️component.ts";

export function diff(payload: CreateNode): { nodes: { added: JackNode[] } } {
  return { nodes: { added: [payload.node] } };
}
