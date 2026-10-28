/** 🪢️ Exact finite polyline profiles reuse the shared checked ring triangulation through actual exchange. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import {ShapeUtils,Vector2} from 'three';
import cases from './polyline-extrusion-cases.json';
import schema from './polyline-extrusion.schema.json';
import {ifcPolylineFixture} from './polyline-extrusion-fixtures.ts';
import {extractIfc4x3Triangles,type IfcTriangleMesh} from '../geometry.ts';
import {ifc4x3ToGltf,gltfToIfc4x3,writeIfcGltfGlb,readIfcGltfGlb} from '../gltf.ts';
import {parseIfc4x3Step} from '../step.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
const volume=(mesh:IfcTriangleMesh)=>{let total=0;for(let at=0;at<mesh.indices.length;at+=3){const[a,b,c]=[0,1,2].map(k=>mesh.positions[mesh.indices[at+k]!]!);total+=(a![0]*(b![1]*c![2]-b![2]*c![1])+a![1]*(b![2]*c![0]-b![0]*c![2])+a![2]*(b![0]*c![1]-b![1]*c![0]))/6;}return total;};
test('polyline sweep cases have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('polyline area extrusion '+row.id,async()=>{
 const source=ifcPolylineFixture(row).snapshot,before=structuredClone(source),outcome=await extractIfc4x3Triangles(source,{batchSize:4096});expect(source).toEqual(before);
 if(row.expected==='refuse'){expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}
 expect(outcome.artifact).toHaveLength(1);expect(outcome.diagnostics).toEqual([]);const mesh=outcome.artifact![0]!;expect(volume(mesh)).toBeCloseTo(row.volume!,9);
 for(let side=0;side<2;side++)for(let k=0;k<3;k++)expect((side?Math.max:Math.min)(...mesh.positions.map(p=>p[k]!))).toBeCloseTo(row.bounds![side]![k]!,10);
 const outer=row.outer.slice(0,-1).map(p=>new Vector2(p[0],p[1])),holes=(row.holes??[]).map(r=>r.slice(0,-1).map(p=>new Vector2(p[0],p[1]))),points=outer.concat(...holes),triangles=ShapeUtils.triangulateShape(outer,holes);
 let area=0;for(const[a,b,c]of triangles){const p=points[a!]!,q=points[b!]!,r=points[c!]!;area+=Math.abs((q.x-p.x)*(r.y-p.y)-(q.y-p.y)*(r.x-p.x))/2;}expect(area).toBeCloseTo(row.area!,8);
 const edges=new Map<string,number[]>();for(let at=0;at<mesh.indices.length;at+=3)for(let k=0;k<3;k++){const a=mesh.indices[at+k]!,b=mesh.indices[at+(k+1)%3]!,key=Math.min(a,b)+':'+Math.max(a,b),entry=edges.get(key)??[];entry.push(a<b?1:-1);edges.set(key,entry);}for(const edge of edges.values())expect(edge.toSorted()).toEqual([-1,1]);
 const exported=await ifc4x3ToGltf(source,{batchSize:4096}),bytes=await writeIfcGltfGlb(exported.artifact!,{batchSize:4096}),imported=await gltfToIfc4x3(await readIfcGltfGlb(bytes,{batchSize:4096}),{batchSize:4096}),roundtrip=(await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096})).artifact![0]!;expect(volume(roundtrip)).toBeCloseTo(row.volume!,4);
});
test('original official scheduling polyline bodies use authored inch units',async()=>{
 const bytes=await readFile(new URL('../../🧪️fixtures/official/construction-scheduling-task.ifc',import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe('abbb30934dd36ea1370d7464300cd5a33c85ebd438ab10ed2fd258823514fb7d');
 const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),result=await extractIfc4x3Triangles(source,{batchSize:4096});
 for(const[id,expected]of [[292n,5.43683454566397],[356n,2.26534772736002],[385n,3.39802159103996],[407n,2.26534772735999],[429n,3.39802159104],[208n,927.886429126656]] as const){const mesh=result.artifact?.find(row=>row.productId===id);expect(mesh).toBeDefined();expect(mesh!.positions).toHaveLength(8);expect(mesh!.indices).toHaveLength(36);expect(volume(mesh!)).toBeCloseTo(expected,8);}
 expect(result.diagnostics.some(d=>d.code==='geometry.unsupported-item')).toBe(true);
});
for(const endpoint of ['triangles','glb'])for(const frontier of ['initial','interior','completion'])for(const fault of ['error','abort','undefined'])test('original voided sweep '+endpoint+' '+frontier+' '+fault,async()=>{
 const source=ifcPolylineFixture(cases.cases.find(row=>row.id==='two-holes')!).snapshot,before=structuredClone(source),events:number[]=[];
 const run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const result=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(result.artifact!,control);};
 await run({batchSize:64,progress:event=>{if(event.phase==='extrusions')events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,original=new AbortController(),replacement=new AbortController(),failure=fault==='error'?new Error('caller'):undefined;let calls=0,replacementCalls=0,caught=false;
 const control:IfcIoControl={batchSize:64,signal:original.signal,progress(event){if(event.phase!=='extrusions'||calls++!==target)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;if(fault==='abort')original.abort();else throw failure;}};
 try{await run(control);}catch(error){caught=true;if(fault==='abort')expect(error).toBeInstanceOf(IfcStepError);else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(replacementCalls).toBe(0);expect(source).toEqual(before);
});
