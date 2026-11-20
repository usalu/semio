import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { BoxGeometry, EllipseCurve, Line3, Matrix4, Path, PlaneGeometry, Ray, Shape, ShapeGeometry, Triangle, Vector2, Vector3 } from "three";

/** 🎟️ Independent geometry oracle for the original retained tessellation laws. */
test("original tessellation grants and unit-box parity agree with Three", () => {
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎟️ownership/🔣️.json",import.meta.url),"utf8"));
  const geometry=new BoxGeometry(1,1,1);const position=geometry.getAttribute("position");const index=geometry.getIndex()!;
  let area=0,volume=0;const corners:[Vector3,Vector3,Vector3]=[new Vector3(),new Vector3(),new Vector3()];
  for(let cursor=0;cursor<index.count;cursor+=3){corners.forEach((point,offset)=>point.fromBufferAttribute(position,index.getX(cursor+offset)));area+=new Triangle(...corners).getArea();volume+=corners[0].dot(corners[1].clone().cross(corners[2]))/6;}
  expect(area).toBe(fixture.box.area);expect(Math.abs(volume)).toBeCloseTo(fixture.box.volume,12);expect(index.count/3).toBe(fixture.box.triangles);geometry.dispose();
  expect(Math.abs(volume)*2).toBe(fixture.nativeFamily.compoundVolume);
  const wire=fixture.nativeFamily.wire;const points=[new Vector3(),new Vector3(wire.width,0,0),new Vector3(wire.width,wire.height,0),new Vector3(0,wire.height,0)];
  expect(points.reduce((length,start,cursor)=>length+new Line3(start,points[(cursor+1)%points.length]).distance(),0)).toBe(wire.length);
  const rows=new Map<string,Uint8Array>(fixture.liveRows.keys.map((key:string)=>[key,new Uint8Array(fixture.liveRows.payloadBytes)]));
  const original=new Map(rows);const removed=fixture.liveRows.removedKeys.map((key:string)=>{const value=rows.get(key)!;rows.delete(key);const expectedAt16x157 = original.get(key); expect<typeof expectedAt16x157>(value).toBe(expectedAt16x157);return key;});
  expect(removed).toEqual(fixture.liveRows.removedKeys);expect(rows.size+removed.length).toBe(fixture.liveRows.keys.length);
  const triangle=fixture.meshImport.positions;expect(new Triangle(...[0,3,6].map(offset=>new Vector3(...triangle.slice(offset,offset+3))) as [Vector3,Vector3,Vector3]).getArea()).toBe(fixture.meshImport.area);
  const modeling=JSON.parse(readFileSync(new URL("../../../../../🥽️mesh/🧪️tests/🧫️fixtures/🛠️modeling/🔣️.json",import.meta.url),"utf8"));
expect(area).toBe(modeling.ownedCapture.area);expect(Math.abs(volume)).toBeCloseTo(modeling.ownedCapture.volume,12);
});

test("the analytic disk area agrees with a Three.js ellipse triangulation", () => {
  const fixtures=JSON.parse(readFileSync(new URL("../🧫️fixtures/curve-preview.json",import.meta.url),"utf8"));
  for (const row of fixtures) {
    const points = new EllipseCurve(0,0,row.major,row.minor,0,2*Math.PI).getPoints(512);
    const mesh = new ShapeGeometry(new Shape(points));
    const index = mesh.getIndex()!;
    const positions = mesh.getAttribute("position");
    let area=0;
    const tri=new Triangle();
    for (let i=0; i<index.count; i+=3) {
      tri.a.fromBufferAttribute(positions,index.getX(i));
      tri.b.fromBufferAttribute(positions,index.getX(i+1));
      tri.c.fromBufferAttribute(positions,index.getX(i+2));
      area+=tri.getArea();
    }
    expect(Math.abs(area/row.diskArea-1)).toBeLessThan(1e-4);
    mesh.dispose();
  }
});

test("a Three.js triangulation preserves planar concave area and boundary length", () => {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/planar-preview.json", import.meta.url), "utf8"));
  for (const row of fixture.cases) {
    const ring = row.ring.map(([x, y]: [number, number]) => new Vector2(x, y));
    const mesh = new ShapeGeometry(new Shape(ring));
    const positions = mesh.getAttribute("position");
    const indices = mesh.getIndex()!;
    const triangle = new Triangle();
    let area = 0;
    for (let i = 0; i < indices.count; i += 3) {
      triangle.a.fromBufferAttribute(positions, indices.getX(i));
      triangle.b.fromBufferAttribute(positions, indices.getX(i + 1));
      triangle.c.fromBufferAttribute(positions, indices.getX(i + 2));
      area += triangle.getArea();
    }
    expect(area).toBeCloseTo(row.area, 8);
    expect(indices.count / 3).toBe(row.triangles);
    expect(ring.reduce((sum: number, point: Vector2, i: number) => sum + point.distanceTo(ring[(i + 1) % ring.length]), 0)).toBeCloseTo(row.perimeter, 8);
    mesh.dispose();
  }
});

