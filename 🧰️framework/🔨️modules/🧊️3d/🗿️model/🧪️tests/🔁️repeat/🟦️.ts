/** 🔁️ Repeat: the k-th copy applies the step k times; copies are located by connected components of the welded three.js geometry. */
import { expect, it } from "vitest";
import { tessellate } from "../../🟦️.ts";
import { bounds, fixture, mergedGeometry, weldedGeometry } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("🔁️repeat");

it("places every rotation-step copy at its expected centre", async () => {
  const data = fixture("🔁️repeat");
  const c = data.cases.find((x: any) => x.name === "rotation-steps");
  const out = await tessellate(c.document, data.quality);
  const welded = weldedGeometry(mergedGeometry(out));
  const index = welded.getIndex()!, position = welded.getAttribute("position");
  const parent = [...Array(position.count).keys()];
  const find = (x: number): number => (parent[x] === x ? x : (parent[x] = find(parent[x]!)));
  for (let i = 0; i < index.count; i += 3) { parent[find(index.getX(i))] = find(index.getX(i + 1)); parent[find(index.getX(i))] = find(index.getX(i + 2)); }
  const boxes = new Map<number, { min: number[]; max: number[] }>();
  for (let v = 0; v < position.count; v++) {
    const b = boxes.get(find(v)) ?? { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
    boxes.set(find(v), b);
    for (let k = 0; k < 3; k++) { b.min[k] = Math.min(b.min[k]!, position.getComponent(v, k)); b.max[k] = Math.max(b.max[k]!, position.getComponent(v, k)); }
  }
  const centres = [...boxes.values()].map((b) => [0, 1, 2].map((k) => (b.min[k]! + b.max[k]!) / 2));
  expect(centres.length).toBe(6);
  for (const expected of c.expected.copyCenters as number[][]) expect(centres.some((got) => got.every((g, k) => Math.abs(g - expected[k]!) < 1e-6))).toBe(true);
  expect(bounds(welded).max[0]).toBeGreaterThan(1);
});
