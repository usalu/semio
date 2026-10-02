/** 🔺️ rewriting delete-working-nodes/🔺️diff — mirror of the node and incident-edge removal of the working graph. */
import type { DeleteWorkingNodes } from "../🟦️.ts";

export function diff(payload: DeleteWorkingNodes, base: { readonly nodes: readonly { id: string }[]; readonly edges: readonly { source: string; target: string }[] }): { nodes: { id: string }[]; edges: { source: string; target: string }[] } {
  const node = (key: string): string => {
    const at = key.indexOf("@");
    return at > 0 && at < key.length - 1 ? key.slice(0, at) : key;
  };
  const gone = new Set(payload.targets.filter((id) => base.nodes.some((held) => held.id === id)));
  return { nodes: base.nodes.filter((held) => !gone.has(held.id)), edges: base.edges.filter((edge) => !gone.has(node(edge.source)) && !gone.has(node(edge.target))) };
}
