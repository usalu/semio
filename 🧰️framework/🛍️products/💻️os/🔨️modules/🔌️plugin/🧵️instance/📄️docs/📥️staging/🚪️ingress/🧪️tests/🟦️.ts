/** 🪢️ Head-reachability corpus `head-reachability.json` validated with the independent third-party `graphlib` (`alg.preorder`) instead of our own fixpoint walk. */
import { expect, test } from "bun:test";
import { alg, Graph } from "graphlib";
import corpus from "./head-reachability.json";

test("roster survivors are the members reachable from the root through head-named edges", () => {
  for (const item of corpus.cases) {
    const graph = new Graph();
    graph.setNode(item.root);
    const named = new Set(item.head.map((row) => `${row.slot}\u0000${row.childId}\u0000${row.artifactId}`));
    for (const member of item.members) {
      const admitted = member.parent !== item.root || named.has(`${member.slot}\u0000${member.childId}\u0000${member.artifactId}`);
      if (!admitted) continue;
      graph.setNode(member.artifactId);
      graph.setEdge(member.parent, member.artifactId);
    }
    const reached = new Set(alg.preorder(graph, [item.root]));
    const kept = item.members.map((member) => member.artifactId).filter((id) => reached.has(id));
    expect(kept, item.name).toEqual(item.survivors);
  }
});
