/** 🧊️ Complete faceted items retain original topology, edits and physical geometry through STEP and GLB. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import {ShapeUtils,Vector2} from 'three';
import cases from './faceted-brep-cases.json';
import schema from './faceted-brep.schema.json';
import {ifcFacetedFixture} from './faceted-brep-fixtures.ts';
import {extractIfc4x3Triangles,type IfcTriangleMesh} from '../geometry.ts';
import {ifc4x3ToGltf,writeIfcGltfGlb,readIfcGltfGlb,gltfToIfc4x3} from '../gltf.ts';
import {parseIfc4x3Step,writeIfc4x3Step,ifcReal} from '../step.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {ifcMutationDiff,inverseIfcMutation,applyIfcMutation} from '../../🧬️schema/🧬️mutations/🟦️.ts';
import {applyIfcDiff,inverseIfcDiff,absorbIfcDiff,type IfcDiff} from '../../🧬️schema/🔺️diff/🟦️.ts';
const volume=(mesh:IfcTriangleMesh)=>{const origin=mesh.positions[0]!;let value=0;for(let at=0;at<mesh.indices.length;at+=3){const[a,b,c]=[0,1,2].map(k=>mesh.positions[mesh.indices[at+k]!]!.map((v,i)=>v-origin[i]!));value+=(a![0]!*(b![1]!*c![2]!-b![2]!*c![1]!)+a![1]!*(b![2]!*c![0]!-b![0]!*c![2]!)+a![2]!*(b![0]!*c![1]!-b![1]!*c![0]!))/6;}return value;};
const bounds=(mesh:IfcTriangleMesh)=>[0,1].map(side=>[0,1,2].map(axis=>(side?Math.max:Math.min)(...mesh.positions.map(p=>p[axis]!))));
const closed=(mesh:IfcTriangleMesh)=>{const edges=new Map<string,number[]>();for(let at=0;at<mesh.indices.length;at+=3)for(let k=0;k<3;k++){const a=mesh.indices[at+k]!,b=mesh.indices[at+(k+1)%3]!,key=Math.min(a,b)+':'+Math.max(a,b),values=edges.get(key)??[];values.push(a<b?1:-1);edges.set(key,values);}for(const values of edges.values())expect(values.toSorted()).toEqual([-1,1]);};
const area=(outer:number[][],holes:number[][][])=>{const a=outer.map(p=>new Vector2(p[0],p[1])),b=holes.map(r=>r.map(p=>new Vector2(p[0],p[1]))),points=[...a,...b.flat()];let value=0;for(const[i,j,k]of ShapeUtils.triangulateShape(a,b)){const p=points[i!]!,q=points[j!]!,r=points[k!]!;value+=Math.abs((q.x-p.x)*(r.y-p.y)-(q.y-p.y)*(r.x-p.x))/2;}return value;};
test('closed faceted physical inputs have unchanged independent provenance',async()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,nativeExecution:true})).toBe(false);expect(createHash('sha256').update(await readFile(new URL('./faceted-brep-cases.json',import.meta.url))).digest('hex')).toBe('e4e2e4bd7a6bec441b2b71f04dd57799e5c97db014f81e55828b6d2e68381dda');expect(createHash('sha256').update(await readFile(new URL('./faceted-brep.schema.json',import.meta.url))).digest('hex')).toBe('d834570219ae0d4f81fae3b0f2fe4aa1ba0d2d0cb57424482e69d667435c788f');});
for(const row of cases.cases)test('faceted represented geometry '+row.id,async()=>{
 const fixture=ifcFacetedFixture(row),source=fixture.snapshot,before=structuredClone(source),result=await extractIfc4x3Triangles(source,{batchSize:4096});expect(source).toEqual(before);
 if(row.expected==='unsupported'){expect(result.artifact??[]).toEqual([]);expect(result.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}
 expect(result.diagnostics).toEqual([]);expect(result.artifact).toHaveLength(1);const mesh=result.artifact![0]!;expect(mesh.entityId).toBe(fixture.solid.id);expect(mesh.productId).toBe(fixture.product.id);expect(volume(mesh)).toBeCloseTo(row.volume!,8);closed(mesh);
 for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)expect(bounds(mesh)[side]![axis]).toBeCloseTo(row.bounds![side]![axis]!,9);
 const shape=row.geometry,reference=(area(shape.outer,shape.holes)*shape.height-shape.voids.reduce((sum,v)=>sum+area(v.outer,v.holes)*v.height,0))*(row.unit.coordinateScale*row.unit.siScale)**3;expect(reference).toBeCloseTo(volume(mesh),8);
 const exported=await ifc4x3ToGltf(source,{batchSize:4096}),bytes=await writeIfcGltfGlb(exported.artifact!,{batchSize:4096}),imported=await gltfToIfc4x3(await readIfcGltfGlb(bytes,{batchSize:4096}),{batchSize:4096}),roundtrip=(await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096})).artifact![0]!;expect(volume(roundtrip)).toBeCloseTo(row.volume!,4);
});
for(const row of [
 {file:'brep-model.ifc',hash:'b247026e85ef163acd5a165177d8293e606e7b8f83348a9897a3a04967fe4fc1',product:1000n,vertices:8,triangles:12,volume:2,bounds:[[.5,-.5,0],[1.5,.5,2]]},
 {file:'basin-faceted-brep.ifc',hash:'d2bdf9d5f485ff77ce9723a24ceb75a5532b51d1788b9a5f3f5145944d32065f',product:213n,vertices:160,triangles:316,volume:.002037647249806292,bounds:[[-.304879990220018,-.15350296491882,-.09399999999999911],[.30487999011286304,.268843232748677,0]]}
])test('untouched official faceted '+row.file,async()=>{
 const bytes=await readFile(new URL('../../🧪️fixtures/official/'+row.file,import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe(row.hash);const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),before=structuredClone(source),result=await extractIfc4x3Triangles(source,{batchSize:4096}),mesh=result.artifact?.find(m=>m.productId===row.product);expect(mesh).toBeDefined();expect(result.diagnostics).toEqual([]);expect(source).toEqual(before);expect(mesh!.positions).toHaveLength(row.vertices);expect(mesh!.indices.length/3).toBe(row.triangles);expect(volume(mesh!)).toBeCloseTo(row.volume,10);closed(mesh!);for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)expect(bounds(mesh!)[side]![axis]).toBeCloseTo(row.bounds[side]![axis]!,9);
});
test('actual faceted point edits centralize diffs and both inverses before exact STEP restoration',async()=>{
 const fixture=ifcFacetedFixture(cases.cases[0]!),source=fixture.snapshot,before=structuredClone(source),step=await writeIfc4x3Step(source);let changed=source,diff:IfcDiff={};
 for(const point of fixture.points){const coordinates=point.args[0];if(coordinates?.kind!=='aggregate'||coordinates.value[0]?.kind!=='real'||coordinates.value[0].value.bits!==4611686018427387904n)continue;const mutation={mutation:'setEntityValue' as const,id:point.id,index:0,path:[0],value:ifcReal(3)},patch=ifcMutationDiff(changed,mutation),inverse=inverseIfcMutation(changed,mutation),next=applyIfcDiff(changed,patch);expect(applyIfcMutation(next,inverse)).toEqual(changed);diff=absorbIfcDiff(diff,patch);changed=next;}
 expect(volume((await extractIfc4x3Triangles(changed,{batchSize:4096})).artifact![0]!)).toBeCloseTo(9,10);expect(applyIfcDiff(source,diff)).toEqual(changed);const restored=applyIfcDiff(changed,inverseIfcDiff(source,diff));expect(restored).toEqual(source);expect(await writeIfc4x3Step(restored)).toBe(step);expect(source).toEqual(before);
});
for(const endpoint of ['triangles','glb'])for(const frontier of ['initial','interior','completion'])for(const fault of ['error','abort','undefined'])test('original faceted '+endpoint+' '+frontier+' '+fault,async()=>{
 const source=ifcFacetedFixture(cases.cases.find(row=>row.id==='face-hole')!).snapshot,before=structuredClone(source),events:number[]=[];
 const run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const result=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(result.artifact!,control);};
 await run({batchSize:64,progress:event=>{if(event.phase==='facets')events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,abort=new AbortController(),replacement=new AbortController(),original=fault==='error'?new Error('original observer'):undefined;let calls=0,redirected=0,caught=false;
 const control:IfcIoControl={batchSize:64,signal:abort.signal,progress(event){if(event.phase!=='facets'||calls++!==target)return;control.signal=replacement.signal;control.progress=()=>redirected++;if(fault==='abort')abort.abort();else throw original;}};
 try{await run(control);}catch(error){caught=true;if(fault==='abort'){expect(error).toBeInstanceOf(IfcStepError);expect(error).toHaveProperty('code','io.cancelled');}else expect(error).toBe(original);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(redirected).toBe(0);expect(source).toEqual(before);
});
