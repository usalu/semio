import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fleetReadStaleGenerationV1}from"../../../🤝️prefetch/🟦️.ts";
import{fleetAdmittedMegabytesV1,fleetGrantMegabytesV1,inferableProjectsV1,wantedPolicyV1,wantedTextV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{describeHeavyV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,soloGrantV1}from"../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

/** 🌍️ Contract: the Linux worker of the fleet (16 GB declared as 15848 MB, asked from a slice booked with 1 GB; the host is injected and every number derives from the policy tables, never from the live machine) takes the decision by what it can give, not by the booking nor by the free memory of the moment: it infers a wave of unknown projects itself, names only the heavy ones (the workspace root; the saturated seed of s-dev-composition is attempted, never refused by a number), and says in plain text what it can infer; a machine declared as 4 GB cannot run a wave and asks the fleet for everything. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const request=(count:number,extra:readonly string[]=[],roots:Record<string,string>={})=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(count:number,environment:Record<string,string>,extra:readonly string[]=[],roots:Record<string,string>={}){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-booking-"));scratch.push(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;
 try{await fleetReadStaleGenerationV1({root,request:request(count,extra,roots),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,...environment},hostMegabytes:()=>Number(environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES)});return{lines,error:undefined};}
 catch(error){return{lines,error:error as {name:string;key:string;code:string}};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}
const LINUX={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"15848"};
const expected=(host:number,stale:number)=>{
 const admitted=fleetAdmittedMegabytesV1({hostMegabytes:host}),grant=ownerGrantV1(fleetGrantMegabytesV1({hostMegabytes:host}),wantedPolicyV1).managedMegabytes*ownerCostPolicyV1.footprintFactor,wave=inferableProjectsV1(admitted).waveProjects,local=wave*wantedPolicyV1.maximumLocalWaves,solo=soloGrantV1({hostMegabytes:host},wantedPolicyV1).managedMegabytes*ownerCostPolicyV1.footprintFactor;
 const named=describeHeavyV1([{name:"workspace",megabytes:ownerCostPolicyV1.seeds.workspace,source:"seed"}],"en");
 return{grant,wave,local,solo,named,plan:ownerCostTextV1("plan","en",{stale,taken:0,heavy:1,named,grant,local,wave,wanted:stale}),soloLine:ownerCostTextV1("solo","en",{count:1,named,grant:solo}),light:wantedTextV1("plan","en",{stale,taken:0,local,wave,wanted:stale})};
};

test("517 stale projects on the Linux worker: a small booking of the asking slice no longer turns them all into refused ones; the machine can infer a wave of 48 four times itself, and the workspace runs alone after the others",async()=>{
 const booked=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"},["workspace","@semio-tech/s-dev-composition"],{workspace:"."});
 const text=booked.lines.join("");
 expect(booked.error).toBeUndefined();
 const want=expected(15848,519);
 expect(want.wave).toBeGreaterThan(0);expect(want.solo).toBeGreaterThan(want.grant);
 expect(text).toContain(want.plan);expect(text).toContain(want.soloLine);expect(text).not.toContain("@semio-tech/s-dev-composition (about");
});

test("without a measured heavy project 517 stale projects are light: the plan says the machine can infer 192 of them and asks the fleet for the rest",async()=>{
 const result=await run(517,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"});
 const text=result.lines.join("");
 expect(result.error).toBeUndefined();expect(text).toContain(expected(15848,517).light);expect(text).not.toContain("too heavy");
});

test("a few unknown projects stay local on that machine whatever the booking, and a machine declared too small for a wave asks for everything",async()=>{
 expect(await run(40,{...LINUX,SEMIO_GRAPH_MEMORY_MB:"1024"})).toEqual({lines:[],error:undefined});
 const starved=await run(40,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"4096"});
 expect(starved.error?.code).toBe("unserved");expect(starved.lines.join("")).toContain(wantedTextV1("plan","en",{stale:40,taken:0,local:0,wave:0,wanted:40}));
});
