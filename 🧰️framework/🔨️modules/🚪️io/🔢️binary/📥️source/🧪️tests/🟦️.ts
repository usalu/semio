import {test,expect} from "bun:test";
import Ajv from "ajv";
import {toByteArray,fromByteArray} from "base64-js";
import {resolve} from "node:path";
import fonts from "../🧫️fixtures/🔤️fonts/🔣️.json";
import cases from "../🧫️fixtures/🔣️.json";
import invalid from "../🧫️fixtures/⚠️invalid/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {BinarySourceJob,decodeBinarySource} from "../🟦️.ts";
const ajv=new Ajv({strict:true});ajv.addSchema(schema);
const valid=ajv.compile({$ref:schema.$id+"#/definitions/input"}),progressValid=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),outputValid=ajv.compile({$ref:schema.$id+"#/definitions/result"});
const finish=(job:BinarySourceJob,grant:number)=>{let previous=0,source=0,bytes=0;for(let step=0;step<2000000;step++){const p=job.advance(grant);expect(progressValid(p)).toBe(true);expect(p.work-previous).toBeLessThanOrEqual(grant);expect(p.sourceCompleted).toBeGreaterThanOrEqual(source);expect(p.sourceCompleted).toBeLessThanOrEqual(p.sourceTotal);expect(p.bytes).toBeGreaterThanOrEqual(bytes);expect(p.bytes).toBeLessThanOrEqual(p.totalBytes);previous=p.work;source=p.sourceCompleted;bytes=p.bytes;if(p.done){const result=job.result();expect(outputValid(Array.from(result))).toBe(true);return {result,p};}}throw Error("Source did not finish");};
for(const row of cases)test(row.name,async()=>{
 expect(valid(row.input)).toBe(true);
 for(const grant of [1,7,4096]){const job=new BinarySourceJob(row.input);expect(()=>job.result()).toThrow();expect(Array.from(finish(job,grant).result)).toEqual(row.expected);}
 const data=row.input.data,comma=data.indexOf(","),url=data.slice(0,comma).toLowerCase()+data.slice(comma);
 const oracle=data.slice(0,5).toLowerCase()==="data:"?new Uint8Array(await(await fetch(url)).arrayBuffer()):Buffer.from(data,"base64");
 expect(Array.from(oracle)).toEqual(row.expected);
 const base64=data.slice(0,5).toLowerCase()!=="data:"?data:data.slice(0,comma).toLowerCase().endsWith(";base64")?decodeURIComponent(data.slice(comma+1)):undefined;
 if(base64!==undefined)expect(Array.from(toByteArray(base64))).toEqual(row.expected);
});
for(const row of invalid)test("Refuse "+row.name,()=>{
 let job:BinarySourceJob|undefined,error:unknown;
 try{job=new BinarySourceJob(row.input);finish(job,7);}catch(e){error=e;}
 expect(error).toBeDefined();
 if(job){expect(()=>job!.result()).toThrow();expect(()=>job!.advance(1)).toThrow();}
});
test("Exact work/source/byte boundaries and sticky private refusal",()=>{
 const row=cases[0]!,baseline=finish(new BinarySourceJob(row.input),1),work=baseline.p.work;
 expect(Array.from(finish(new BinarySourceJob({...row.input,maxWork:work}),7).result)).toEqual(row.expected);
 for(const patch of [{maxWork:work-1},{maxSourceBytes:row.input.data.length-1},{maxBytes:row.expected.length-1},{minBytes:row.expected.length+1,maxBytes:row.expected.length+1}]){
  let job:BinarySourceJob|undefined;expect(()=>{job=new BinarySourceJob({...row.input,...patch});finish(job,1);}).toThrow();
  if(job){let first:unknown;try{job.advance(1);}catch(e){first=e;}try{job.advance(1);}catch(e){expect(e).toBe(first);}expect(()=>job!.result()).toThrow();}
 }
 for(const budget of [0,-1,NaN,Infinity,0.5])expect(()=>new BinarySourceJob(row.input).advance(budget)).toThrow();
 console.info("[DEBUG] Encoded binary font/image sources admit exact source/byte/work limits and refuse private overages");
});
test("Every source phase cancels and published output stays owned",()=>{
 const row=cases[1]!,probe=new BinarySourceJob(row.input),seen=new Set<string>();let steps=0;
 for(;;){const p=probe.advance(1);steps++;if(!seen.has(p.phase)){seen.add(p.phase);const job=new BinarySourceJob(row.input);for(let n=0;n<steps;n++)job.advance(1);const published=p.done?job.result():undefined;job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);if(published)expect(Array.from(published)).toEqual(row.expected);}if(p.done)break;}
 expect([...seen]).toEqual(["header","validate","decode","complete"]);
 expect(row.input.data).toBe(cases[1]!.input.data);
 console.info("[DEBUG] Encoded binary source header/validation/decode/publication cancellation preserves published bytes");
});
test("Async source decoding yields and checks completion observers",async()=>{
 const row=cases[1]!;let timer=false,calls=0;setTimeout(()=>{timer=true;},0);
 expect(Array.from(await decodeBinarySource(row.input,{workBudget:7,onProgress:()=>calls++}))).toEqual(row.expected);expect(timer).toBe(true);expect(calls).toBeGreaterThan(1);
 for(const phase of ["header","validate","decode","complete"]){const control=new AbortController();await expect(decodeBinarySource(row.input,{signal:control.signal,workBudget:1,onProgress:(p:{phase:string})=>{if(p.phase===phase)control.abort();}})).rejects.toMatchObject({name:"AbortError"});}
 const stopped=new AbortController();stopped.abort();await expect(decodeBinarySource(row.input,{signal:stopped.signal})).rejects.toMatchObject({name:"AbortError"});
 await expect(decodeBinarySource(row.input,{onProgress:()=>{throw Error("observer");}})).rejects.toThrow("observer");
 console.info("[DEBUG] Binary font/image resource jobs yield and observer abort prevents publication");
});

for(const row of fonts)test(row.name,async()=>{
 const path=resolve(import.meta.dir,"../../../../../..",row.resource),source=new Uint8Array(await Bun.file(path).arrayBuffer());
 expect(source.length).toBe(row.bytes);
 const data="data:"+row.mime+";name=Document%20Font;base64,"+fromByteArray(source),input={mime:row.mime,data,minBytes:row.bytes,maxBytes:row.bytes,maxSourceBytes:data.length,maxWork:data.length*2+2};
 expect(valid(input)).toBe(true);
 const output=await decodeBinarySource(input,{workBudget:4096});expect(Array.from(output)).toEqual(Array.from(source));expect(Array.from(toByteArray(data.slice(data.indexOf(",")+1)))).toEqual(Array.from(source));
 expect(Array.from(new Uint8Array(await(await fetch(data)).arrayBuffer()))).toEqual(Array.from(source));
 console.info("[DEBUG] Bundled "+row.mime+" encoded resource retained all "+row.bytes+" bytes through the real source job");
});
