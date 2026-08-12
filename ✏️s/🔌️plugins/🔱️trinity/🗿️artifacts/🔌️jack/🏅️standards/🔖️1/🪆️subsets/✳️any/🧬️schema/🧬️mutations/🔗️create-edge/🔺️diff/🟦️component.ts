/** 🔺️ jack create-edge/🔺️diff — mirror of the append-only edges-added delta builder. */
import type { CreateEdge, JackEdge } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: CreateEdge): { edges: { added: JackEdge[] } } {
  return { edges: { added: [payload.edge] } };
}
