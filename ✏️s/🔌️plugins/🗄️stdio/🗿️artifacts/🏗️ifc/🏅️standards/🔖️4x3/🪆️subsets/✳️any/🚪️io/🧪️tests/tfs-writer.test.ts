/** 🔺 Direct source writers reserve their genuine owned output and retain original triangulation domains. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {createHash} from 'node:crypto';
import {readFile} from 'node:fs/promises';
import * as source from '../geometry/triangulated.ts';
import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from '../geometry/mesh-writer.ts';
import {IfcGeometryGraph,ifcIdentity,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {runIfcIoSteps,type IfcIoSteps} from '../control.ts';
import {ifcTfsFixture,type IfcTfsCase} from './tfs-admission-fixtures.ts';
import inputs from './tfs-admission-cases.json';
import cases from './tfs-writer-cases.json';
import schema from './tfs-writer.schema.json';
type Mesh=Pick<IfcTriangleMesh,'positions'|'indices'>;
type Write=<T>(graph:IfcGeometryGraph,item:ReturnType<IfcGeometryGraph['lookup']>&{},world:IfcMatrix,scale:number,maxValues:number,make:IfcMeshWriterFactory<T>)=>IfcIoSteps<T>;
const write=():Write=>{const fn=(source as unknown as Record<string,unknown>).ifcTriangulatedFaceSetWriteSteps;expect(typeof fn).toBe('function');return fn as Write;};
const drain=<T>(steps:IfcIoSteps<T>):T=>{for(;;){const step=steps.next();if(step.done)return step.value;}};
async function fixture(id:string){const row=inputs.cases.find(row=>row.id===id)! as IfcTfsCase,value=ifcTfsFixture(row),graph=(await IfcGeometryGraph.capture(value.snapshot,{batchSize:4096})).artifact!;return{...value,graph,item:graph.lookup(value.faceSet.id)!};}
test('closed direct triangulation contract pins the existing32 source cases',async()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect(createHash('sha256').update(await readFile(new URL('tfs-admission-cases.json',import.meta.url))).digest('hex')).toBe(cases.sourceSha256);expect(cases.normalTransport).toBe(false);});
for(const row of cases.cases)test('actual direct triangulation source '+row.id,async()=>{const fn=write(),value=await fixture(row.id),before=structuredClone(value.snapshot);let births=0,closed=0,finished=0;const make:IfcMeshWriterFactory<Mesh>=(v,t)=>{births++;const out=ifcArrayMeshWriterFactory(100000)(v,t);return{point:(...p)=>out.point(...p),triangle:(...p)=>out.triangle(...p),finish:()=>{finished++;return out.finish();},close:()=>{closed++;out.close();}};};const steps=fn(value.graph,value.item,ifcIdentity(),1,100000,make);if(row.expected==='refuse'){expect(()=>drain(steps)).toThrow();expect(finished).toBe(0);}else{const mesh=drain(steps),work=drain(source.IfcTriangulatedFaceWork.captureSteps(value.graph,value.item,ifcIdentity(),1,100000));for(const phase of[work.verticesSteps(),work.normalsSteps(),work.pnSteps(),work.trianglesSteps(),work.validationSteps()])drain(phase);expect(mesh).toEqual({positions:work.mesh().positions,indices:work.mesh().indices});expect(finished).toBe(1);expect(births).toBe(1);}expect(closed).toBe(births);expect(value.snapshot).toEqual(before);});
