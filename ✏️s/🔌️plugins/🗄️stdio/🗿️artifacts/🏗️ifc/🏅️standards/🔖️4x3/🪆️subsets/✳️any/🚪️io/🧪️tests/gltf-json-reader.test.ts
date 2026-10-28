import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './gltf-json-reader-cases.json';
import schema from './gltf-json-reader-cases.schema.json';
import {readIfcGltfGlb} from '../gltf.ts';
import {parseGltfDocument} from '../../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts';
import {decodeJsonSyntax} from '@semio-tech/pack-json-decode-ts';
import {JsonMemberPolicy} from '@semio-tech/framework-pack-json-ts/members';
import type {IfcIoControl} from '../control.ts';

function glb(source:string):Uint8Array{
 const json=new TextEncoder().encode(source),length=Math.ceil(json.length/4)*4,bytes=new Uint8Array(32+length),view=new DataView(bytes.buffer);
 view.setUint32(0,0x46546c67,true);view.setUint32(4,2,true);view.setUint32(8,bytes.length,true);view.setUint32(12,length,true);view.setUint32(16,0x4e4f534a,true);bytes.fill(32,20,20+length);bytes.set(json,20);view.setUint32(20+length,4,true);view.setUint32(24+length,0x004e4942,true);bytes[28+length]=1;return bytes;
}
const native={parse:(source:string)=>JSON.parse(source) as unknown};
test('neutral GLB JSON reader fixture schema is closed',()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,cases)).toBe(true);expect(ajv.validate(schema,{...cases,skip:true})).toBe(false);});
for(const row of cases.cases)test('actual GLB endpoint preserves native JSON/schema roles '+row.id,async()=>{
 const actual=await readIfcGltfGlb(glb(row.source),{batchSize:256});expect(actual.document).toEqual(parseGltfDocument(native.parse(row.source)));expect(actual.buffers).toEqual([[1]]);expect(({} as Record<string,unknown>).polluted).toBeUndefined();
});
test('owned syntax projects through shared glTF wire member/default tables',async()=>{
 const parser=await import('../../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts');
 for(const row of cases.cases){const syntax=await decodeJsonSyntax(row.source,JsonMemberPolicy.Replace,{maximumBytes:65536,maximumNodes:4096,maximumDepth:64,chunk:64,cancelled:()=>false,progress:()=>{},yield:async()=>{}}),steps=parser.parseGltfDocumentSyntaxSteps(syntax);let turns=0;for(;;){const step=steps.next();if(step.done){expect(step.value).toEqual(parseGltfDocument(native.parse(row.source)));break;}turns++;}expect(turns).toBeGreaterThan(10);}
});
const long=JSON.stringify({asset:{version:'2.0'},buffers:[{byteLength:1}],nodes:[{name:'ä😀'.repeat(1024)}],extras:{values:Array.from({length:256},(_,i)=>i)}});
for(const phase of ['glb-json-decode','glb-json-syntax','glb-json-project'])for(const boundary of ['interior','completion'])test('original GLB reader cancellation '+phase+' '+boundary,async()=>{
 const controller=new AbortController(),other=new AbortController(),bytes=glb(long),before=bytes.slice();let reached=false,otherCalls=0;
 const control:IfcIoControl={signal:controller.signal,batchSize:64,progress:event=>{if(event.phase!==phase||event.completed===0)return;const complete=event.total>0&&event.completed===event.total;if((boundary==='completion')!==complete)return;reached=true;control.signal=other.signal;control.progress=()=>otherCalls++;control.batchSize=4096;controller.abort();}};
 await expect(readIfcGltfGlb(bytes,control)).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(otherCalls).toBe(0);expect(bytes).toEqual(before);
});
for(const batchSize of [1,2,3,4])test('streaming fatal UTF8 and one leading BOM preserve split boundaries '+batchSize,async()=>{
 const source='\ufeff'+cases.cases.find(row=>row.id==='unicode-strings')!.source;expect((await readIfcGltfGlb(glb(source),{batchSize})).document).toEqual(parseGltfDocument(native.parse(source.slice(1))));
});
test('reader phases use private captured bytes after caller mutation',async()=>{
 const source=cases.cases[0]!.source,bytes=glb(source);let changed=false;const result=await readIfcGltfGlb(bytes,{batchSize:64,progress:event=>{if(event.phase==='capture-glb-bytes'&&event.completed===event.total){bytes.fill(255);changed=true;}}});expect(changed).toBe(true);expect(result.document).toEqual(parseGltfDocument(native.parse(source)));
});

test('syntax depth counts scalar roots and overwritten source branches before schema defaults',async()=>{
 const source=cases.cases[0]!.source;expect((await readIfcGltfGlb(glb(source),{maxDepth:4,batchSize:256})).document).toEqual(parseGltfDocument(native.parse(source)));await expect(readIfcGltfGlb(glb(source),{maxDepth:3,batchSize:256})).rejects.toMatchObject({code:'glb.limit'});
 const discarded=source.replace('{','{"extras":{"ignored":[[[0]]]},"extras":null,');await expect(readIfcGltfGlb(glb(discarded),{maxDepth:4,batchSize:256})).rejects.toMatchObject({code:'glb.limit'});
});
test('retained numeric conversion enforces the explicit original token ceiling before Number',async()=>{
 const token='1e+'+'0'.repeat(31),source=cases.cases[0]!.source.replace('{','{"scene":'+token+',');expect((await readIfcGltfGlb(glb(source),{maxNumericCodeUnits:token.length,batchSize:256})).document.scene).toBe(1n);await expect(readIfcGltfGlb(glb(source),{maxNumericCodeUnits:token.length-1,batchSize:256})).rejects.toMatchObject({code:'glb.limit'});
 const overwritten=source.replace('"scene":'+token,'"scene":'+token+',"scene":0');expect((await readIfcGltfGlb(glb(overwritten),{maxNumericCodeUnits:1,batchSize:256})).document.scene).toBe(0n);
});
for(const prefix of ['\ufeff\ufeff','\u0000','false'])test('malformed JSON has a stable domain refusal '+JSON.stringify(prefix),async()=>{
 await expect(readIfcGltfGlb(glb(prefix+cases.cases[0]!.source),{batchSize:256})).rejects.toMatchObject({code:'glb.json'});
});
