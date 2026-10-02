/** 🔺️ rewriting connect-working-ports/🔺️diff — mirror of the wire drawn on the working graph (`null`: an endpoint node is missing). */
import type { ConnectWorkingPorts } from "../🟦️.ts";

type WorkingEdge = { id: string; kind: string; source: string; target: string };

export function diff(payload: ConnectWorkingPorts, base: { readonly nodes: readonly { id: string }[]; readonly edges: readonly WorkingEdge[] }): { edges: WorkingEdge[] } | null {
  const node = (key: string): string => {
    const at = key.indexOf("@");
    return at > 0 && at < key.length - 1 ? key.slice(0, at) : key;
  };
  if (![payload.source, payload.target].every((key) => base.nodes.some((held) => held.id === node(key)))) return null;
  if (base.edges.some((edge) => edge.source === payload.source && edge.target === payload.target)) return { edges: [...base.edges] };
  return { edges: [...base.edges, { id: `${payload.source}->${payload.target}`, kind: payload.kind, source: payload.source, target: payload.target }] };
}
