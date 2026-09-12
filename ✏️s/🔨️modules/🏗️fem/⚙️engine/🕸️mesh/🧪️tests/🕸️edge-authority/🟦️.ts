import assert from "node:assert/strict";
import { createRequire } from "node:module";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🕸️edge-authority/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🕸️edge-authority/🔣️.json" with { type: "json" };

export function testMeshEdgeAuthority(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  const { Graph } = createRequire(import.meta.url)("graphlib");
  for (const row of corpus.cases) {
    const graph = new Graph({ directed: false });
    for (const [face, triangle] of row.triangles.entries()) {
      for (let local = 0; local < 3; local++) {
        const first = String(triangle[local]), second = String(triangle[(local + 1) % 3]);
        const adjacent = graph.edge(first, second) ?? [];
        graph.setEdge(first, second, [...adjacent, face]);
      }
    }
    const adjacencies = graph.edges().map((edge: { v: string; w: string }) => graph.edge(edge));
    assert.deepEqual({
      slots: row.triangles.flat().length,
      edges: graph.edgeCount(),
      boundary: adjacencies.filter((owners: number[]) => owners.length === 1).length,
      interior: adjacencies.filter((owners: number[]) => owners.length === 2).length,
    }, row.expected, row.id);
  }
  console.log("[DEBUG] Mesh edge authority slots and adjacency agree with three neutral Graphlib cases");
}
