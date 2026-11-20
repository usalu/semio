import { expect, test } from "bun:test";
import { Triangle, Vector3 } from "three";
import fixture from "./🔣️import-rollback.json";

test("Three.js independently verifies the first imported triangle", () => {
  const points = fixture.points.map(point => new Vector3(...point as [number, number, number]));
  const triangles = fixture.triangles.map(([a, b, c]) => new Triangle(points[a], points[b], points[c]));
  expect(triangles[0].getArea()).toBe(fixture.firstTriangleArea);
  expect(triangles[1].getArea()).toBe(fixture.firstTriangleArea);
  expect(triangles[0].getNormal(new Vector3()).dot(triangles[1].getNormal(new Vector3()))).toBe(1);
});
