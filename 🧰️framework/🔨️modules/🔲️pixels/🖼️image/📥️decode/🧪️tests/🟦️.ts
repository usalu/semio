import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import fundingSchema from "../../../../🌱️value/🧬️retained-clone/🧬️contract/🧬️schema/🔣️.json";
import {PNG} from "pngjs";
import cases from "../🧫️fixtures/🔣️.json";
import invalid from "../🧫️fixtures/⚠️invalid/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {ImageDecodeJob,decodeImageSource,type ImageDecodeInput} from "../🟦️.ts";
const funded={maximumItems:4096,maximumCopyBytes:536870912,maximumCapacityBytes:536870912,maximumReleaseBytes:0,maximumDepth:1};
const originals=new WeakMap<ImageDecodeJob,string>();
const create=(input:ImageDecodeInput)=>{const job=new ImageDecodeJob(input);originals.set(job,input.data);return job;};
const advance=(job:ImageDecodeJob,items:number)=>job.advance(originals.get(job)!,{...funded,maximumItems:items}).progress;
const finish=(job:ImageDecodeJob,grant:number)=>{let work=0,read=0,bytes=0,pixels=0;for(let step=0;step<2000000;step++){const p=advance(job,grant);expect(p.work-work).toBeLessThanOrEqual(grant);expect(p.sourceCompleted).toBeGreaterThanOrEqual(read);expect(p.sourceCompleted).toBeLessThanOrEqual(p.sourceTotal);expect(p.bytes).toBeGreaterThanOrEqual(bytes);expect(p.pixels).toBeGreaterThanOrEqual(pixels);work=p.work;read=p.sourceCompleted;bytes=p.bytes;pixels=p.pixels;if(p.done)return job.result();}throw Error("Image source did not finish");};
const validate=new Ajv({strict:true}).addSchema(fundingSchema).compile(schema);
for(const row of cases)test(row.name,async()=>{
 expect(validate(row.input)).toBe(true);
 for(const grant of [1,7,4096]){const job=create(row.input);expect(()=>job.result()).toThrow();const result=finish(job,grant);expect(result.width).toBe(row.expected.width);expect(result.height).toBe(row.expected.height);expect(Array.from(result.pixels)).toEqual(row.expected.pixels);}
 const data=row.input.data,comma=data.indexOf(","),referenceUrl=data.slice(0,comma).toLowerCase()+data.slice(comma),bytes=data.slice(0,5).toLowerCase()==="data:"?Buffer.from(await(await fetch(referenceUrl)).arrayBuffer()):Buffer.from(data,"base64");
 const reference=PNG.sync.read(bytes,{skipRescale:bytes[24]===16}),pixels=reference.depth===16?Array.from(reference.data as unknown as Uint16Array,v=>v>>>8):Array.from(reference.data);
 const visible=(v:number[])=>v.map((n,i)=>i%4<3&&row.expected.pixels[i-i%4+3]===0?0:n);
 expect(visible(pixels)).toEqual(visible(row.expected.pixels));
});
for(const row of invalid)test("refuse "+row.name,()=>{
 let job:ImageDecodeJob|undefined;expect(()=>{job=create(row.input);finish(job,7);}).toThrow();
 if(job){expect(()=>job!.result()).toThrow();expect(()=>advance(job!,1)).toThrow();}
});
test("source preparation owns cancellation, malformed grants and output",()=>{
 for(const steps of [0,1,10,50,200,1000,3000]){const job=create(cases[45]!.input);for(let i=0;i<steps;i++){if(advance(job,1).done)break;}job.cancel();expect(()=>advance(job,1)).toThrow();expect(()=>job.result()).toThrow();}
 for(const grant of [-1,NaN,Infinity,0.5])expect(()=>advance(create(cases[0]!.input),grant)).toThrow();
 const job=create(cases[29]!.input),result=finish(job,4096);job.cancel();expect(Array.from(result.pixels)).toEqual(cases[29]!.expected.pixels);
 console.info("[DEBUG] Encoded image sources retained private candidates and owned published RGBA");
});
test("async source preparation yields and checks observers before publication",async()=>{
 const source=cases[0]!.input,control=new AbortController();let timer=false,calls=0;setTimeout(()=>{timer=true;},0);
 await expect(decodeImageSource(source,{workGrant:funded,signal:control.signal,workBudget:7,onProgress:p=>{calls++;if(p.pixels>3)control.abort();}})).rejects.toMatchObject({name:"AbortError"});
 expect(timer).toBe(true);expect(calls).toBeGreaterThan(1);
 await expect(decodeImageSource(source,{workGrant:funded,signal:control.signal})).rejects.toMatchObject({name:"AbortError"});
 await expect(decodeImageSource(source,{workGrant:funded,onProgress:()=>{throw Error("observer");}})).rejects.toThrow("observer");
 const end=new AbortController();await expect(decodeImageSource(source,{workGrant:funded,signal:end.signal,onProgress:p=>{if(p.done)end.abort();}})).rejects.toMatchObject({name:"AbortError"});
 expect(Array.from((await decodeImageSource(source,{workGrant:funded})).pixels)).toEqual(cases[0]!.expected.pixels);
 console.info("[DEBUG] Encoded image source preparation yielded and aborted before publication");
});
test("source caps admit exact boundaries and cancellation covers each live phase",()=>{
 const row=cases[37]!,bytes=Buffer.from(row.input.data.slice(row.input.data.indexOf(",")+1),"base64").length;
 const limits={...row.input,maxSourceBytes:row.input.data.length,maxBytes:bytes,maxPixels:row.expected.width*row.expected.height};
 expect(Array.from(finish(create(limits),1).pixels)).toEqual(row.expected.pixels);
 for(const patch of [{maxSourceBytes:limits.maxSourceBytes-1},{maxBytes:bytes-1},{maxPixels:limits.maxPixels-1}])expect(()=>finish(create({...limits,...patch}),7)).toThrow();
 const seen=new Set<string>();
 const probe=create(limits);let steps=0;
 for(;;){const progress=advance(probe,1);steps++;if(!seen.has(progress.phase)&&!progress.done){seen.add(progress.phase);const cancelled=create(limits);for(let i=0;i<steps;i++)advance(cancelled,1);cancelled.cancel();expect(()=>cancelled.result()).toThrow(/cancel/);expect(()=>advance(cancelled,1)).toThrow(/cancel/);}if(progress.done)break;}
 expect([...seen]).toEqual(["header","validate","decode","source-handoff","decoder","png","image-handoff"]);
 console.info("[DEBUG] Exact source/binary/pixel caps and header/validation/decoding/PNG cancellation verified");
});

