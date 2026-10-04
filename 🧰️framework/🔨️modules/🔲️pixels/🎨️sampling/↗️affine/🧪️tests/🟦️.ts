/** 🧫️ Shared sampled pixels with independent polygon-clipping and native-canvas references. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {areaOracle,canvasOracle,delta} from "./🔭️oracles/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {AffineImageJob,sampleAffineImage,type AffineImageInput} from "../🟦️.ts";
const validate=new Ajv({strict:true}).compile(schema);
function input(v:any):AffineImageInput{return {...v,source:{...v.source,pixels:new Uint8Array(v.source.pixels)}};}
function complete(value:AffineImageInput,budget=4096){const job=new AffineImageJob(value);let work=0;for(let step=0;step<2000000;step++){const p=job.advance(budget);expect(p.work-work).toBeLessThanOrEqual(budget);work=p.work;if(p.done)return job.result();}throw Error("Affine sampler did not terminate");}
for(const row of fixture)test(row.name,()=>{
 expect(validate(row.input)).toBe(true);const v=input(row.input),before=structuredClone(v);for(const budget of [1,7,4096])expect([...complete(v,budget).pixels]).toEqual(row.expected);expect(v).toEqual(before);
 const m=v.transform,shrinks=Math.hypot(m[0]!,m[1]!)<1||Math.hypot(m[2]!,m[3]!)<1;
 const reference=v.source.width*v.source.height===1||v.sampling==="area"||v.sampling==="auto"&&shrinks?areaOracle(v):canvasOracle(v);
 expect(delta(new Uint8Array(row.expected),reference)).toBeLessThanOrEqual(2);
});
test("area sampling agrees with independent transformed-cell intersections",()=>{
 let seed=173;
 for(let caseAt=0;caseAt<32;caseAt++){
  const data=new Uint8Array(3*3*4);for(let i=0;i<data.length;i++){seed=(Math.imul(seed,1664525)+1013904223)>>>0;data[i]=seed>>>24;}
  const v:AffineImageInput={source:{width:3,height:3,pixels:data},width:4,height:4,origin:[-.25,.5],transform:caseAt%2?[.6,.15,.25,.8,.25,.75]:[-.7,.2,.3,.5,3,.5],sampling:"area"};
  expect(delta(complete(v).pixels,areaOracle(v))).toBeLessThanOrEqual(1);
 }
 console.error("[DEBUG] 32 affine area samples matched independent polygon-clipping intersections");
});
test("affine sample lifecycle cancels candidates, rejects partial output and keeps failures sticky",()=>{
 const v=input(fixture[9]!.input);
 for(const steps of [0,1,5,20,60]){const job=new AffineImageJob(v);expect(()=>job.result()).toThrow();for(let i=0;i<steps;i++)job.advance(1);job.cancel();expect(()=>job.advance(1)).toThrow(/cancel/);expect(()=>job.result()).toThrow(/cancel/);}
 const job=new AffineImageJob(v);while(!job.advance(4096).done){}const pixels=job.result().pixels;job.cancel();expect([...pixels]).toEqual(fixture[9]!.expected);
 for(const grant of [0,-1,1.5,NaN,Infinity])expect(()=>new AffineImageJob(v).advance(grant)).toThrow();
 for(const patch of [{width:0},{width:16384,height:16384},{origin:[Infinity,0]},{transform:[1,0,0,1,NaN,0]},{sampling:"unknown"},{source:{width:1,height:1,pixels:new Uint8Array(3)}}])expect(()=>new AffineImageJob({...v,...patch} as AffineImageInput)).toThrow();
 const failed=new AffineImageJob({...v,transform:[1e9,0,0,1,1e9,0]});expect(()=>failed.advance(100000)).toThrow();expect(()=>failed.result()).toThrow();expect(()=>failed.advance(1)).toThrow();
});
test("large reductions yield per-cell progress and respond to abort",async()=>{
 const v:AffineImageInput={source:{width:64,height:64,pixels:new Uint8Array(64*64*4).fill(255)},width:1,height:1,origin:[0,0],transform:[1/64,0,0,1/64,0,0],sampling:"auto"};
 const events:string[]=[];let maximum=0;const timer=setTimeout(()=>events.push("timer"),0);
 const image=await sampleAffineImage(v,{workBudget:256,onProgress:p=>{events.push(p.phase);maximum=Math.max(maximum,p.sampled);}});
 clearTimeout(timer);expect([...image.pixels]).toEqual([255,255,255,255]);expect(maximum).toBeGreaterThan(256);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(sampleAffineImage(v,{workBudget:1,signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toThrow(/cancel/);let seen=false;await expect(sampleAffineImage(v,{signal:controller.signal,onProgress:()=>{seen=true;}})).rejects.toThrow(/cancel/);expect(seen).toBe(false);
});
