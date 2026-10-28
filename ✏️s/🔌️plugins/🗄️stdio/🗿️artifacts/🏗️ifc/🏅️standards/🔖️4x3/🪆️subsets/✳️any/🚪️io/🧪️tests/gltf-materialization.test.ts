/** ⏱️ Post-accessor materialization is bounded by the same original cancellation authority. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './gltf-materialization-cases.json';
import schema from './gltf-materialization.schema.json';
import {gltfToIfc4x3,ifc4x3ToGltf,readIfcGltfGlb} from '../gltf.ts';
import {parseIfc4x3Step} from '../step.ts';
import type {IfcIoControl} from '../control.ts';

test('glTF materialization vectors have a closed neutral schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const route of cases.routes)for(const phase of cases.phases)for(const boundary of cases.boundaries)test(route+' '+phase+' '+boundary+' cancels before publication',async()=>{
 const source=await readIfcGltfGlb(await readFile(new URL('box.glb',import.meta.url)));
 if(route==='implicit')delete source.document.meshes[0]!.primitives[0]!.indices;
 if(route==='reflected'){const node=source.document.nodes[0]!;delete node.matrix;delete node.rotation;node.scale=[{bits:0xbff0000000000000n},{bits:0x3ff0000000000000n},{bits:0x3ff0000000000000n}];}
 const before=structuredClone(source),original=new AbortController(),replacement=new AbortController();let reached=false,replacements=0,completed=false;
 const control:IfcIoControl={signal:original.signal,batchSize:cases.batchSize,progress:progress=>{
  if(progress.phase==='gltf-import'&&progress.completed===progress.total)completed=true;
  if(progress.phase===phase&&(boundary==='interior'?progress.completed>=cases.interior:progress.completed===progress.total)){if(boundary==='interior')expect(progress.completed).toBeLessThan(progress.total);else expect(progress.completed).toBe(progress.total);reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}
 }};
 await expect(gltfToIfc4x3(source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(completed).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});
for(const phase of cases.exportPhases)for(const boundary of cases.boundaries)test('export '+phase+' '+boundary+' cancels before publication',async()=>{
 const source=await parseIfc4x3Step(await readFile(new URL('triangulated.ifc',import.meta.url),'utf8')),before=structuredClone(source),original=new AbortController(),replacement=new AbortController();let reached=false,replacements=0,completed=false;
 const control:IfcIoControl={signal:original.signal,batchSize:cases.exportBatchSize,progress:progress=>{
  if(progress.phase==='gltf'&&progress.completed===progress.total)completed=true;
  if(progress.phase===phase&&(boundary==='interior'?progress.completed>=cases.interior:progress.completed===progress.total)){if(boundary==='interior')expect(progress.completed).toBeLessThan(progress.total);else expect(progress.completed).toBe(progress.total);reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}
 }};
 await expect(ifc4x3ToGltf(source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(completed).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});