import funding from "../🧫️fixtures/🎟️funding.json";
import jpegCases from "../../../📸️jpeg/📥️decode/🧫️fixtures/🔣️.json";
import sharp from "sharp";
test("Original PNG and JPEG composition denies independent axes and preserves cancellation custody",async()=>{
 const source=cases[0]!.input;
 for(const axis of funding.axes){const job=create(source);if(axis==="maximumCapacityBytes"||axis==="releaseForCapacity")while(!job.nextCapacityByteDemand())advance(job,1);const grant={maximumItems:1,maximumCopyBytes:job.nextCopyByteDemand(),maximumCapacityBytes:job.nextCapacityByteDemand(),maximumReleaseBytes:0,maximumDepth:1};if(axis==="maximumItems")grant.maximumItems=0;else if(axis==="maximumCopyBytes")grant.maximumCopyBytes--;else if(axis==="maximumCapacityBytes")grant.maximumCapacityBytes--;else if(axis==="maximumDepth")grant.maximumDepth=0;else{grant.maximumCapacityBytes=0;grant.maximumReleaseBytes=536870912;}const before=job.progress(),denied=job.advance(source.data,grant);expect(denied.progress).toEqual(before);expect(denied.receipt).toEqual({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});expect(()=>job.advance(source.data+"x",funded)).toThrow("Original encoded image source changed");advance(job,1);}
 const seen=new Set<string>();
 for(const index of funding.jpegCases){const row=jpegCases.cases[index]!,input={mime:"image/jpeg",data:"data:image/jpeg;base64,"+row.base64,maxSourceBytes:268439552,maxBytes:67108864,maxPixels:16777216,maxChunks:65536};expect(validate(input)).toBe(true);const job=create(input);while(!job.progress().done){const phase=job.progress().phase;if(!seen.has(phase)){seen.add(phase);const cancel=create(input);while(cancel.progress().phase!==phase)advance(cancel,1);cancel.cancel();expect(()=>advance(cancel,1)).toThrow(/cancel/);expect(()=>cancel.result()).toThrow(/cancel/);}advance(job,1);}const output=job.result(),oracle=await sharp(Buffer.from(row.base64,"base64")).ensureAlpha().raw().toBuffer();expect(output.width).toBe(row.width);expect(output.height).toBe(row.height);expect(output.pixels.length).toBe(oracle.length);let maximum=0;for(let at=0;at<oracle.length;at++){maximum=Math.max(maximum,Math.abs(output.pixels[at]!-oracle[at]!));expect(oracle[at]).toBe(row.pixels[at]);}expect(maximum).toBeLessThanOrEqual(row.oracleTolerance);expect(job.takeResult({...funded,maximumCopyBytes:funding.handoffBytes-1})).toBeUndefined();expect(job.result()).toBe(output);expect(job.takeResult({...funded,maximumCopyBytes:funding.handoffBytes})!.value).toBe(output);}
 for(const phase of funding.interruptionPhases.filter(value=>value!=="png"))expect(seen.has(phase)).toBe(true);
 console.info("[DEBUG] Original PNG/JPEG composition independent grants, cancelled child custody, seven sharp oracles and funded handoffs passed");
},30000);
