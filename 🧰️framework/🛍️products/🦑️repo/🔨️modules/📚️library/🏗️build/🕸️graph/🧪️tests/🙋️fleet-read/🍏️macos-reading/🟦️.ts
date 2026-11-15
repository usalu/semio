import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";
import{createHostAvailableBytesV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";

/** 🍏️ Contract: a 16 GB Mac worker plans by its host memory, not by the free memory of the moment: the first plan line of an idle or a busy worker says "it can give about 3512 MB per project" and "192 of the others (48 per wave)" and names only the heavy projects; the workspace root is heavy and runs alone after the others with up to about 8428 MB (the solo grant), while the saturated seed of s-dev-composition is attempted and never refused by a number; a 4 GB Mac that cannot run a wave has a grant of 0 MB and asks the fleet for everything.
 * @see ../../../🤝️prefetch/🟦️.ts */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const stat=(free:number,inactive:number,file:number)=>`Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:      ${free}.\nPages active:    200000.\nPages inactive:  ${inactive}.\nPages speculative: 5000.\nPages wired down: 150000.\nPages purgeable: 8000.\nFile-backed pages: ${file}.\nPages occupied by compressor: 40000.\n`;
const reading=(text:string)=>createHostAvailableBytesV1({platform:"darwin",run:()=>text,total:()=>16384*1048576,environment:{}})();
const request=(count:number,extra:readonly string[],roots:Record<string,string>)=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(host:number){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-macos-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(host);
 try{await fleetReadStaleGenerationV1({root,request:request(517,["workspace","@semio-tech/s-dev-composition"],{workspace:"."}),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(host)},hostMegabytes:()=>host});return{lines:lines.join(""),code:undefined as string|undefined};}
 catch(error){return{lines:lines.join(""),code:(error as {code?:string}).code};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}

test("an idle 16 GB Mac (10 GB available): grant about 3512 MB per project, 192 of the others inferred here (48 per wave), the workspace too heavy for the share and inferred alone with up to about 8428 MB",async()=>{
 const result=await run(16384);
 expect(result.code).toBeUndefined();
 expect(result.lines).toContain("1 too heavy for this machine (workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)");
 expect(result.lines).toContain("the fleet did not deliver 1 too heavy project(s) (workspace (about 6200 MB)); this machine infers them itself, one at a time and alone after the others, each with up to about 8428 MB");
 expect(result.lines).not.toContain("@semio-tech/s-dev-composition (about");expect(result.lines).not.toContain("about 0 MB");
});

test("a busy 16 GB Mac (6.7 GB available) plans like an idle one: the plan judges the host, not the free memory of the moment",async()=>{
 expect(reading(stat(9000,420000,300000))).toBeLessThan(reading(stat(15000,620000,330000)));
 const busy=await run(16384),idle=await run(16384);
 expect(busy.lines).toBe(idle.lines);
 expect(busy.lines).toContain("it can give about 3512 MB per project");expect(busy.lines).toContain("it can infer 192 of the others (48 per wave)");expect(busy.lines).toContain("1 too heavy for this machine");
});

test("a 4 GB Mac that cannot run the smallest wave has a grant of 0 MB and asks the fleet for everything, the read refuses",async()=>{
 const result=await run(4096);
 expect(result.code).toBe("unserved");
 expect(result.lines).toContain("it can give about 0 MB per project");expect(result.lines).toContain("it can infer 0 of the others (0 per wave)");
});
