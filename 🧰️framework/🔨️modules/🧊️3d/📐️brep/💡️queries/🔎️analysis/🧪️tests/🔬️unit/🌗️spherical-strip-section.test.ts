/** 🌗️ Checks spherical Section fixture areas against independently clipped Three.js ellipses. */
import {expect,test} from "bun:test";
import {EllipseCurve,ShapeUtils,Vector2,Vector3} from "three";
import fixture from "../🧫️fixtures/spherical-strip-section.json";

function clip(ring:Vector2[],height:(point:Vector2)=>number,limit:number,sign:number) {
  return ring.flatMap((a,index)=>{
    const b=ring[(index+1)%ring.length];
    const first=sign*(height(a)-limit);
    const last=sign*(height(b)-limit);
    const retained=first>=0?[a]:[];
    if ((first>=0)!==(last>=0)) retained.push(a.clone().lerp(b,first/(first-last)));
    return retained;
  });
}

for(const row of fixture.cases) test(row.name,()=>{
  for(const sign of [-1,1]) {
    const normal=new Vector3(...row.normal as [number,number,number]).multiplyScalar(sign);
    const origin=new Vector3(...row.plane as [number,number,number]);
    const planeHeight=normal.dot(origin);
    const center=normal.clone().multiplyScalar(planeHeight);
    const circleRadius=Math.sqrt(fixture.sphereRadius**2-planeHeight**2);
    const x=new Vector3(0,0,1).addScaledVector(normal,-normal.z);
    if(x.lengthSq()===0) x.set(1,0,0);
    x.normalize();
    const y=new Vector3().crossVectors(normal,x);
    const height=(point:Vector2)=>center.z+x.z*point.x+y.z*point.y;
    const samples=new EllipseCurve(0,0,circleRadius,circleRadius).getPoints(32768).slice(0,-1);
    const ring=clip(clip(samples,height,row.axial[0],1),height,row.axial[1],-1);
    expect(circleRadius).toBeCloseTo(row.radius,12);
    expect(Math.abs(ShapeUtils.area(ring))).toBeCloseTo(row.area,6);
  }
});
