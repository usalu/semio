/** 🧪️ Three.js independently reproduces the same affine products and curve bounds. */
import { expect, test } from "bun:test";
import { Box2, CubicBezierCurve, Matrix3, Vector2 } from "three";
import fixture from "../../🧫️fixtures/📐️bounds/🔣️.json";
import { cubicBounds, multiply, type Matrix, type Point } from "../../🟦️.ts";

test("Drawing extrema match shared fixtures and Three.js curves", () => {
  for (const entry of fixture.curves) {
    const points = entry.points as [Point, Point, Point, Point];
    const own = cubicBounds(points);
    const curve = new CubicBezierCurve(...points.map(point => new Vector2(...point)) as [Vector2, Vector2, Vector2, Vector2]);
    const box = new Box2().setFromPoints(curve.getPoints(10000));
    const independent = [box.min.x, box.min.y, box.max.x-box.min.x, box.max.y-box.min.y];
    own.forEach((value, index) => { expect(value).toBeCloseTo(entry.bounds[index]!, 8); expect(value).toBeCloseTo(independent[index]!, 6); });
  }
});

test("Drawing parent composition matches shared fixtures and Three.js matrices", () => {
  const matrix = ([a,b,c,d,e,f]: number[]) => new Matrix3().set(a!,c!,e!,b!,d!,f!,0,0,1);
  for (const entry of fixture.matrices) {
    const own = multiply(entry.parent as Matrix, entry.child as Matrix);
    const oracle = matrix(entry.parent).multiply(matrix(entry.child)).elements;
    expect(own).toEqual(entry.expected);
    expect(own).toEqual([oracle[0],oracle[1],oracle[3],oracle[4],oracle[6],oracle[7]]);
  }
});


test("path splitting and inverse transforms match shared fixtures and Three.js", async () => {
  const { inverse, splitCubic } = await import("../../🟦️.ts");
  const fixture = await Bun.file(new URL("../../🧫️fixtures/✏️editing/🔣️.json", import.meta.url)).json();
  for (const item of fixture.splits) {
    const split = splitCubic(item.points, item.t);
    expect(split).toEqual(item.expected);
    if (!split) continue;
    const curve = (points: Point[]) => new CubicBezierCurve(...points.map(point => new Vector2(...point)) as [Vector2, Vector2, Vector2, Vector2]);
    const original = curve(item.points), left = curve(split[0]), right = curve(split[1]);
    for (const t of [0,0.2,0.5,0.8,1]) { expect(left.getPoint(t).distanceTo(original.getPoint(t*item.t))).toBeLessThan(1e-10); expect(right.getPoint(t).distanceTo(original.getPoint(item.t+t*(1-item.t)))).toBeLessThan(1e-10); }
  }
  for (const item of fixture.inverses) {
    const own = inverse(item.matrix);
    if (item.expected === null) expect(own).toBeNull();
    else { expect(own).not.toBeNull(); own!.forEach((value,index) => expect(value).toBeCloseTo(item.expected[index],12)); }
    const [a,b,c,d,e,f] = item.matrix;
    const matrix = new Matrix3().set(a,c,e,b,d,f,0,0,1);
    if (!own) { expect(matrix.determinant()).toBe(0); continue; }
    const elements = matrix.invert().elements;
    [elements[0],elements[1],elements[3],elements[4],elements[6],elements[7]].forEach((value,index) => expect(own[index]).toBeCloseTo(value!,12));
  }
});
