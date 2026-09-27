/** 🧪️ The interaction manifest must enumerate the document before any layer is picked. */
import { expect, test } from "bun:test";

test("Select All resolves document topology without a prior pick", async () => {
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("hierarchy: HierarchyProvider::Topology")).toBe(true);
  expect(source.includes("fn interaction_topology(")).toBe(true);
});

import { Object3D } from "three";
import { drawingInteractionTopology, type DrawingInteractionLayer } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";

for (const item of fixture) test(`document topology: ${item.name}`, () => {
  const actual = drawingInteractionTopology(item.layers);
  expect(actual.ordered).toEqual(item.ordered);
  const root = new Object3D();
  const build = (layers: readonly DrawingInteractionLayer[], parent: Object3D): void => {
    for (const layer of layers) {
      const node = new Object3D(); node.name = layer.base.id; parent.add(node);
      if (layer.kind === "group") build(layer.children as readonly DrawingInteractionLayer[] ?? [], node);
    }
  };
  build(item.layers, root);
  const oracle: { id: string; granularity: string; parent?: string }[] = [];
  root.traverse(node => {
    if (node === root) return;
    oracle.push({ id: node.name, granularity: "stroke", ...(node.parent === root ? {} : { parent: node.parent!.name }) });
  });
  expect(actual.ordered).toEqual(oracle);
});
