/** 🎛️ Flattened positions accumulate intrinsic displacements over retained endpoints. */
import type {JackArtifact} from "../../🟦️.ts";
import type {JackFlatPosition,JackFlatPositionUv} from "../🟦️.ts";
import {retainedJackGraph,type JackGraphQuery} from "../🪆️graph-query/🟦️.ts";
import {ValueError} from "@semio-tech/framework/value/refusal";
export type {JackFlatPosition,JackFlatPositionUv};
/** 🧩️ Deterministic root and component walks preserve each edge's selected displacement. */
export function inferJackFlatPosition(parent: JackArtifact, children: ReadonlyMap<string, JackGraphQuery>): JackFlatPosition {
  const child = retainedJackGraph(parent, children);
  const ids = [...child.nodeIds()].sort();
  const members = new Set(ids);
  if (members.size !== ids.length) throw new ValueError("invalidValue", "Jack inference requires unique node identities");
  const seed = parent.rootNodeId ?? ids[0];
  if (seed !== undefined && !members.has(seed)) throw new ValueError("invalidValue", "Jack flattened root requires retained node");

  const edges = [...child.edges()].sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0);
  const edgeIds = new Set<string>();
  const incoming = new Set<string>();
  const outgoing = new Map<string, {target: string; offset: JackFlatPositionUv}[]>();
  for (const edge of edges) {
    if (edgeIds.has(edge.id)) throw new ValueError("invalidValue", "Jack inference requires unique edge identities");
    if (!members.has(edge.source) || !members.has(edge.target)) throw new ValueError("invalidValue", "Jack inference edge requires retained endpoints");
    edgeIds.add(edge.id);
    incoming.add(edge.target);
    const {u, v} = child.displacement(edge.id);
    const branches = outgoing.get(edge.source) ?? [];
    branches.push({target: edge.target, offset: finitePosition(u, v)});
    outgoing.set(edge.source, branches);
  }

  const positions: Record<string, JackFlatPositionUv> = Object.create(null);
  const visit = (root: string) => {
    if (Object.hasOwn(positions, root)) return;
    positions[root] = {u: 0, v: 0};
    const stack = [root];
    while (stack.length) {
      const source = stack.pop()!;
      const held = positions[source]!;
      for (const {target, offset} of outgoing.get(source) ?? []) {
        if (Object.hasOwn(positions, target)) continue;
        positions[target] = finitePosition(held.u + offset.u, held.v + offset.v);
        stack.push(target);
      }
    }
  };
  if (seed !== undefined) visit(seed);
  for (const id of ids) if (!incoming.has(id)) visit(id);
  for (const id of ids) visit(id);
  return {positions};
}

function finitePosition(u: number, v: number): JackFlatPositionUv {
  if (!Number.isFinite(u) || !Number.isFinite(v)) throw new ValueError("invalidValue", "Jack flattened position must be finite");
  return {u, v};
}
