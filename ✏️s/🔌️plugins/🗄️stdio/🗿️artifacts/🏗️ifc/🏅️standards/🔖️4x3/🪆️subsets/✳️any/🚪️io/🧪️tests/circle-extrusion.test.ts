/** ⭕️ Circle accuracy and resource laws govern actual geometry, canonical editing and GLB exchange. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020';
import {Box3} from 'three';
import {GLTFLoader} from 'three/examples/jsm/loaders/GLTFLoader.js';
import cases from './circle-extrusion-cases.json';
import schema from './circle-extrusion.schema.json';
import {ifcCircleFixture} from './circle-extrusion-fixtures.ts';
import {extractIfc4x3Triangles,type IfcTriangleMesh} from '../geometry.ts';
import {ifc4x3ToGltf,writeIfcGltfGlb,readIfcGltfGlb,gltfToIfc4x3} from '../gltf.ts';
import {IfcStepError} from '../control.ts';
import {parseIfc4x3Step,writeIfc4x3Step,ifcReal} from '../step.ts';
import {applyIfcDiff,inverseIfcDiff} from '../../🧬️schema/🔺️diff/🟦️.ts';
import {ifcMutationDiff} from '../../🧬️schema/🧬️mutations/🟦️.ts';
const bounds=(mesh:IfcTriangleMesh)=>[0,1].map(side=>[0,1,2].map(k=>side?Math.max(...mesh.positions.map(p=>p[k]!)):Math.min(...mesh.positions.map(p=>p[k]!))));
const volume=(mesh:IfcTriangleMesh)=>{let total=0;const origin=mesh.positions[0]!;for(let at=0;at<mesh.indices.length;at+=3){const [a,b,c]=[0,1,2].map(k=>mesh.positions[mesh.indices[at+k]!]!.map((v,j)=>v-origin[j]!));total+=(a![0]!*(b![1]!*c![2]!-b![2]!*c![1]!)+a![1]!*(b![2]!*c![0]!-b![0]!*c![2]!)+a![2]!*(b![0]!*c![1]!-b![1]!*c![0]!))/6;}return total;};
const close=(actual:number[][],expected:number[][],digits=10)=>{for(let side=0;side<2;side++)for(let k=0;k<3;k++)expect(actual[side]![k]).toBeCloseTo(expected[side]![k]!,digits);};

test('circle sweep vectors have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('circle sweep '+row.id,async()=>{
 const source=ifcCircleFixture(row).snapshot,before=structuredClone(source),control={batchSize:4096,curveSampling:row.sampling},outcome=await extractIfc4x3Triangles(source,control);
 expect(source).toEqual(before);
 if(row.expected==='refuse'){expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}
 expect(outcome.diagnostics).toEqual([]);expect(outcome.artifact).toHaveLength(1);const mesh=outcome.artifact![0]!,n=row.segments!;
 expect(mesh.positions).toHaveLength(2*n);expect(mesh.indices).toHaveLength(12*n-12);
 const ratio=n*Math.sin(2*Math.PI/n)/(2*Math.PI);expect(volume(mesh)/row.volume!).toBeCloseTo(ratio,12);
 const edges=new Map<string,number[]>();for(let at=0;at<mesh.indices.length;at+=3)for(let k=0;k<3;k++){const a=mesh.indices[at+k]!,b=mesh.indices[at+(k+1)%3]!,key=Math.min(a,b)+':'+Math.max(a,b),entry=edges.get(key)??[];entry.push(a<b?1:-1);edges.set(key,entry);}for(const edgesAt of edges.values())expect(edgesAt.toSorted((a,b)=>a-b)).toEqual([-1,1]);
 if(row.id==='default'||row.id==='absolute')close(bounds(mesh),[[-row.radius,-row.radius,0],[row.radius,row.radius,row.depth]]);
});

test('official circle opening remains editable through central diff, inverse, STEP and actual GLB',async()=>{
 const bytes=await readFile(new URL('../../🧪️fixtures/official/slab-openings.ifc',import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe('958350475139ad5ee7bfdf1181c259c9f188c977302233c10dbe5644b5843a90');
 const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),outcome=await extractIfc4x3Triangles(source,{batchSize:4096}),mesh=outcome.artifact!.find(mesh=>mesh.productId===325n)!;
 expect(mesh).toBeDefined();expect(mesh.positions).toHaveLength(144);close(bounds(mesh),[[.05,.25,-.2],[.15,.35,0]]);expect(volume(mesh)/(Math.PI*.05*.05*.2)).toBeCloseTo(72*Math.sin(2*Math.PI/72)/(2*Math.PI),12);
 expect(outcome.diagnostics.filter(d=>d.code==='geometry.opening-subtraction')).toHaveLength(2);
 const diff=ifcMutationDiff(source,{mutation:'setEntityArg',id:318n,index:3,value:ifcReal(100)}),edited=applyIfcDiff(source,diff),parsed=await parseIfc4x3Step(await writeIfc4x3Step(edited,{batchSize:4096}),{batchSize:4096});
 close(bounds((await extractIfc4x3Triangles(parsed,{batchSize:4096})).artifact!.find(mesh=>mesh.productId===325n)!),[[0,.2,-.2],[.2,.4,0]]);expect(applyIfcDiff(edited,inverseIfcDiff(source,diff))).toEqual(source);
 const fixture=ifcCircleFixture(cases.cases[0]!).snapshot,control={batchSize:4096,curveSampling:{relativeChordError:1,maxSegments:4}},exported=await ifc4x3ToGltf(fixture,control),glb=await writeIfcGltfGlb(exported.artifact!,{batchSize:4096}),independent=await new GLTFLoader().parseAsync(glb.buffer.slice(glb.byteOffset,glb.byteOffset+glb.byteLength) as ArrayBuffer,'');
 independent.scene.updateMatrixWorld(true);const box=new Box3().setFromObject(independent.scene,true);close([box.min.toArray(),box.max.toArray()],[[-1,0,-1],[1,2,1]],5);
 expect(exported.artifact!.document.accessors[0]!.count).toBe(8n);expect(exported.diagnostics.some(d=>d.code==='exchange.loss')).toBe(true);
 const imported=await gltfToIfc4x3(await readIfcGltfGlb(glb,{batchSize:4096}),{batchSize:4096}),roundtrip=(await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096})).artifact![0]!;
 expect(roundtrip.positions).toHaveLength(8);expect(roundtrip.indices).toHaveLength(36);close(bounds(roundtrip),[[-1,-1,0],[1,1,2]]);expect(volume(roundtrip)).toBeCloseTo(4,12);
});

for(const endpoint of [extractIfc4x3Triangles,ifc4x3ToGltf])for(const sampling of [{absoluteChordErrorMetres:0,relativeChordError:0},{absoluteChordErrorMetres:-1},{relativeChordError:Infinity},{relativeChordError:NaN},{relativeChordError:2},{maxSegments:3},{maxSegments:4.5},{maxSegments:65540},{absoluteChordErrorMetres:null},{relativeChordError:null},{maxSegments:null},{unknown:1}])test(endpoint.name+' rejects invalid curve sampling '+JSON.stringify(sampling),async()=>{
 const control={curveSampling:sampling} as unknown as Parameters<typeof extractIfc4x3Triangles>[1];await expect(endpoint(ifcCircleFixture(cases.cases[0]!).snapshot,control)).rejects.toMatchObject({code:'geometry.control'});
});
for(const endpoint of [extractIfc4x3Triangles,ifc4x3ToGltf])test(endpoint.name+' retains original sampling before caller mutation',async()=>{
 const curveSampling={relativeChordError:1,maxSegments:4},control={batchSize:4096,curveSampling,progress:()=>{curveSampling.relativeChordError=.000001;curveSampling.maxSegments=65536;}},outcome=await endpoint(ifcCircleFixture(cases.cases[0]!).snapshot,control);
 expect(outcome.artifact).toBeDefined();if(Array.isArray(outcome.artifact))expect(outcome.artifact[0]!.positions).toHaveLength(8);else expect(outcome.artifact!.document.accessors[0]!.count).toBe(8n);
});
for(const endpoint of [extractIfc4x3Triangles,ifc4x3ToGltf])for(const frontier of ['initial','interior','completion'])for(const kind of ['domain','undefined','number','cancel'])test(endpoint.name+' circle '+frontier+' '+kind,async()=>{
 const controller=new AbortController(),failure:unknown=kind==='domain'?new IfcStepError('geometry.invalid','original caller refusal'):kind==='number'?19:undefined,source=ifcCircleFixture(cases.cases[0]!).snapshot,before=structuredClone(source);let reached=false,caught:unknown=Symbol('not caught');
 const control={signal:controller.signal,batchSize:16,progress:(event:{phase:string;completed:number;total:number})=>{if(event.phase==='extrusions'&&(frontier==='initial'?event.completed===0:frontier==='interior'?event.completed>0&&event.total===0:event.completed>0&&event.completed===event.total)){reached=true;if(kind==='cancel')controller.abort();else throw failure;}}};
 try{await endpoint(source,control);}catch(error){caught=error;}
 expect(reached).toBe(true);if(kind==='cancel')expect(caught).toMatchObject({code:'io.cancelled'});else expect(caught).toBe(failure);expect(source).toEqual(before);
});
test('circle output values obey the original resource grant',async()=>{
 const source=ifcCircleFixture(cases.cases[0]!).snapshot,outcome=await extractIfc4x3Triangles(source,{batchSize:4096,maxValues:500});expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);
});
