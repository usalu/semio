import {test,expect} from "bun:test";
import Ajv from "ajv";
import {PNG} from "pngjs";
import cases from "../🧫️fixtures/🔣️.json";
import invalid from "../🧫️fixtures/⚠️invalid/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {ImageDecodeJob,decodeImageSource,type ImageDecodeInput} from "../🟦️.ts";
const finish=(job:ImageDecodeJob,grant:number)=>{let work=0,read=0,bytes=0,pixels=0;for(let step=0;step<2000000;step++){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);expect(p.sourceCompleted).toBeGreaterThanOrEqual(read);expect(p.sourceCompleted).toBeLessThanOrEqual(p.sourceTotal);expect(p.bytes).toBeGreaterThanOrEqual(bytes);expect(p.pixels).toBeGreaterThanOrEqual(pixels);work=p.work;read=p.sourceCompleted;bytes=p.bytes;pixels=p.pixels;if(p.done)return job.result();}throw Error("Image source did not finish");};
const validate=new Ajv({strict:true}).compile(schema);
for(const row of cases)test(row.name,async()=>{
 expect(validate(row.input)).toBe(true);
 for(const grant of [1,7,4096]){const job=new ImageDecodeJob(row.input);expect(()=>job.result()).toThrow();const result=finish(job,grant);expect(result.width).toBe(row.expected.width);expect(result.height).toBe(row.expected.height);expect(Array.from(result.pixels)).toEqual(row.expected.pixels);}
 const data=row.input.data,comma=data.indexOf(","),referenceUrl=data.slice(0,comma).toLowerCase()+data.slice(comma),bytes=data.slice(0,5).toLowerCase()==="data:"?Buffer.from(await(await fetch(referenceUrl)).arrayBuffer()):Buffer.from(data,"base64");
 const reference=PNG.sync.read(bytes,{skipRescale:bytes[24]===16}),pixels=reference.depth===16?Array.from(reference.data as unknown as Uint16Array,v=>v>>>8):Array.from(reference.data);
 const visible=(v:number[])=>v.map((n,i)=>i%4<3&&row.expected.pixels[i-i%4+3]===0?0:n);
 expect(visible(pixels)).toEqual(visible(row.expected.pixels));
});
for(const row of invalid)test("refuse "+row.name,()=>{
 let job:ImageDecodeJob|undefined;expect(()=>{job=new ImageDecodeJob(row.input);finish(job,7);}).toThrow();
 if(job){expect(()=>job!.result()).toThrow();expect(()=>job!.advance(1)).toThrow();}
});
test("source preparation owns cancellation, malformed grants and output",()=>{
 for(const steps of [0,1,10,50,200,1000,3000]){const job=new ImageDecodeJob(cases[45]!.input);for(let i=0;i<steps;i++){if(job.advance(1).done)break;}job.cancel();expect(()=>job.advance(1)).toThrow();expect(()=>job.result()).toThrow();}
 for(const grant of [0,-1,NaN,Infinity,0.5])expect(()=>new ImageDecodeJob(cases[0]!.input).advance(grant)).toThrow();
 const job=new ImageDecodeJob(cases[29]!.input),result=finish(job,4096);job.cancel();expect(Array.from(result.pixels)).toEqual(cases[29]!.expected.pixels);
 console.info("[DEBUG] Encoded image sources retained private candidates and owned published RGBA");
});
test("async source preparation yields and checks observers before publication",async()=>{
 const source=cases[37]!.input,control=new AbortController();let timer=false,calls=0;setTimeout(()=>{timer=true;},0);
 await expect(decodeImageSource(source,{signal:control.signal,workBudget:7,onProgress:p=>{calls++;if(p.pixels>3)control.abort();}})).rejects.toMatchObject({name:"AbortError"});
 expect(timer).toBe(true);expect(calls).toBeGreaterThan(1);
 await expect(decodeImageSource(source,{signal:control.signal})).rejects.toMatchObject({name:"AbortError"});
 await expect(decodeImageSource(source,{onProgress:()=>{throw Error("observer");}})).rejects.toThrow("observer");
 const end=new AbortController();await expect(decodeImageSource(source,{signal:end.signal,onProgress:p=>{if(p.done)end.abort();}})).rejects.toMatchObject({name:"AbortError"});
 expect(Array.from((await decodeImageSource(source)).pixels)).toEqual(cases[37]!.expected.pixels);
 console.info("[DEBUG] Encoded image source preparation yielded and aborted before publication");
});
test("source caps admit exact boundaries and cancellation covers each live phase",()=>{
 const row=cases[37]!,bytes=Buffer.from(row.input.data.slice(row.input.data.indexOf(",")+1),"base64").length;
 const limits={...row.input,maxSourceBytes:row.input.data.length,maxBytes:bytes,maxPixels:row.expected.width*row.expected.height};
 expect(Array.from(finish(new ImageDecodeJob(limits),1).pixels)).toEqual(row.expected.pixels);
 for(const patch of [{maxSourceBytes:limits.maxSourceBytes-1},{maxBytes:bytes-1},{maxPixels:limits.maxPixels-1}])expect(()=>finish(new ImageDecodeJob({...limits,...patch}),7)).toThrow();
 const seen=new Set<string>();
 const probe=new ImageDecodeJob(limits);let steps=0;
 for(;;){const progress=probe.advance(1);steps++;if(!seen.has(progress.phase)&&!progress.done){seen.add(progress.phase);const cancelled=new ImageDecodeJob(limits);for(let i=0;i<steps;i++)cancelled.advance(1);cancelled.cancel();expect(()=>cancelled.result()).toThrow(/cancel/);expect(()=>cancelled.advance(1)).toThrow(/cancel/);}if(progress.done)break;}
 expect([...seen]).toEqual(["header","validate","decode","png"]);
 console.info("[DEBUG] Exact source/binary/pixel caps and header/validation/decoding/PNG cancellation verified");
});
