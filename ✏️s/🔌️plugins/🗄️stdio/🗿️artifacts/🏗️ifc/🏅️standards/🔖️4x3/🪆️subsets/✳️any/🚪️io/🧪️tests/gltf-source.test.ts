/** 🫳️ Real Khronos artifact adapters interpret only completed private sources. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './gltf-source-cases.json';
import schema from './gltf-source.schema.json';
import {gltfToIfc4x3,readIfcGltfGlb,writeIfcGltfGlb} from '../gltf.ts';
import type {IfcIoControl} from '../control.ts';
const bytes=()=>readFile(new URL('box.glb',import.meta.url));
const fixture=async()=>readIfcGltfGlb(await bytes());
const comparable=(value:any)=>{const result=structuredClone(value);if(result.artifact)result.artifact.header.fileName[1]={kind:'string',value:'generated timestamp'};return result;};
const run=async(route:string,source:Awaited<ReturnType<typeof fixture>>,control:IfcIoControl={})=>route==='importGltf'?gltfToIfc4x3(source,control):writeIfcGltfGlb(source,control);

test('glTF source vectors have a closed neutral schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const phase of cases.semanticPhases)test('loaded glTF source is private at '+phase,async()=>{
 const route=phase.startsWith('gltf-')?'importGltf':'writeGlb',source=await fixture(),expected=comparable(await run(route,source));let changed=false;
 const actual=await run(route,source,{progress:progress=>{if(progress.phase===phase&&!changed){changed=true;source.document.nodes[0]!.name='caller mutation';source.buffers[0]!.fill(0);}}});
 expect(changed).toBe(true);expect(comparable(actual)).toEqual(expected);
});
for(const route of cases.loadedRoutes)for(const variant of cases.accessors)test(route+' capture refuses '+variant+' without invoking it',async()=>{
 const source=await fixture();let owner:object=source,key=variant,reads=0;
 if(variant==='asset')owner=source.document;
 if(variant==='byte'){owner=source.buffers[0]!;key='0';}
 if(variant==='constructor')owner=source.document.nodes;
 const value=Object.getOwnPropertyDescriptor(owner,key)?.value;Object.defineProperty(owner,key,{get:()=>{reads++;return value;},enumerable:true,configurable:true});
 let result:any,error:any;try{result=await run(route,source);}catch(caught){error=caught;}
 expect(reads).toBe(0);if(route==='importGltf'){expect(result?.artifact).toBeUndefined();expect(result?.diagnostics[0]?.code).toBe(cases.sourceRefusal);}else expect(error?.code).toBe(cases.sourceRefusal);
});
for(const route of cases.loadedRoutes)test(route+' source limits refuse before semantic work',async()=>{
 for(const limit of cases.limits){let semantic=false,result:any,error:any;try{result=await run(route,await fixture(),{...limit,progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;}});}catch(caught){error=caught;}
 expect(semantic).toBe(false);if(route==='importGltf'){expect(result?.artifact).toBeUndefined();expect(result?.diagnostics[0]?.code).toBe(cases.sourceRefusal);}else expect(error?.code).toBe(cases.sourceRefusal);}
});
for(const route of cases.loadedRoutes)for(const phase of cases.capturePhases)test(route+' '+phase+' retains original cancellation',async()=>{
 const source=await fixture(),before=structuredClone(source),original=new AbortController(),replacement=new AbortController();let reached=false,semantic=false,replacements=0;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;if(progress.phase===phase&&progress.completed>=2){reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}}};
 await expect(run(route,source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(semantic).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});
for(const phase of ['glb','glb-bytes'])test('raw GLB remains private at '+phase,async()=>{
 const source=await bytes(),expected=await readIfcGltfGlb(source);let changed=false;
 const actual=await readIfcGltfGlb(source,{progress:progress=>{if(progress.phase===phase&&!changed){changed=true;source.fill(0);}}});
 expect(changed).toBe(true);expect(actual).toEqual(expected);
});
for(const key of cases.rawAccessors)test('raw GLB borrows internal '+key+' without a caller getter',async()=>{
 const source=await bytes(),expected=await readIfcGltfGlb(source);let reads=0;Object.defineProperty(source,key,{get:()=>{reads++;throw Error('caller getter');}});
 let actual;try{actual=await readIfcGltfGlb(source);}catch{}expect(reads).toBe(0);expect(actual).toEqual(expected);
});
test('raw GLB capture cancels inside the original byte view',async()=>{
 const source=await bytes(),before=Buffer.from(source),original=new AbortController(),replacement=new AbortController();let reached=false,semantic=false,replacements=0;
 const control:IfcIoControl={signal:original.signal,batchSize:16,progress:progress=>{if(progress.phase==='glb')semantic=true;if(progress.phase==='capture-glb-bytes'&&progress.completed>=16){reached=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}}};
 await expect(readIfcGltfGlb(source,control)).rejects.toHaveProperty('code',cases.cancellation);expect(reached).toBe(true);expect(semantic).toBe(false);expect(replacements).toBe(0);expect(source).toEqual(before);
});
test('raw GLB source uses the exact admitted subarray extent',async()=>{
 const source=await bytes(),outer=new Uint8Array(source.length+17);outer.fill(255);outer.set(source,9);
 expect(await readIfcGltfGlb(outer.subarray(9,9+source.length))).toEqual(await readIfcGltfGlb(source));
});
test('raw GLB source brand refusal does not invoke proxy or tag accessors',async()=>{
 const source=await bytes(),expected=await readIfcGltfGlb(source);let reads=0;
 Object.defineProperty(source,Symbol.toStringTag,{get:()=>{reads++;throw Error('caller tag');}});
 expect(await readIfcGltfGlb(source)).toEqual(expected);expect(reads).toBe(0);
 const proxy=new Proxy(source,{get:()=>{reads++;throw Error('caller proxy');}});
 await expect(readIfcGltfGlb(proxy)).rejects.toHaveProperty('code','glb.source');expect(reads).toBe(0);
 for(const other of [new Int8Array(32),new Uint16Array(32),new DataView(new ArrayBuffer(32))])await expect(readIfcGltfGlb(other as unknown as Uint8Array)).rejects.toHaveProperty('code','glb.source');
});
for(const route of cases.loadedRoutes)test(route+' validates unused source buffer bytes',async()=>{
 for(const byte of [-1,256,0.5,NaN,Infinity]){
  const source=await fixture();source.buffers[0]!.push(byte);let semantic=false,result:any,error:any;
  try{result=await run(route,source,{progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;}});}catch(caught){error=caught;}
  expect(semantic).toBe(false);if(route==='importGltf'){expect(result?.artifact).toBeUndefined();expect(result?.diagnostics[0]?.code).toBe(cases.byteRefusals.importGltf);}else expect(error?.code).toBe(cases.byteRefusals.writeGlb);
 }
});
