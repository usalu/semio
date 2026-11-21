import { expect, test } from "bun:test";
import { Triangle, Vector3, Vector4 } from "three";
import { NURBSSurface } from "three/addons/curves/NURBSSurface.js";
import fixture from "../🧫️fixtures/rational-plane-nearest.json";

const point = (values: number[]) => new Vector3().fromArray(values);
test("Three.js agrees on finite rational plane interior, edge, and corner projections", () => {
  const [u, v] = fixture.domain;
  const control = fixture.controls.map((row, i) => row.map((p, j) => {
    return new Vector4(p[0], p[1], p[2], fixture.weights[i][j]);
  }));
  const patch = new NURBSSurface(1, 1, [u[0], u[0], u[1], u[1]], [v[0], v[0], v[1], v[1]], control);
  const [[a, d], [b, c]] = fixture.controls.map(row => row.map(point));
  const triangles = [new Triangle(a,b,c), new Triangle(a,c,d)];
  for (const row of fixture.queries) {
    const target = point(row.target);
    const expected = point(row.point);
    const parameters = row.uv.map((p, i) => {
      const [min, max] = fixture.domain[i];
      return (p - min) / (max - min);
    });
    const model = new Vector3();
    patch.getPoint(parameters[0], parameters[1], model);
    expect(model.distanceTo(expected)).toBeLessThan(1e-12);
    const projected = triangles.map(triangle => triangle.closestPointToPoint(target, new Vector3()))
      .reduce((a, b) => a.distanceToSquared(target) <= b.distanceToSquared(target) ? a : b);
    expect(projected.distanceTo(expected)).toBeLessThan(1e-12);
    expect(projected.distanceTo(target)).toBeCloseTo(row.distance, 12);
  }
});
