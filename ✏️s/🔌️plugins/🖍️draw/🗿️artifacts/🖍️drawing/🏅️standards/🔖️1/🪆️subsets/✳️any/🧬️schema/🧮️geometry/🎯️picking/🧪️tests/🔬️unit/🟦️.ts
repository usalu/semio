/** 🎯️ Picking follows path contours, with bounded incremental curve work. */
import {expect,test} from "bun:test";
import {ShapeUtils,Vector2,Line3,Vector3,CubicBezierCurve,Matrix3} from "three";
import {PathHitCursor} from "../../🟦️.ts";
import type {PathSegment} from "../../../../🟦️.ts";
import type {Matrix,Point} from "../../../🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";
for(const sample of cases)test(`painted path hit: ${sample.name}`,()=>{
  const cursor=new PathHitCursor(sample.point as Point,sample.matrix as Matrix,sample.radius,.001);
  let steps=0;while(!cursor.step(sample.segments as PathSegment[])){expect(++steps).toBeLessThan(100000);}
  expect(cursor.contains(sample.fill,sample.stroke)).toBe(sample.expected);
  expect(cursor.maximumDepth).toBeLessThanOrEqual(33);
  if(sample.name.startsWith("concave")){
    const [a,b,c,d,e,f]=sample.matrix,matrix=new Matrix3().set(a!,c!,e!,b!,d!,f!,0,0,1);
    const contour=sample.segments.filter(s=>s.kind!=="close").map(s=>new Vector2(...s.to!).applyMatrix3(matrix)),p=new Vector2(...sample.point);
    const cross=(a:Vector2,b:Vector2,c:Vector2)=>(b.x-a.x)*(c.y-a.y)-(b.y-a.y)*(c.x-a.x);
    const inside=ShapeUtils.triangulateShape(contour,[]).some(face=>{const [a,b,c]=face.map(i=>contour[i]!);const signs=[cross(a!,b!,p),cross(b!,c!,p),cross(c!,a!,p)];return signs.every(v=>v>=0)||signs.every(v=>v<=0);});
    expect(inside).toBe(sample.expected);
  }
  if(sample.name==="zero-tolerance-diagonal"){
    const line=new Line3(new Vector3(0,0,0),new Vector3(10,10,0)),p=new Vector3(...sample.point,0);
    expect(line.closestPointToPoint(p,true,new Vector3()).distanceTo(p)).toBe(0);
  }
  if(sample.name==="stroke-near-edge"){
    const line=new Line3(new Vector3(0,0,0),new Vector3(0,100,0)),p=new Vector3(...sample.point,0);
    expect(line.closestPointToPoint(p,true,new Vector3()).distanceTo(p)<=sample.radius).toBe(sample.expected);
  }
  if(sample.name==="cubic-outside-bulge"){
    const curve=new CubicBezierCurve(new Vector2(0,0),new Vector2(0,100),new Vector2(100,100),new Vector2(100,0));
    expect(Math.min(...curve.getPoints(1000).map(p=>p.distanceTo(new Vector2(...sample.point))))).toBeGreaterThan(sample.radius);
  }
});

test("curve picking yields with bounded storage and refuses unresolved geometry",()=>{
  const segments:PathSegment[]=[{kind:"move",to:[0,0]},{kind:"cubic",ctrl1:[0,1e100],ctrl2:[1e100,1e100],to:[1e100,0]}];
  const cursor=new PathHitCursor([0,0],[1,0,0,1,0,0],1,.001);
  for(let index=0;index<16;index++)expect(cursor.step(segments)).toBe(false);
  let steps=16;while(!cursor.step(segments))expect(++steps).toBeLessThan(100);
  expect(cursor.failed()).toBe(true);
  expect(cursor.contains(true,true)).toBe(false);
  expect(cursor.maximumDepth).toBeLessThanOrEqual(33);
});
