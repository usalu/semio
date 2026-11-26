import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";
import{fleetAdmittedMegabytesV1,fleetGrantMegabytesV1,inferableProjectsV1,wantedPolicyV1,wantedTextV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{describeHeavyV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,soloGrantV1}from"../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{createHostAvailableBytesV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";

/** 🍏️ Contract: a 16 GB Mac worker plans by its host memory, not by the free memory of the moment: the first plan line of an idle or a busy worker says what it can give per project and how many of the others it infers (the numbers come from the declared policies and the declared host, never from the live workspace or this machine's memory: `expected` derives them from the same policy tables the production plan reads) and names only the heavy projects; the workspace root is heavy and runs alone after the others with the solo grant, while the saturated seed of s-dev-composition is attempted and never refused by a number; a 4 GB Mac that cannot run a wave infers nothing and asks the fleet for everything.
 * @see ../../../🤝️prefetch/🟦️.ts */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const stat=(free:number,inactive:number,file:number)=>`Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:      ${free}.\nPages active:    200000.\nPages inactive:  ${inactive}.\nPages speculative: 5000.\nPages wired down: 150000.\nPages purgeable: 8000.\nFile-backed pages: ${file}.\nPages occupied by compressor: 40000.\n`;
const reading=(text:string)=>createHostAvailableBytesV1({platform:"darwin",run:()=>text,total:()=>16384*1048576,environment:{}})();
const request=(count:number,extra:readonly string[],roots:Record<string,string>)=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const expected=(host:number,language:"en"|"de")=>{
 const admitted=fleetAdmittedMegabytesV1({hostMegabytes:host}),grant=ownerGrantV1(fleetGrantMegabytesV1({hostMegabytes:host}),wantedPolicyV1).managedMegabytes*ownerCostPolicyV1.footprintFactor,wave=inferableProjectsV1(admitted).waveProjects,local=wave*wantedPolicyV1.maximumLocalWaves,solo=soloGrantV1({hostMegabytes:host},wantedPolicyV1).managedMegabytes*ownerCostPolicyV1.footprintFactor;
 const named=describeHeavyV1([{name:"workspace",megabytes:ownerCostPolicyV1.seeds.workspace,source:"seed"}],language);
 return{grant,wave,local,solo,named,plan:ownerCostTextV1("plan",language,{stale:519,taken:0,heavy:1,named,grant,local,wave,wanted:519}),soloLine:ownerCostTextV1("solo",language,{count:1,named,grant:solo})};
};
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(host:number,language:"en"|"de"="en"){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-macos-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(host);
 try{await fleetReadStaleGenerationV1({root,request:request(517,["workspace","@semio-tech/s-dev-composition"],{workspace:"."}),invocation,language,environment:{PATH:process.env.PATH,HOME:root,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(host)},hostMegabytes:()=>host});return{lines:lines.join(""),code:undefined as string|undefined};}
 catch(error){return{lines:lines.join(""),code:(error as {code?:string}).code};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}

test("an idle 16 GB Mac: the grant per project and the others inferred here come from the policies, the workspace is too heavy for the share and inferred alone with the solo grant",async()=>{
 const result=await run(16384),want=expected(16384,"en");
 expect(result.code).toBeUndefined();
 expect(want.local).toBeGreaterThan(0);expect(want.solo).toBeGreaterThan(want.grant);
 expect(result.lines).toContain(want.plan);
 expect(result.lines).toContain(want.soloLine);
 expect(result.lines).not.toContain("@semio-tech/s-dev-composition (about");expect(result.lines).not.toContain("about 0 MB");
});

test("the plan and the solo line use the German template as well",async()=>{
 const result=await run(16384,"de"),want=expected(16384,"de");
 expect(result.lines).toContain(want.plan);expect(result.lines).toContain(want.soloLine);
 expect(want.plan).toContain("zu schwer für diese Maschine");
});

test("a busy 16 GB Mac (6.7 GB available) plans like an idle one: the plan judges the host, not the free memory of the moment",async()=>{
 expect(reading(stat(9000,420000,300000))!).toBeLessThan(reading(stat(15000,620000,330000))!);
 const busy=await run(16384),idle=await run(16384);
 expect(busy.lines).toBe(idle.lines);
 expect(busy.lines).toContain(expected(16384,"en").plan);
});

test("a 4 GB Mac that cannot run the smallest wave has no wave at all, so every project is light, none is judged by a number, and it asks the fleet for everything, the read refuses",async()=>{
 const result=await run(4096);
 expect(result.code).toBe("unserved");
 expect(expected(4096,"en").local).toBe(0);expect(result.lines).toContain(wantedTextV1("plan","en",{stale:519,taken:0,local:0,wave:0,wanted:519}));expect(result.lines).not.toContain("too heavy");
});
