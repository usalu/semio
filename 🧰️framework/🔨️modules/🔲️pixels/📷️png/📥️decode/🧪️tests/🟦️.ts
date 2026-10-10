import funding from "../🧫️fixtures/🎟️funding.json";
import {describe,expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import fundingSchema from "../../../../🌱️value/🧬️retained-clone/🧬️contract/🧬️schema/🔣️.json";
import {PNG} from "pngjs";
import {deflateSync,inflateSync} from "node:zlib";
import cases from "../🧫️fixtures/🔣️.json";
import invalid from "../🧫️fixtures/⚠️invalid/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PngDecodeJob,decodePngImage,type PngDecodeInput} from "../🟦️.ts";
const funded={maximumItems:4096,maximumCopyBytes:268435456,maximumCapacityBytes:67108864,maximumReleaseBytes:0,maximumDepth:1};
const advance=(job:PngDecodeJob,items:number)=>job.advance({...funded,maximumItems:items}).progress;
const input=(row:typeof cases[number]["input"]):PngDecodeInput=>({...row,data:Uint8Array.from(row.data)});
const finish=(job:PngDecodeJob,budget:number)=>{let work=0,bytes=0,pixels=0;for(let i=0;i<2000000;i++){const p=advance(job,budget);expect(p.work-work).toBeLessThanOrEqual(budget);expect(p.work).toBeGreaterThanOrEqual(work);expect(p.bytes).toBeGreaterThanOrEqual(bytes);expect(p.pixels).toBeGreaterThanOrEqual(pixels);work=p.work;bytes=p.bytes;pixels=p.pixels;if(p.done)return job.result();}throw Error("PNG job did not finish");};
const crc=(data:number[])=>{let c=0xffffffff;for(const b of data){c^=b;for(let i=0;i<8;i++)c=(c>>>1)^((c&1)?0xedb88320:0);}return(c^0xffffffff)>>>0;};
const be=(n:number)=>[n>>>24&255,n>>>16&255,n>>>8&255,n&255];
const chunk=(kind:string,data:number[])=>{const type=[...kind].map(c=>c.charCodeAt(0));return [...be(data.length),...type,...data,...be(crc([...type,...data]))];};
const build=(width:number,height:number,raw:number[],strategy?:number)=>Uint8Array.from([137,80,78,71,13,10,26,10,...chunk("IHDR",[...be(width),...be(height),8,6,0,0,0]),...chunk("IDAT",Array.from(deflateSync(Uint8Array.from(raw),strategy===undefined?{}:{strategy}))),...chunk("IEND",[])]);
describe("PNG reconstruction",()=>{
 const validate=new Ajv({strict:true}).addSchema(fundingSchema).compile(schema);
 for(const row of cases)test(row.name,()=>{
  expect(validate(row.input)).toBe(true);
  const source=input(row.input),before=source.data.slice();
  for(const grant of [1,7,4096]){const job=new PngDecodeJob(source);expect(()=>job.result()).toThrow();const result=finish(job,grant);expect(result.width).toBe(row.expected.width);expect(result.height).toBe(row.expected.height);expect(Array.from(result.pixels)).toEqual(row.expected.pixels);expect(source.data).toEqual(before);}
  const oracle=PNG.sync.read(Buffer.from(source.data),{skipRescale:source.data[24]===16});
  const pixels=oracle.depth===16?Array.from(oracle.data as unknown as Uint16Array,v=>v>>>8):Array.from(oracle.data);
  const visible=(values:number[])=>values.map((v,i)=>i%4<3&&row.expected.pixels[i-i%4+3]===0?0:v);
  expect(visible(pixels)).toEqual(visible(row.expected.pixels));
 });
 test("seeded third-party compression with dynamic, fixed and stored blocks",()=>{
  let seed=0x762ba5;const width=257,height=41,expected:number[]=[],raw:number[]=[];
  for(let y=0;y<height;y++){raw.push(0);for(let x=0;x<width*4;x++){seed=(Math.imul(seed,1664525)+1013904223)>>>0;const value=x%9===0?seed>>>24:(x+y)%32;raw.push(value);expected.push(value);}}
  for(const strategy of [undefined,4,0]){const data=build(width,height,raw,strategy);for(const grant of [1,97,4096])expect(Array.from(finish(new PngDecodeJob({data,maxPixels:16384,maxBytes:1048576,maxChunks:4096}),grant).pixels)).toEqual(expected);expect(Array.from(PNG.sync.read(Buffer.from(data)).data)).toEqual(expected);}
  console.info("[DEBUG] Seeded independent PNG compression matched RGBA under bounded grants");
 });
 for(const row of invalid)test("reject "+row.name,()=>{
  let job:PngDecodeJob|undefined;
  expect(()=>{job=new PngDecodeJob(input(row.input));finish(job,7);}).toThrow();
  if(job){const rejected=job;expect(()=>rejected.result()).toThrow();expect(()=>advance(rejected,1)).toThrow();}
  if(row.name.startsWith("incomplete dynamic")||row.name==="oversubscribed dynamic Huffman tree"){expect(String.fromCharCode(...row.input.data.slice(37,41))).toBe("IDAT");expect(()=>inflateSync(Uint8Array.from(row.input.data.slice(41,-16)))).toThrow();}
 });
 test("cancellation is sticky and private at each stage",()=>{
  for(const steps of [0,1,10,100,300,600]){const job=new PngDecodeJob(input(cases[24]!.input));for(let i=0;i<steps;i++){const p=advance(job,1);if(p.done)break;}job.cancel();expect(()=>job.result()).toThrow();expect(()=>advance(job,1)).toThrow();}
  for(const grant of [-1,NaN,Infinity,1.2]){const job=new PngDecodeJob(input(cases[0]!.input));expect(()=>advance(job,grant)).toThrow();expect(()=>job.result()).toThrow();}
 });
 test("every truncated fixture prefix refuses publication",()=>{
  let prefixes=0;for(const row of cases){for(let length=0;length<row.input.data.length;length++){expect(()=>finish(new PngDecodeJob({...input(row.input),data:Uint8Array.from(row.input.data.slice(0,length))}),4096)).toThrow();prefixes++;}}
  console.info("[DEBUG] PNG reconstruction refused all "+prefixes+" truncated fixture prefixes");
 });
 test("async grants yield, abort during progress and refuse callback failures",async()=>{
  const source=input(cases[0]!.input),control=new AbortController();let calls=0,timer=false;
  setTimeout(()=>{timer=true;},0);
  await expect(decodePngImage(source,{workGrant:funded,workBudget:7,onProgress:p=>{calls++;if(p.pixels>3)control.abort();},signal:control.signal})).rejects.toMatchObject({name:"AbortError"});
  expect(calls).toBeGreaterThan(1);expect(timer).toBe(true);
  control.abort();await expect(decodePngImage(source,{workGrant:funded,signal:control.signal})).rejects.toMatchObject({name:"AbortError"});
  await expect(decodePngImage(source,{workGrant:funded,onProgress:()=>{throw Error("observer failure");}})).rejects.toThrow("observer failure");
  const completeControl=new AbortController();await expect(decodePngImage(source,{workGrant:funded,onProgress:p=>{if(p.done)completeControl.abort();},signal:completeControl.signal})).rejects.toMatchObject({name:"AbortError"});
  const result=await decodePngImage(source,{workGrant:funded,workBudget:4096});expect(Array.from(result.pixels)).toEqual(cases[0]!.expected.pixels);expect(result.pixels.buffer).not.toBe(source.data.buffer);
  console.info("[DEBUG] Async PNG reconstruction yielded and cancelled before publication");
 });
});

test("Original PNG funding preserves independent axes, source bytes and private handoff",()=>{
 const source=input(cases[funding.fixtureCase]!.input),before=source.data.slice();
 for(const row of funding.cases){const job=new PngDecodeJob(source);if(row.axis==="maximumCapacityBytes"||row.axis==="releaseForCapacity")while(!job.nextCapacityByteDemand())job.advance({...funded,maximumItems:1});const grant={maximumItems:1,maximumCopyBytes:job.nextCopyByteDemand(),maximumCapacityBytes:job.nextCapacityByteDemand(),maximumReleaseBytes:0,maximumDepth:1};if(row.axis==="maximumItems")grant.maximumItems=0;else if(row.axis==="maximumCopyBytes")grant.maximumCopyBytes--;else if(row.axis==="maximumCapacityBytes")grant.maximumCapacityBytes--;else if(row.axis==="releaseForCapacity"){grant.maximumCapacityBytes=0;grant.maximumReleaseBytes=4096;}else if(row.axis==="maximumDepth")grant.maximumDepth=0;const old=job.progress(),step=job.advance(grant);expect(step.progress.work!==old.work).toBe(row.accepted);expect(source.data).toEqual(before);if(!row.accepted)expect(step.receipt).toEqual({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});}
 const job=new PngDecodeJob(source),result=finish(job,4096);expect(job.takeResult({...funded,maximumCopyBytes:funding.handoffBytes-1})).toBeUndefined();expect(job.result()).toBe(result);const handoff=job.takeResult({...funded,maximumCopyBytes:funding.handoffBytes})!;expect(handoff.value).toBe(result);expect(handoff.receipt.copiedBytes).toBe(funding.handoffBytes);expect(Array.from(PNG.sync.read(Buffer.from(source.data)).data)).toEqual(Array.from(result.pixels));
 console.info("[DEBUG] PNG original independent grant denial and separately funded handoff match pngjs byte oracle");
});