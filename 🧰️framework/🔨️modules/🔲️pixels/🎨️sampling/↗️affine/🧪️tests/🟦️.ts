/** 🧫️ Shared sampled pixels with independent polygon-clipping and native-canvas references. */
import {test,expect,spyOn} from "bun:test";
import Ajv from "ajv";
import {areaOracle,canvasOracle,delta} from "./🔭️oracles/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {AffineImageJob,sampleAffineImage,type AffineImageInput} from "../🟦️.ts";
import {CoverageJob} from "../../../🖊️coverage/🟦️.ts";
import retirementCases from "../🧫️fixtures/🧹️retirement/🔣️.json";
const validate=new Ajv({strict:true}).compile(schema);
const validateProgress=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/progress"});
const validateRetirement=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/retirement"});
test("actual affine owners survive interruptions and drain genuine coverage before source detachment",()=>{let comparisons=0;
 for(const row of retirementCases)for(const grant of [1,7,4096]){
  const source=fixture.find(v=>v.name===row.source)!,v=input(source.input);if(row.mode==="failure")v.transform=[1e9,0,0,1,1e9,0];
  const before=structuredClone(v),job=new AffineImageJob(v),state=job as any,records:{source:CoverageJob;job:any;work:number}[]=[];
  const original=CoverageJob.prototype.intoRetirement,spy=spyOn(CoverageJob.prototype,"intoRetirement").mockImplementation(function(this:CoverageJob){
   const retired=original.call(this),record={source:this,job:retired.job,work:0};records.push(record);const advance=retired.job.advance.bind(retired.job);
   retired.job.advance=unit=>{expect(unit).toBe(1);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return retired;
  });
  try{
   if(row.mode==="failure")expect(()=>job.advance(100000)).toThrow();else{let reached=false;for(let at=0;at<2000000;at++){if(state.phase===row.phase){reached=true;break;}const p=job.advance(1);expect(validateProgress(p)).toBe(true);}expect(reached).toBe(true);if(row.steps)job.advance(row.steps);}
   const child=state.coverage,closing=state.coverageRetirement,candidate=state.output.pixels,published=row.mode==="published"?job.result():null;
   if(row.mode==="cancelled"||published){job.cancel();expect(state.source).toBe(v.source);expect(state.coverage).toBe(child);if(!published)expect(state.output.pixels).toBe(candidate);}
   const retired=(job as any).intoRetirement();expect(retired.output!==null).toBe(row.phase==="complete"&&row.mode==="live");if(retired.output)expect(retired.output.pixels).toBe(candidate);
   if(child)expect(records.some(r=>r.source===child)).toBe(true);if(closing)expect(state.coverageRetirement).toBe(closing);expect(state.source).toBe(v.source);
   expect(()=>job.advance(1)).toThrow(/cancel/);expect(()=>job.result()).toThrow(/cancel/);expect(()=>(job as any).intoRetirement()).toThrow(/transferred/);job.cancel();
   for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>retired.job.advance(invalid)).toThrow(/grant/i);
   let work=0,done=false;
   for(let at=0;at<2000000;at++){const points=state.coverageContour?.length??0,p=retired.job.advance(grant);expect(validateRetirement(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(points-(state.coverageContour?.length??0)).toBeLessThanOrEqual(grant);if(records.some(r=>!r.job.terminalIsEmpty())){expect(state.source).toBe(v.source);expect(state.coverageContour).not.toBeNull();}work=p.work;if(p.done){done=true;break;}}
   expect(done).toBe(true);expect(retired.job.terminalIsEmpty()).toBe(true);expect(retired.job.advance(1)).toEqual({phase:"complete",work,done:true});
   for(const key of ["source","coverage","coverageRetirement","coverageContour","mask","inverse"])expect(state[key]).toBeNull();
   for(const key of ["quad","scratchA","scratchB","sums"])expect(state[key].length).toBe(0);expect(state.output.pixels.length).toBe(0);for(const record of records)expect(record.job.terminalIsEmpty()).toBe(true);
   expect(v).toEqual(before);const image=retired.output??published;if(image){expect([...image.pixels]).toEqual(source.expected);const m=v.transform,shrinks=Math.hypot(m[0]!,m[1]!)<1||Math.hypot(m[2]!,m[3]!)<1;expect(delta(image.pixels,v.source.width*v.source.height===1||v.sampling==="area"||v.sampling==="auto"&&shrinks?areaOracle(v):canvasOracle(v))).toBeLessThanOrEqual(2);comparisons++;}
   console.error(`[DEBUG] Actual affine retirement ${row.name}: grant=${grant} work=${work} terminal_empty=true children=${records.length}`);
  }finally{spy.mockRestore();}
 }
 console.error(`[DEBUG] Affine preserved output matched ${comparisons} independent polygon and canvas comparisons`);
});
test("normal affine sampling waits for actual coverage retirement before touching output",()=>{
 const v=input(fixture[9]!.input),before=structuredClone(v),job=new AffineImageJob(v),state=job as any;let adopted=0,work=0,closed:any=null;
 const original=CoverageJob.prototype.intoRetirement,spy=spyOn(CoverageJob.prototype,"intoRetirement").mockImplementation(function(this:CoverageJob){
  const mask=this.result(),retired=original.call(this);closed=retired;adopted++;expect(retired.output).toBe(mask);const advance=retired.job.advance.bind(retired.job);
  retired.job.advance=unit=>{expect(unit).toBe(1);expect(state.phase).toBe("coverageCleanup");expect(state.coverage).toBeNull();expect(state.mask).toBe(mask);expect(state.at).toBe(0);expect(state.output.pixels.every((v:number)=>v===0)).toBe(true);const p=advance(unit);expect(p.work-work).toBe(1);work=p.work;return p;};return retired;
 });
 try{let done=false;for(let at=0;at<2000000;at++){const p=job.advance(1);expect(validateProgress(p)).toBe(true);if(p.phase==="sampling"){expect(closed.job.terminalIsEmpty()).toBe(true);expect(state.coverageContour).toBeNull();expect(state.coverageRetirement).toBeNull();}if(p.done){done=true;break;}}expect(done).toBe(true);}finally{spy.mockRestore();}
 expect(adopted).toBe(1);expect(work).toBeGreaterThanOrEqual(14);expect([...job.result().pixels]).toEqual(fixture[9]!.expected);expect(v).toEqual(before);console.error(`[DEBUG] Actual affine coverage handoff: ${work} one-unit grants; untouched pixels until child terminal`);
});
test("async affine finally drains genuine parent after completion abort and callback failure",async()=>{
 const original=(AffineImageJob.prototype as any).intoRetirement;
 for(const mode of ["success","abort","callback"]){const v=input(fixture[13]!.input),before=structuredClone(v),controller=new AbortController();let adopted=0,work=0,closed:any=null;
  const spy=spyOn(AffineImageJob.prototype as any,"intoRetirement").mockImplementation(function(this:AffineImageJob){adopted++;const retired=original.call(this);closed=retired;const advance=retired.job.advance.bind(retired.job);retired.job.advance=(unit:number)=>{expect(unit).toBe(1);const p=advance(unit);expect(p.work-work).toBe(1);work=p.work;return p;};return retired;});
  try{const pending=sampleAffineImage(v,{signal:controller.signal,workBudget:1,onProgress:p=>{if(p.done){if(mode==="abort")controller.abort();if(mode==="callback")throw Error("affine completion callback failed");}}});if(mode==="success"){const image=await pending;expect(image).toBe(closed.output);expect([...image.pixels]).toEqual(fixture[13]!.expected);}else{await expect(pending).rejects.toThrow(mode==="abort"?/cancel/:/completion callback failed/);expect(closed.output).toBeNull();}expect(adopted).toBe(1);expect(work).toBeGreaterThanOrEqual(11);expect(closed.job.terminalIsEmpty()).toBe(true);expect(v).toEqual(before);console.error(`[DEBUG] Actual async affine ${mode}: one adoption and ${work} one-unit retirement grants`);}finally{spy.mockRestore();}
 }
});
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
 const validateGrant=new Ajv({strict:true}).compile(schema.definitions.grant);
 for(const grant of [1,7,4096])expect(validateGrant(grant)).toBe(true);
 for(const grant of [0,-1,1.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1]){expect(validateGrant(grant)).toBe(false);expect(()=>new AffineImageJob(v).advance(grant)).toThrow();}
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
