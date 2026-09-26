/** 🎯️ Three.js independently validates the transformed selection fixture. */
import { expect, test } from "bun:test";
import { CubicBezierCurve, Vector2, Matrix3, Box2 } from "three";
import fixture from "../../🧫️fixtures/🎯️selection/🔣️.json";

test("nested curve selection agrees with independent transformed geometry", () => {
  const start = fixture.segments[0]!.to!;
  const segment = fixture.segments[1]!;
  const curve = new CubicBezierCurve(new Vector2(...start),new Vector2(...segment.ctrl1!),new Vector2(...segment.ctrl2!),new Vector2(...segment.to!));
  for (const item of fixture.cases) {
    const [x,y,sx,sy,rotation] = item.parent as [number,number,number,number,number];
    const matrix = new Matrix3().set(sx*Math.cos(rotation),-sy*Math.sin(rotation),x,sx*Math.sin(rotation),sy*Math.cos(rotation),y,0,0,1);
    const box = new Box2().setFromPoints(curve.getPoints(1000).map(point => point.applyMatrix3(matrix)));
    const controls = "includeControls" in item && item.includeControls === true && [segment.ctrl1!,segment.ctrl2!].some(point => new Vector2(point[0],point[1]).applyMatrix3(matrix).distanceTo(new Vector2(item.point[0],item.point[1])) < 1e-10);
    const selected = !("visible" in item && item.visible === false) && !("locked" in item && item.locked === true) && (controls || box.containsPoint(new Vector2(item.point[0],item.point[1])));
    expect(selected).toBe(item.selected);
  }
});