test("Three.js triangulates a planar face with two disjoint inner rings", () => {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/planar-preview.json", import.meta.url), "utf8")).holes;
  const vectors = (ring: Array<[number, number]>) => ring.map(([x,y]) => new Vector2(x,y));
  const shape = new Shape(vectors(fixture.outer));
  for (const ring of fixture.rings) shape.holes.push(new Path(vectors(ring)));
  const mesh = new ShapeGeometry(shape);
  const positions = mesh.getAttribute("position");
  const indices = mesh.getIndex()!;
  const t = new Triangle();
  let total = 0;
  for (let i = 0; i < indices.count; i += 3) {
    t.a.fromBufferAttribute(positions, indices.getX(i));
    t.b.fromBufferAttribute(positions, indices.getX(i+1));
    t.c.fromBufferAttribute(positions, indices.getX(i+2));
    total += t.getArea();
  }
  expect(total).toBeCloseTo(fixture.area, 10);
  expect(indices.count / 3).toBe(fixture.triangles);
  mesh.dispose();
});

test("the original vertex coordinate fixture agrees with Three.js", () => {
  const { point, length } = JSON.parse(readFileSync(new URL("../🧫️fixtures/planar-preview.json", import.meta.url), "utf8")).vertex;
  expect(new Vector3(point[0], point[1], point[2]).length()).toBeCloseTo(length, 14);
});

test("Three.js curved boundaries agree with the planar circular-hole fixture", () => {
  const row = JSON.parse(readFileSync(new URL("../🧫️fixtures/planar-preview.json", import.meta.url), "utf8")).circularHole;
  const outer = new Shape(row.outer.map(([x,y]: [number,number]) => new Vector2(x,y)));
  const [x,y] = row.center;
  const circle = new EllipseCurve(x,y,row.radius,row.radius,0,2*Math.PI);
  outer.holes.push(new Path(circle.getPoints(256)));
  const mesh = new ShapeGeometry(outer);
  const points = mesh.getAttribute("position");
  const indices = mesh.getIndex()!;
  const triangle = new Triangle();
  let area = 0;
  for (let i=0;i<indices.count;i+=3) {
    triangle.a.fromBufferAttribute(points,indices.getX(i));
    triangle.b.fromBufferAttribute(points,indices.getX(i+1));
    triangle.c.fromBufferAttribute(points,indices.getX(i+2));
    area += triangle.getArea();
  }
  expect(area).toBeCloseTo(row.area, 3);
  mesh.dispose();
});

test("analytic preview pick domains agree with Three curve and surface features",()=>{
  const f=JSON.parse(readFileSync(new URL("../🧫️fixtures/analytic-picks.json",import.meta.url),"utf8"));
  const circle=new EllipseCurve(0,0,f.curve.radius,f.curve.radius,0,Math.PI*2,false,0);
  const samples=circle.getPoints(32);
  const segments=samples.slice(1).map((p,i)=>samples[i].clone().lerp(p,0.5));
  expect(segments.every(p=>f.curve.radius-p.length()<f.curve.deflection)).toBe(true);
  expect(f.curve.domain).toBe("edge");
  expect(BigInt(f.curve.label).toString()).toBe(f.curve.label);
  const plane=new PlaneGeometry(f.plane.width,f.plane.width);
  const points=plane.getAttribute("position"),indices=plane.getIndex()!;
  const ray=new Ray(new Vector3(0.25,0.5,1),new Vector3(0,0,-1));
  let hits=0;
  for(let i=0;i<indices.count;i+=3){
    const [a,b,c]=[0,1,2].map(j=>new Vector3().fromBufferAttribute(points,indices.getX(i+j)));
    if(ray.intersectTriangle(a,b,c,true,new Vector3()))hits++;
  }
  expect(hits).toBe(1);
  expect(indices.count/3).toBe(f.plane.triangles);
  expect(f.plane.domain).toBe("face");
  expect(BigInt(f.plane.label).toString()).toBe(f.plane.label);
  const [x,y,z]=f.reflectedSphere.scale;
  const reflection=new Matrix4().makeScale(x,y,z);
  const normal=new Vector3(1,0,0).transformDirection(reflection).cross(new Vector3(0,1,0).transformDirection(reflection));
  expect(normal.toArray().map(n=>n||0)).toEqual(f.reflectedSphere.normal);
  expect(new Vector3(0,0,f.reflectedSphere.radius).applyMatrix4(reflection).toArray()).toEqual(f.reflectedSphere.pole);
});
