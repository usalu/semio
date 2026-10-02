/** 🔺️ rewriting disconnect-working-edges/🔺️diff — mirror of the edge removal of the working graph. */
import type { DisconnectWorkingEdges } from "../🟦️.ts";

export function diff(payload: DisconnectWorkingEdges, base: { readonly edges: readonly { id: string }[] }): { edges: { id: string }[] } {
  return { edges: base.edges.filter((edge) => !payload.targets.includes(edge.id)) };
}
