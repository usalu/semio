/** 🧪️ Verifies deterministic synthetic testing registries against independent graph topology. */
import { expect, test } from "vitest";
import { DirectedGraph } from "graphology";
import examples from "../../../../🧪️testing/⚖️scale/🧫️fixtures/📇️registry/🔣️.json";
import { renderScaleFixtureArtifacts } from "../../../../🧪️testing/⚖️scale/📽️projection/🟦️.ts";

for (const example of examples) test(example.name, () => {
  const first = renderScaleFixtureArtifacts(example.plugins, example.extensions, example.seed);
  const second = renderScaleFixtureArtifacts(example.plugins, example.extensions, example.seed);
  expect(first.registryJson).toBe(second.registryJson);
  expect(first.catalogJson).toBe(second.catalogJson);
  const graph = new DirectedGraph();
  for (const record of first.registry.records) graph.addNode(record.id);
  for (const record of first.registry.records) if (record.parentId) graph.addEdge(record.parentId, record.id);
  expect(graph.order).toBe(example.records);
  expect(graph.size).toBe(example.plugins * example.extensions);
  const catalog = JSON.parse(first.catalogJson) as { totalRecordCount: number; plugins: {pluginId: string; extensionIds: string[]}[] };
  expect(catalog.totalRecordCount).toBe(graph.order);
  for (const row of catalog.plugins) {
    expect(graph.inDegree(row.pluginId)).toBe(0);
    expect(graph.outNeighbors(row.pluginId)).toEqual(row.extensionIds);
    expect(graph.outDegree(row.pluginId)).toBe(example.extensions);
  }
});
