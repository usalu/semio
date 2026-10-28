/** 🧊️ Exact rectangular sweeps retain original geometry through editing, STEP and actual GLB endpoints. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import {Box3} from 'three';
import {GLTFLoader} from 'three/examples/jsm/loaders/GLTFLoader.js';
import cases from './rectangle-extrusion-cases.json';
import schema from './rectangle-extrusion.schema.json';
import {ifcRectangleFixture} from './rectangle-extrusion-fixtures.ts';
import {extractIfc4x3Triangles,type IfcTriangleMesh} from '../geometry.ts';
import {ifc4x3ToGltf,gltfToIfc4x3,writeIfcGltfGlb,readIfcGltfGlb} from '../gltf.ts';
import {parseIfc4x3Step,writeIfc4x3Step,ifcReal} from '../step.ts';
import {applyIfcDiff,inverseIfcDiff} from '../../🧬️schema/🔺️diff/🟦️.ts';
import {ifcMutationDiff} from '../../🧬️schema/🧬️mutations/🟦️.ts';
const bounds=(mesh:IfcTriangleMesh)=>[0,1].map(side=>[0,1,2].map(k=>side?Math.max(...mesh.positions.map(p=>p[k]!)):Math.min(...mesh.positions.map(p=>p[k]!))));
const volume=(mesh:IfcTriangleMesh)=>{let total=0;for(let at=0;at<mesh.indices.length;at+=3){const [a,b,c]=[0,1,2].map(k=>mesh.positions[mesh.indices[at+k]!]!);total+=(a![0]*(b![1]*c![2]-b![2]*c![1])+a![1]*(b![2]*c![0]-b![0]*c![2])+a![2]*(b![0]*c![1]-b![1]*c![0]))/6;}return total;};
const close=(actual:number[][],expected:number[][],digits=10)=>{for(let side=0;side<2;side++)for(let k=0;k<3;k++)expect(actual[side]![k]).toBeCloseTo(expected[side]![k]!,digits);};
test('rectangular extrusion vectors have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('rectangular sweep '+row.id,async()=>{
 const source=ifcRectangleFixture(row).snapshot,before=structuredClone(source),outcome=await extractIfc4x3Triangles(source,{batchSize:4096});
 expect(source).toEqual(before);
 if(row.expected==='refuse'){expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}
 expect(outcome.diagnostics).toEqual([]);expect(outcome.artifact).toHaveLength(1);const mesh=outcome.artifact![0]!;
 expect(mesh.positions).toHaveLength(8);expect(mesh.indices).toHaveLength(36);close(bounds(mesh),row.bounds!);expect(volume(mesh)).toBeCloseTo(row.volume!,10);
 const edges=new Map<string,number[]>();for(let at=0;at<36;at+=3)for(let k=0;k<3;k++){const a=mesh.indices[at+k]!,b=mesh.indices[at+(k+1)%3]!,key=Math.min(a,b)+':'+Math.max(a,b),entry=edges.get(key)??[];entry.push(a<b?1:-1);edges.set(key,entry);}for(const edgesAt of edges.values())expect(edgesAt.toSorted((a,b)=>a-b)).toEqual([-1,1]);
 const exported=await ifc4x3ToGltf(source,{batchSize:4096});expect(exported.diagnostics.some(d=>d.code==='exchange.loss')).toBe(true);
 const bytes=await writeIfcGltfGlb(exported.artifact!,{batchSize:4096}),independent=await new GLTFLoader().parseAsync(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength) as ArrayBuffer,'');independent.scene.updateMatrixWorld(true);
 const box=new Box3().setFromObject(independent.scene,true);close([box.min.toArray(),box.max.toArray()],[[row.bounds![0]![0]!,row.bounds![0]![2]!,-row.bounds![1]![1]!],[row.bounds![1]![0]!,row.bounds![1]![2]!,-row.bounds![0]![1]!]],5);
 const imported=await gltfToIfc4x3(await readIfcGltfGlb(bytes,{batchSize:4096}),{batchSize:4096}),roundtrip=(await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096})).artifact![0]!;close(bounds(roundtrip),row.bounds!,5);expect(volume(roundtrip)).toBeCloseTo(row.volume!,4);
});
test('official rectangular extrusion remains editable through central diff, inverse and STEP',async()=>{
 const bytes=await readFile(new URL('../../🧪️fixtures/official/extruded-solid.ifc',import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe('c488f0367106a17289468409531b1b687f72c25cea8e1b46924f59972ad7a8e9');
 const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),original=(await extractIfc4x3Triangles(source)).artifact![0]!;close(bounds(original),[[.5,-.5,0],[1.5,.5,2]]);expect(volume(original)).toBeCloseTo(2,12);
 const profile=source.entities.find(row=>row.name==='IFCRECTANGLEPROFILEDEF')!,diff=ifcMutationDiff(source,{mutation:'setEntityArg',id:profile.id,index:3,value:ifcReal(2000)}),edited=applyIfcDiff(source,diff),parsed=await parseIfc4x3Step(await writeIfc4x3Step(edited,{batchSize:4096}),{batchSize:4096});
 const mesh=(await extractIfc4x3Triangles(parsed)).artifact![0]!;close(bounds(mesh),[[0,-.5,0],[2,.5,2]]);expect(volume(mesh)).toBeCloseTo(4,12);expect(applyIfcDiff(edited,inverseIfcDiff(source,diff))).toEqual(source);
});
