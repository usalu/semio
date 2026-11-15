import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";

/** 🌍️ Contract: the Linux worker of the fleet (16 GB declared as 15848 MB, asked from a slice booked with 1 GB) takes the decision by what it can give, not by the booking nor by the free memory of the moment: it infers a wave of unknown projects itself, names only the heavy ones (the workspace root; the saturated seed of s-dev-composition is attempted, never refused by a number), and says in plain text what it can infer; a machine declared as 4 GB cannot run a wave and asks the fleet for everything. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const request=(count:number,extra:readonly string[]=[],roots:Record<string,string>={})=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(count:number,environment:Record<string,string>,extra:readonly string[]=[],roots:Record<string,string>={}){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-booking-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;
 try{await fleetReadStaleGenerationV1({root,request:request(count,extra,roots),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,...environment}});return{lines,error:undefined};}
 catch(error){return{lines,error:error as {name:string;key:string;code:string}};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}
const MB=1048576,LINUX={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"15848"};

test("517 stale projects on the Linux worker: a small booking of the asking slice no longer turns them all into refused ones; the machine can infer a wave of 48 four times itself, and the workspace runs alone after the others",async()=>{
 const booked=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"},["workspace","@semio-tech/s-dev-composition"],{workspace:"."});
 const text=booked.lines.join("");
 expect(booked.error).toBeUndefined();
 expect(text).toContain("1 too heavy for this machine (workspace (about 6200 MB)");expect(text).toContain("it can infer 192 of the others (48 per wave)");
 expect(text).toContain("the fleet did not deliver 1 too heavy project(s) (workspace (about 6200 MB))");expect(text).not.toContain("@semio-tech/s-dev-composition (about");
 expect(text).toContain("it can give about 3298 MB per project");expect(text).toContain("each with up to about 8000 MB");
});

test("without a measured heavy project 517 stale projects are light: the plan says the machine can infer 192 of them and asks the fleet for the rest",async()=>{
 const result=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"});
 const text=result.lines.join("");
 expect(result.error).toBeUndefined();expect(text).toContain("this machine can infer 192");expect(text).not.toContain("too heavy");
});

test("a few unknown projects stay local on that machine whatever the booking, and a machine declared too small for a wave asks for everything",async()=>{
 expect(await run(40,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"})).toEqual({lines:[],error:undefined});
 const starved=await run(40,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"4096"});
 expect(starved.error?.code).toBe("unserved");expect(starved.lines.join("")).toContain("this machine can infer 0");
});
