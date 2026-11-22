/** 📥️ Contract: `recoverNeedV1` frees the need of a waiting stage in registry priority order, judging by the need alone on any volume size, reads the volume after each collector, isolates failures, honours cancellation and answers the largest consumers with their commands when nothing collectable remains; lines come in one language or both. */
import{expect,test}from"bun:test";
import{DISK_WRITERS_V1,diskReliefV1,diskTextV1,recoverNeedV1,sweepDiskV1,type DiskWriterV1}from"../../🟦️.ts";
import{realDiskWritersV1}from"../../🔌️real/🟦️.ts";

const MIB=1048576,GIB=1024*MIB;
const rowOf=(id:string)=>DISK_WRITERS_V1.find(row=>row.id===id)!;
function volume(totalGiB:number,freeMiB:number){return{total:totalGiB*GIB,free:freeMiB*MIB};}
const writer=(v:{total:number;free:number},id:string,holds:number,gives:number,log:string[],fail=false):DiskWriterV1=>({row:rowOf(id),root:"/w/"+id,measure:()=>holds*MIB,collect:async()=>{log.push(id);if(fail)throw Error("nope");v.free+=gives*MIB;return gives*MIB;}});
const input=(v:{total:number;free:number},writers:readonly DiskWriterV1[],needMiB:number,extra:Partial<Parameters<typeof recoverNeedV1>[0]>={})=>({facts:()=>({partition:"vol",totalBytes:v.total,freeBytes:v.free}),needBytes:needMiB*MIB,writers,language:"en" as const,...extra});

test("collectors run by priority until the need fits and the rest never run, on a 1 GiB and on a 4 TiB volume alike",async()=>{
 for(const total of[1,238,4096]){
  const v=volume(total,2993),log:string[]=[];
  const outcome=await recoverNeedV1(input(v,[writer(v,"image-slack",9000,9000,log),writer(v,"cas-lru",900,600,log),writer(v,"stale-waves",800,800,log),writer(v,"replica-lost",100,100,log)],4096));
  expect(outcome.verdict,String(total)).toBe("admit");expect(log,String(total)).toEqual(["stale-waves","cas-lru"]);
  expect(outcome.steps.map(step=>step.id)).toEqual(["stale-waves","cas-lru"]);expect(outcome.steps.map(step=>step.freedBytes)).toEqual([800*MIB,600*MIB]);
 }
});

test("a volume that already holds the need collects nothing, an unreadable volume is never a refusal",async()=>{
 const v=volume(238,5000),log:string[]=[];
 expect((await recoverNeedV1(input(v,[writer(v,"cas-lru",900,600,log)],4096))).verdict).toBe("admit");expect(log).toEqual([]);
 expect((await recoverNeedV1({...input(v,[],4096),facts:()=>undefined})).verdict).toBe("fixed-shares");
});

test("nothing collectable: refused with the largest consumers first, each with the command that frees it, the image slack and the store named even when small",async()=>{
 const v=volume(238,2993),log:string[]=[];
 const outcome=await recoverNeedV1(input(v,[writer(v,"stale-waves",3,0,log),writer(v,"cas-lru",5000,0,log),writer(v,"leaked-staging",800,0,log),writer(v,"read-verdicts",700,0,log),writer(v,"dependency-generations",600,0,log),writer(v,"compile-caches",500,0,log),writer(v,"image-slack",50,0,log),writer(v,"replica-store",20,0,log)],4096));
 expect(outcome.verdict).toBe("refuse");
 expect(outcome.consumers.slice(0,5).map(item=>item.id)).toEqual(["cas-lru","leaked-staging","read-verdicts","dependency-generations","compile-caches"]);
 expect(outcome.consumers.map(item=>item.id)).toContain("image-slack");expect(outcome.consumers.map(item=>item.id)).toContain("replica-store");
 for(const item of outcome.consumers){expect(item.command.length).toBeGreaterThan(0);expect(item.root.startsWith("/w/")).toBe(true);}
 expect(outcome.consumers[0]!.command).toBe(rowOf("cas-lru").command);
});

