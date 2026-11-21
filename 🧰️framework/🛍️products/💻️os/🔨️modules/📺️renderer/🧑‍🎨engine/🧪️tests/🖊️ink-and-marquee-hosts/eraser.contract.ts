import assert from "node:assert/strict";
import { Ray } from "three/src/math/Ray.js";
import { Sphere } from "three/src/math/Sphere.js";
import { Vector3 } from "three/src/math/Vector3.js";
import { applyEventsLocal } from "../../🧱️elements/🖋️InkCanvasHost/🌳️ink-document-ops/🟦️.ts";
import { createInkHostId, eraseInkStrokePointEventsNearPoint, eraseInkStrokePointsInItem } from "../../🧱️elements/🖋️InkCanvasHost/📐️ink-geometry/🟦️.ts";
import type { InkDocument, InkItem, InkStrokeItem, Vec2 } from "../../🧱️elements/🖋️InkCanvasHost/🧾️ink-model/🟦️.ts";
import cases from "./eraser.cases.json";

const vector = (point: readonly number[]) => new Vector3(point[0], point[1], 0);
const equal = (a: Vec2, b: Vec2) => Math.abs(a[0] - b[0]) < 1e-9 && Math.abs(a[1] - b[1]) < 1e-9;

function threeFragments(points: readonly number[][], sphere: Sphere): Vec2[][] {
  const vertices = points.map(vector);
  if (vertices.every((point) => point.equals(vertices[0]!))) return sphere.containsPoint(vertices[0]!) ? [] : [vertices.map(({ x, y }) => [x, y])];
  const runs: Vec2[][] = [];
  let run: Vec2[] | undefined;
  for (let index = 1; index < vertices.length; index++) {
    const start = vertices[index - 1]!;
    const end = vertices[index]!;
    if (start.equals(end)) continue;
    const direction = end.clone().sub(start).normalize();
    const entered = new Ray(start, direction).intersectSphere(sphere, new Vector3());
    const exited = new Ray(end, direction.clone().negate()).intersectSphere(sphere, new Vector3());
    const cuts = [start, ...[entered, exited].filter((point): point is Vector3 =>
      point !== null && point.clone().sub(start).dot(direction) > 1e-9 && point.clone().sub(end).dot(direction) < -1e-9
    ), end].sort((a, b) => start.distanceToSquared(a) - start.distanceToSquared(b));
    for (let cut = 1; cut < cuts.length; cut++) {
      const a = cuts[cut - 1]!;
      const b = cuts[cut]!;
      if (a.distanceTo(b) < 1e-9) continue;
      if (a.clone().lerp(b, 0.5).distanceTo(sphere.center) < sphere.radius - 1e-9) {
        run = undefined;
      } else {
        if (!run) {
          run = [[a.x, a.y]];
          runs.push(run);
        }
        run.push([b.x, b.y]);
      }
    }
  }
  return runs.map((run) => run.filter((point) => sphere.center.distanceTo(vector(point)) > sphere.radius + 1e-9 || point === run[0] || point === run.at(-1)
    || points.some((sample) => equal(point, [sample[0]!, sample[1]!]))));
}

function assertFragments(actual: readonly (readonly Vec2[])[], expected: readonly number[][][], message: string): void {
  assert.deepEqual(actual.map((points) => points.length), expected.map((points) => points.length), message);
  actual.forEach((points, fragment) => points.forEach((point, index) => {
    const target = expected[fragment]![index]!;
    assert.ok(equal(point, [target[0]!, target[1]!]), message);
  }));
}

const stroke = (id: string, points: readonly Vec2[], x = 0, y = 0): InkStrokeItem => ({
  kind: "stroke", id, name: "Ink", x, y, width: 80, height: 1, visible: true, locked: false,
  points, strokeWidth: 2, color: [0, 0, 0, 1],
});

export function verifyEraserClipping(): void {
  for (const entry of cases) {
    const [x, y] = entry.origin as [number, number];
    const [cx, cy] = entry.center as [number, number];
    const original = stroke("ink", entry.points.map(([px, py]) => [px!, py!]), x, y);
    const erased = eraseInkStrokePointsInItem(original, cx, cy, entry.radius);
    assertFragments(threeFragments(entry.points, new Sphere(new Vector3(cx - x, cy - y, 0), entry.radius)), entry.fragments, entry.title + " (Three.js)");
    assertFragments(erased.map((item) => item.points), entry.fragments, entry.title);
    if (entry.unchanged) assert.equal(erased[0], original, entry.title);
    if (erased[0]) {
      assert.equal(erased[0].id, original.id);
      assert.equal(erased[0].name, original.name);
    }
    assert.equal(new Set(erased.map(({ id }) => id)).size, erased.length);
    for (const item of erased) assert.deepEqual(eraseInkStrokePointsInItem(item, cx, cy, entry.radius), [item], entry.title);
  }
  const sourceId = `stroke-host-${Number(createInkHostId("stroke").split("-").at(-1)) + 1}`;
  const fragments = eraseInkStrokePointsInItem(stroke(sourceId, [[0, 0], [80, 0]]), 40, 0, 5);
  assert.equal(fragments[0]?.id, sourceId);
  assert.notEqual(fragments[1]?.id, sourceId);
}

export function verifyEraserEventPlacement(): void {
  const group = (id: string, children: readonly InkItem[]): InkItem => ({
    kind: "group", id, name: id, x: 0, y: 0, width: 120, height: 50, visible: true, locked: false, children,
  });
  const occupiedId = `stroke-host-${Number(createInkHostId("stroke").split("-").at(-1)) + 1}`;
  const sibling = stroke(occupiedId, [[0, 30], [80, 30]]);
  const blocks = [stroke("root", [[0, 0], [80, 0]]), group("outer", [
    group("inner", [stroke("erase", [[40, 0]]), stroke("nested", [[0, 0], [80, 0]]), sibling]),
  ])];
  const doc: InkDocument = { schema: "ink.document", id: "test", camera: { x: 0, y: 0, zoom: 1 }, blocks };
  const events = eraseInkStrokePointEventsNearPoint(doc, 40, 0, 5);
  assert.deepEqual(events.map((event) => {
    switch (event.operation) {
      case "updateBlock": return [event.operation, event.blockId, event.block.id];
      case "removeBlock": return [event.operation, event.blockId];
      case "addBlock": return [event.operation, event.parentId ?? null, event.index];
      default: return [event.operation];
    }
  }), [
    ["updateBlock", "root", "root"],
    ["addBlock", null, 1],
    ["removeBlock", "erase"],
    ["updateBlock", "nested", "nested"],
    ["addBlock", "inner", 1],
  ]);
  const result = applyEventsLocal(doc, events);
  const rootFragment = events[1];
  const nestedFragment = events[4];
  assert.ok(rootFragment?.operation === "addBlock" && nestedFragment?.operation === "addBlock");
  assert.notEqual(rootFragment.block.id, occupiedId);
  const outer = result.blocks[2];
  assert.ok(outer?.kind === "group");
  const inner = outer.children[0];
  assert.ok(inner?.kind === "group");
  assert.deepEqual(result.blocks.map(({ id }) => id), ["root", rootFragment.block.id, "outer"]);
  assert.deepEqual(inner.children.map(({ id }) => id), ["nested", nestedFragment.block.id, occupiedId]);
  assert.deepEqual(eraseInkStrokePointEventsNearPoint(result, 40, 0, 5), []);
}
