import { expect, test } from "bun:test";
import { Box3, Triangle, Vector3 } from "three";
import fixture from "./🔣️.json";

test("typed preview replacement carries the two independently measured LOD meshes", () => {
  expect(fixture.lods[1].tolerance).toBeLessThan(fixture.lods[0].tolerance);
  for (const lod of fixture.lods) {
    const vertices = lod.indices.map(index => new Vector3().fromArray(lod.positions, index * 3));
    expect(new Triangle(vertices[0], vertices[1], vertices[2]).getArea()).toBe(lod.area);
    expect(new Box3().setFromPoints(vertices).max.toArray()).toEqual(lod.maximum);
    expect(fixture.invalidIndices.some(index => index > 0xffffffff)).toBe(true);
  }
});
