/** 🧊️ Proves neutral icon transform vectors against Three's independent world-matrix projection. */
import {test,expect} from "bun:test";
import * as THREE from "three";
import fixture from "../🧫️fixtures/🖼️icon-transform/🔣️.json";
import {applyIconRenderTransform} from "../../🎯️targets/⚛️react/🔌️host-ports/🧊️transform/🟦️.ts";
import type {IconRenderTransform} from "@semio-tech/ui-styling";
for(const sample of fixture.cases)test(sample.id,()=>{
 const root=new THREE.Group();
 applyIconRenderTransform(root,sample.transform as unknown as IconRenderTransform);
 const bounds=new THREE.Box3(new THREE.Vector3(...sample.localBounds.minimum as [number,number,number]),new THREE.Vector3(...sample.localBounds.maximum as [number,number,number])).applyMatrix4(root.matrixWorld);
 for(const [actual,expected] of [[bounds.min.toArray(),sample.worldBounds.minimum],[bounds.max.toArray(),sample.worldBounds.maximum]] as const)actual.forEach((value,axis)=>expect(Math.abs(value-expected[axis]!)).toBeLessThan(fixture.tolerance));
 for(let index=0;index<sample.localPoints.length;index++){
  const point=sample.localPoints[index]!,expected=sample.worldPoints[index]!;
  const world=root.localToWorld(new THREE.Vector3(point[0],point[1],point[2]));
  world.toArray().forEach((value,axis)=>expect(Math.abs(value-expected[axis]!)).toBeLessThan(fixture.tolerance));
 }
});
test("missing transform preserves an already framed GLB root",()=>{
 const root=new THREE.Group();root.position.set(1,2,3);
 applyIconRenderTransform(root,undefined);
 expect(root.position.toArray()).toEqual([1,2,3]);
});
