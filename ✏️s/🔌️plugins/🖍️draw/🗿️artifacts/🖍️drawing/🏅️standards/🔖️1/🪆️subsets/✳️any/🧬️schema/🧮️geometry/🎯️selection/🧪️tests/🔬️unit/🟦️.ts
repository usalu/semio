/** 🧪️ Polygon selection fixtures checked against Three.js triangulation. */
import { expect,test } from "bun:test";
import { ShapeUtils,Triangle,Vector2,Vector3 } from "three";
import { polygonContainsPoint,polygonEnclosesBounds } from "../../🟦️.ts";
import type { Point } from "../../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";
test("lasso containment follows the polygon, including concave boundaries",()=>{
  for(const item of fixture) {
    const polygon=item.polygon as Point[];
    const points=polygon.map(point=>new Vector2(point[0],point[1]));
    const triangles=ShapeUtils.triangulateShape(points,[]).map(face=>new Triangle(...face.map(index=>new Vector3(points[index]!.x,points[index]!.y,0)) as [Vector3,Vector3,Vector3]));
    for(const query of item.points) {
      const actual=polygonContainsPoint(polygon,query.point as Point);
      expect(actual).toBe(query.inside);
      expect(actual).toBe(triangles.some(triangle=>triangle.containsPoint(new Vector3(query.point[0],query.point[1],0))));
    }
    for(const query of item.boxes) expect(polygonEnclosesBounds(polygon,query.bounds as [number,number,number,number])).toBe(query.inside);
  }
});