test("a failing collector does not stop the next, cancellation stops between collectors, a collector over its budget is cut",async()=>{
 const v=volume(238,2993),log:string[]=[],lines:string[]=[];
 const outcome=await recoverNeedV1(input(v,[writer(v,"stale-waves",1,0,log,true),writer(v,"cas-lru",2000,2000,log)],4096,{report:line=>lines.push(line)}));
 expect(outcome.verdict).toBe("admit");expect(log).toEqual(["stale-waves","cas-lru"]);expect(outcome.steps[0]!.failure).toBe("nope");expect(lines.some(line=>line.includes("nope"))).toBe(true);
 const controller=new AbortController(),w=volume(238,2993),seen:string[]=[];
 const first:DiskWriterV1={...writer(w,"stale-waves",1,10,seen),collect:async()=>{seen.push("stale-waves");controller.abort();return 0;}};
 await expect(recoverNeedV1(input(w,[first,writer(w,"cas-lru",2000,2000,seen)],4096,{signal:controller.signal}))).rejects.toThrow();expect(seen).toEqual(["stale-waves"]);
 const slow=volume(238,2993),slowLog:string[]=[];
 const hang:DiskWriterV1={...writer(slow,"stale-waves",1,0,slowLog),collect:({signal})=>new Promise((_,reject)=>signal?.addEventListener("abort",()=>reject(Error("cut")),{once:true}))};
 const cut=await recoverNeedV1(input(slow,[hang,writer(slow,"cas-lru",2000,2000,slowLog)],4096,{budgetMilliseconds:30}));
 expect(cut.verdict).toBe("admit");expect(cut.steps[0]!.failure).toBe("cut");expect(slowLog).toEqual(["cas-lru"]);
});

test("progress lines come in English, German or both joined by a slash, and none keeps a placeholder",async()=>{
 for(const language of["en","de","both"] as const){
  const v=volume(238,2993),lines:string[]=[];
  await recoverNeedV1(input(v,[writer(v,"stale-waves",800,800,[]),writer(v,"cas-lru",900,900,[])],4096,{language,report:line=>lines.push(line)}));
  expect(lines.length).toBeGreaterThanOrEqual(3);
  for(const line of lines){expect(line).not.toContain("{");expect(line.includes(" / ")).toBe(language==="both");}
 }
 expect(diskTextV1("recovery.step","en",{id:"x",freed:"1 GiB",free:"2 GiB"})).not.toBe(diskTextV1("recovery.step","de",{id:"x",freed:"1 GiB",free:"2 GiB"}));
});

/** 🪟️ Contract: on a platform whose partitions are plain directories the writers that relieve the image give host bytes directly, so a stage that needs space frees the superseded dependency generations, tool generations and closure layers (through the ledger collector, told how many bytes to free) instead of finding nothing eligible; on an image platform they stay out of the host-relief order. */
test("directory-backed hosts collect the image-relief writers, asking the ledger for the shortfall, and image hosts do not",async()=>{
 for(const row of DISK_WRITERS_V1){expect(diskReliefV1(row,true),row.id).toBe(row.relief);expect(diskReliefV1(row,false),row.id).toBe(row.relief==="image"?"host":row.relief);}
 const context=(imageBacked:boolean,calls:number[])=>({workspace:"/ws",home:"/home",tmp:"/tmp",uid:1,environment:{},dryRun:true,imageBacked,ledger:async(_signal?:AbortSignal,need?:number)=>{calls.push(need??-1);}});
 for(const imageBacked of[false,true]){
  const calls:number[]=[],writers=realDiskWritersV1(context(imageBacked,calls)),v=volume(256,4937),outcome=await recoverNeedV1({facts:()=>({partition:"C:",totalBytes:v.total,freeBytes:v.free}),needBytes:6144*MIB,writers,language:"en",budgetMilliseconds:2000});
  const collectable=writers.filter(item=>item.row.relief==="host"&&item.collect!==undefined&&item.row.collector==="ledger").map(item=>item.row.id).sort();
  expect(collectable.includes("dependency-generations"),String(imageBacked)).toBe(!imageBacked);expect(collectable.includes("compile-caches")).toBe(true);
  expect(calls.length,String(imageBacked)).toBe(1);expect(calls[0],String(imageBacked)).toBe(6144*MIB-4937*MIB);expect(outcome.verdict).toBe("refuse");
 }
});

/** 🧹️ Contract: a stage that failed with "no space left" inside an image or a quota sweeps every collector once, the image-relief writers included, whatever the host volume says in between, and names the consumers when nothing ran. */
test("a sweep runs every collector once even when the host volume gains nothing",async()=>{
 const v=volume(256,50000),log:string[]=[],image=(id:string):DiskWriterV1=>({row:rowOf(id),root:"/w/"+id,measure:()=>10*MIB,collect:async()=>{log.push(id);return 0;}});
 const outcome=await sweepDiskV1({facts:()=>({partition:"vol",totalBytes:v.total,freeBytes:v.free}),needBytes:4096*MIB,writers:[image("stale-waves"),image("tool-generations"),image("dependency-generations"),image("cas-lru")],language:"en"});
 expect(log).toEqual(["tool-generations","stale-waves","dependency-generations","cas-lru"]);expect(outcome.verdict).toBe("admit");expect(outcome.steps.length).toBe(4);
 const none=await sweepDiskV1({facts:()=>({partition:"vol",totalBytes:v.total,freeBytes:v.free}),needBytes:4096*MIB,writers:[],language:"en"});expect(none.verdict).toBe("refuse");
});
