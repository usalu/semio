import { expect, test } from "bun:test";
import { Triangle, Vector3 } from "three";
import fixture from "./🔣️import-rollback.json";

test("Three.js independently verifies the first imported triangle", () => {
  const points = fixture.points.map(point => new Vector3(...point as [number, number, number]));
  const triangles = fixture.triangles.map(([a, b, c]) => new Triangle(points[a], points[b], points[c]));
  expect(triangles[0].getArea()).toBe(fixture.firstTriangleArea);
  expect(triangles[1].getArea()).toBe(fixture.firstTriangleArea);
  expect(triangles[0].getNormal(new Vector3()).dot(triangles[1].getNormal(new Vector3()))).toBe(1);
  const edgeUses = new Map<string, number>();
  for (const [a, b, c] of fixture.triangles) {
    for (const [from, to] of [[a, b], [b, c], [c, a]]) {
      const key = [Math.min(from, to), Math.max(from, to)].join(",");
      edgeUses.set(key, (edgeUses.get(key) ?? 0) + 1);
    }
  }
  expect(triangles).toHaveLength(fixture.openShell.faces);
  expect(edgeUses.size).toBe(fixture.openShell.edges);
  expect([...edgeUses.values()].filter(uses => uses === 1)).toHaveLength(fixture.openShell.boundaryEdges);
});

test("the late mesh refusal has an otherwise valid second triangle", () => {
  const points = fixture.points.map(point => new Vector3(...point as [number, number, number]));
  const first = fixture.triangles[0];
  const second = fixture.lateRefusal.triangle;
  const [from, to] = fixture.lateRefusal.sharedEdge;
  for (const [a, b, c] of [first, second]) {
    expect(new Triangle(points[a], points[b], points[c]).getArea()).toBe(fixture.firstTriangleArea);
    expect([[a, b], [b, c], [c, a]]).toContainEqual([from, to]);
  }
});
