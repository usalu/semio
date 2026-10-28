/** ⭕️ Original chord and output grants govern circles after represented world stretch. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {Matrix4,Vector3} from 'three';
import cases from './mapped-circle-cases.json';
import schema from './mapped-circle.schema.json';
import mapping from './mapping-cases.json';
import {ifcMappedCircleFixture} from './circle-extrusion-fixtures.ts';
import {extractIfc4x3Triangles} from '../geometry.ts';
test('mapped circle physical policy contract is closed',()=>{const valid=new Ajv({strict:true}).compile(schema);expect(valid(cases)).toBe(true);expect(valid({...cases,publish:true})).toBe(false);});
for(const row of cases.cases)test('world circle '+row.id,async()=>{
 const fixture=ifcMappedCircleFixture(row,mapping.baseBounds),{axes}=fixture;
 const before=structuredClone(fixture.snapshot),outcome=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,curveSampling:row.sampling,maxValues:row.maxValues});expect(fixture.snapshot).toEqual(before);if(row.expected==='refuse'){expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}expect(outcome.diagnostics).toEqual([]);const mesh=outcome.artifact![0]!,n=mesh.positions.length/2;expect(n).toBe(row.segments!);
 const world=new Matrix4().makeBasis(...axes.map((axis,i)=>new Vector3(...axis as [number,number,number]).multiplyScalar(row.scales[i]!)) as [Vector3,Vector3,Vector3]).setPosition(10,20,30),tolerance=Math.max(row.sampling.absoluteChordErrorMetres,row.sampling.relativeChordError*row.worldRadius);let error=0;
 for(let at=0;at<n;at++){const angle=2*Math.PI*(at+.5)/n+row.profileRotation,exact=new Vector3(Math.cos(angle),Math.sin(angle),0).applyAxisAngle(new Vector3(0,1,0),row.solidTilt).applyMatrix4(world),chord=new Vector3(...mesh.positions[at]!).add(new Vector3(...mesh.positions[(at+1)%n]!)).multiplyScalar(.5);error=Math.max(error,exact.distanceTo(chord));}expect(error).toBeLessThanOrEqual(tolerance*(1+1e-9));
});
