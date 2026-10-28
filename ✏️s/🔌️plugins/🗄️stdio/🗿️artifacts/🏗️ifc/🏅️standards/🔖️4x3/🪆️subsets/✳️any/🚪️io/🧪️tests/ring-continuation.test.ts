/** 🕳️ The real shared ring core supports original bounded asynchronous controls. */
import {expect,test} from 'bun:test';
import {ShapeUtils,Vector2} from 'three';
import {triangulateRings,triangulateRingsSteps,type RingPoint} from '../../../../../../📐️geometry/🕳️rings/🟦️.ts';
import {runIfcIoSteps,IfcStepError,type IfcIoControl} from '../control.ts';
import cases from '../../../../../../📐️geometry/🕳️rings/🧪️tests/🔣️.json';
const fixture=(row:typeof cases.cases[number])=>{const points:RingPoint[]=[],rings:number[][]=[];for(const source of row.rings){const ids:number[]=[];for(const point of source){ids.push(points.length);points.push([...point] as RingPoint);}rings.push(ids);}return{points,rings};};
for(const row of cases.cases)test('controlled existing ring core '+row.name,async()=>{
 const {points,rings}=fixture(row),before=structuredClone({points,rings}),actual=await runIfcIoSteps(triangulateRingsSteps(points,rings),{batchSize:4096},'rings'),expected=triangulateRings(points,rings,()=>{});
 expect(actual).toEqual(expected);expect({points,rings}).toEqual(before);
 const oracle=ShapeUtils.triangulateShape(row.rings[0]!.map(p=>new Vector2(p[0],p[1])),row.rings.slice(1).map(r=>r.map(p=>new Vector2(p[0],p[1]))));
 const area=(triangles:number[][])=>triangles.reduce((sum,[a,b,c])=>sum+Math.abs((points[b!]![0]-points[a!]![0])*(points[c!]![1]-points[a!]![1])-(points[b!]![1]-points[a!]![1])*(points[c!]![0]-points[a!]![0]))/2,0);
 expect(area(actual)).toBeCloseTo(row.area,8);expect(area(oracle)).toBeCloseTo(row.area,8);
});
for(const frontier of ['initial','interior','completion'])for(const fault of ['abort','error','undefined','number'])test('original ring continuation '+frontier+' '+fault,async()=>{
 const {points,rings}=fixture(cases.cases[0]!),before=structuredClone({points,rings}),events:number[]=[];await runIfcIoSteps(triangulateRingsSteps(points,rings),{batchSize:32,progress:event=>events.push(event.completed)},'rings');expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,original=new AbortController(),replacement=new AbortController(),failure=fault==='error'?new Error('caller'):fault==='number'?17:undefined;let calls=0,replacementCalls=0,caught=false;
 const control:IfcIoControl={batchSize:32,signal:original.signal,progress(){if(calls++!==target)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;if(fault==='abort')original.abort();else throw failure;}};
 try{await runIfcIoSteps(triangulateRingsSteps(points,rings),control,'rings');}catch(error){caught=true;if(fault==='abort')expect(error).toBeInstanceOf(IfcStepError);else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(replacementCalls).toBe(0);expect({points,rings}).toEqual(before);
});
