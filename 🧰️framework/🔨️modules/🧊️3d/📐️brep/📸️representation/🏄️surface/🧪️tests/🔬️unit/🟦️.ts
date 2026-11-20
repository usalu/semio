import { expect, test } from "bun:test";
import { CylinderGeometry, Triangle, Vector3, Vector4 } from "three";
import { NURBSSurface } from "three/addons/curves/NURBSSurface.js";
import fixture from "../🧫️fixtures/rational-patch.json";

test("Three.js agrees on the rational patch points and tangent-plane normals", () => {
  const net=fixture.controls.map((row,i)=>row.map(([x,y,z],j)=>new Vector4(x,y,z,fixture.weights[i][j])));
  const patch=new NURBSSurface(fixture.uDegree,fixture.vDegree,fixture.uKnots,fixture.vKnots,net);
  const point=(u:number,v:number)=>{
    const result=new Vector3();
    patch.getPoint(u,v,result);
    return result;
  };
  const h=1e-6;
  for(const {uv:[u,v],point:expectedPoint,normal:expectedNormal} of fixture.samples) {
    expect(point(u,v).distanceTo(new Vector3(...expectedPoint))).toBeLessThan(1e-12);
    const du=point(Math.min(1,u+h),v).sub(point(Math.max(0,u-h),v));
    const dv=point(u,Math.min(1,v+h)).sub(point(u,Math.max(0,v-h)));
    expect(du.cross(dv).normalize().distanceTo(new Vector3(...expectedNormal))).toBeLessThan(2e-6);
  }
});

test("Three.js rational surface samples agree with a triangulated quarter cylinder", () => {
  const net = fixture.controls.map((row,i) => row.map(([x,y,z],j) => new Vector4(x,y,z,fixture.weights[i][j])));
  const patch = new NURBSSurface(fixture.uDegree,fixture.vDegree,fixture.uKnots,fixture.vKnots,net);
  const point = (u: number, v: number) => {
    const result = new Vector3();
    patch.getPoint(u, v, result);
    return result;
  };
  const n = fixture.oracleSubdivisions;
  for (let i = 0; i < n; i++) {
    const a = point(i/n, 0);
    const b = point((i+1)/n, 0);
    const d = point(i/n, 1);
    const c = point((i+1)/n, 1);
    const center = (points: Vector3[]) => points.reduce((value, p) => value.add(p), new Vector3()).divideScalar(3);
    expect(point((i+2/3)/n, 1/3).distanceTo(center([a,b,c]))).toBeLessThan(fixture.previewDeflection);
    expect(point((i+1/3)/n, 2/3).distanceTo(center([a,c,d]))).toBeLessThan(fixture.previewDeflection);
  }
});

test("Three.js preserves the distinct sides of a full-multiplicity knot", () => {
  const { uKnots, sections, seam } = fixture.discontinuous;
  const controls = sections.flatMap(section => section.map(x => [new Vector4(x,0,0,1),new Vector4(x,1,0,1)]));
  const patch = new NURBSSurface(1,1,uKnots,fixture.vKnots,controls);
  const left = new Vector3();
  const right = new Vector3();
  patch.getPoint(seam - 1e-12, 0.5, left);
  patch.getPoint(seam, 0.5, right);
  expect(left.x).toBeCloseTo(sections[0][1], 10);
  expect(right.x).toBe(sections[1][0]);
  expect(right.distanceTo(left)).toBeCloseTo(sections[1][0] - sections[0][1], 10);
});

test("Three.js models the finite cylinder preview chart", () => {
  const row = fixture.analyticCylinder;
  const geometry = new CylinderGeometry(row.radius,row.radius,row.height,128,1,true);
  const points = geometry.getAttribute("position");
  const indices = geometry.getIndex()!;
  const triangle = new Triangle();
  let area = 0;
  for (let i=0;i<indices.count;i+=3) {
    triangle.a.fromBufferAttribute(points,indices.getX(i));
    triangle.b.fromBufferAttribute(points,indices.getX(i+1));
    triangle.c.fromBufferAttribute(points,indices.getX(i+2));
    area += triangle.getArea();
  }
  expect(Math.abs(area-row.area)).toBeLessThan(row.tolerance);
  geometry.dispose();
});
