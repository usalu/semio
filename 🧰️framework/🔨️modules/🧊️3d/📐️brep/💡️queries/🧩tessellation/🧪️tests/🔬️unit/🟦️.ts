import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { BoxGeometry, Line3, Triangle, Vector3 } from "three";

/** 🎟️ Independent geometry oracle for the original retained tessellation laws. */
test("original tessellation grants and unit-box parity agree with Three", () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️ownership/🔣️.json",import.meta.url),"utf8"));
  const geometry=new BoxGeometry(1,1,1);const position=geometry.getAttribute("position");const index=geometry.getIndex()!;
  let area=0,volume=0;const corners:[Vector3,Vector3,Vector3]=[new Vector3(),new Vector3(),new Vector3()];
  for(let cursor=0;cursor<index.count;cursor+=3){corners.forEach((point,offset)=>point.fromBufferAttribute(position,index.getX(cursor+offset)));area+=new Triangle(...corners).getArea();volume+=corners[0].dot(corners[1].clone().cross(corners[2]))/6;}
  expect(area).toBe(fixture.box.area);expect(Math.abs(volume)).toBeCloseTo(fixture.box.volume,12);expect(index.count/3).toBe(fixture.box.triangles);geometry.dispose();
  expect(Math.abs(volume)*2).toBe(fixture.nativeFamily.compoundVolume);
  const wire=fixture.nativeFamily.wire;const points=[new Vector3(),new Vector3(wire.width,0,0),new Vector3(wire.width,wire.height,0),new Vector3(0,wire.height,0)];
  expect(points.reduce((length,start,cursor)=>length+new Line3(start,points[(cursor+1)%points.length]).distance(),0)).toBe(wire.length);
  const rows=new Map<string,Uint8Array>(fixture.liveRows.keys.map((key:string)=>[key,new Uint8Array(fixture.liveRows.payloadBytes)]));
  const original=new Map(rows);const removed=fixture.liveRows.removedKeys.map((key:string)=>{const value=rows.get(key)!;rows.delete(key);expect(value).toBe(original.get(key));return key;});
  expect(removed).toEqual(fixture.liveRows.removedKeys);expect(rows.size+removed.length).toBe(fixture.liveRows.keys.length);
  console.log("[DEBUG] Original tessellation neutral ownership fixture: independent Three area=6 volume=1 triangles=12");
});
