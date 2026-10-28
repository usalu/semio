/** 🧾️ Physical JSON emission preserves closed typed values through original scheduling. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './gltf-json-writer-cases.json';
import schema from './gltf-json-writer.schema.json';
import {readIfcGltfGlb,writeIfcGltfGlb} from '../gltf.ts';
import {encodeIfcGltfJson} from '../gltf-json.ts';
import type {IfcIoControl} from '../control.ts';
import type {GltfJson} from '../../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts';
const fixture=async()=>readIfcGltfGlb(await readFile(new URL('box.glb',import.meta.url)));
const rawJson=(bytes:Uint8Array)=>new TextDecoder('utf-8',{fatal:true}).decode(bytes.subarray(20,20+new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getUint32(12,true))).trimEnd();
const inflate=(value:unknown):unknown=>Array.isArray(value)?value.map(inflate):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,key==='bits'?BigInt(String(item)):inflate(item)])):value;

test('GLB JSON writer vectors have a closed neutral schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.texts)test('GLB JSON exact text '+row.id,async()=>{
 const source=await fixture();source.document.asset.generator=row.value;const json=rawJson(await writeIfcGltfGlb(source));
 expect(json).toContain('"generator":'+JSON.stringify(row.value));expect(JSON.parse(json).asset.generator).toBe(row.value);
});
for(const row of cases.extras)test('GLB JSON exact extras '+row.id,async()=>{
 const source=await fixture();source.document.nodes[0]!.extras=inflate(row.value) as GltfJson;const json=rawJson(await writeIfcGltfGlb(source));
 expect(json).toContain('"extras":'+row.expected);expect(JSON.stringify(JSON.parse(json).nodes[0].extras)).toBe(row.expected);expect(Object.getPrototypeOf({})).toBe(Object.prototype);
});
test('GLB JSON attributes retain last-wins and default omission',async()=>{
 const source=await fixture(),primitive=source.document.meshes[0]!.primitives[0]!;
 primitive.attributes=[['POSITION',999n],['POSITION',0n]];primitive.targets=[[]];
 const json=JSON.parse(rawJson(await writeIfcGltfGlb(source)));expect(json.meshes[0].primitives[0].attributes).toEqual({POSITION:0});expect(json.meshes[0].primitives[0].targets).toEqual([{}]);expect(json.meshes[0]).not.toHaveProperty('weights');expect(json).not.toHaveProperty('animations');
});
test('GLB JSON ordinals retain safe exact conversion and explicit refusal',async()=>{
 const source=await fixture();source.document.scene=9007199254740991n;expect(rawJson(await writeIfcGltfGlb(source))).toContain('"scene":9007199254740991');
 source.document.scene=9007199254740992n;await expect(writeIfcGltfGlb(source)).rejects.toHaveProperty('code','gltf.index');
});
test('GLB JSON byte ceiling bounds a long shared immutable name',async()=>{
 const source=await fixture();source.document.asset.generator='x'.repeat(cases.longCodeUnits);
 await expect(writeIfcGltfGlb(source,{maxValues:cases.limitedBytes})).rejects.toHaveProperty('code',cases.limit);
});
test('GLB JSON framing ceiling remains finite under a wider caller grant',async()=>{
 const source=await fixture();await expect(encodeIfcGltfJson(source.document,{maxValues:Number.MAX_SAFE_INTEGER},cases.explicitJsonByteCeiling)).rejects.toHaveProperty('code',cases.limit);
});
for(const phase of cases.phases)for(const boundary of cases.boundaries)test(phase+' '+boundary+' retains original cancellation',async()=>{
 const source=await fixture();source.document.asset.generator='日本語😀'.repeat(cases.longCodeUnits/4);
 const before=structuredClone(source),original=new AbortController(),replacement=new AbortController();let reached=false,replacements=0,completed=false;
 const control:IfcIoControl={signal:original.signal,batchSize:cases.batchSize,progress:progress=>{
  if(progress.phase==='glb'&&progress.completed===progress.total)completed=true;
  if(progress.phase===phase&&(boundary==='interior'?progress.completed>=cases.batchSize:progress.completed===progress.total)){if(boundary==='interior')expect(progress.completed).toBeLessThan(progress.total);else expect(progress.completed).toBe(progress.total);reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}
 }};
 await expect(writeIfcGltfGlb(source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(completed).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});
