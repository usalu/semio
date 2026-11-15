/** 🔁️ Obsolete dependency retirement against a store in any state: a busy store is waited for with a fresh parent per attempt, an interrupted or half-retired store is resumed, and an unusable answer ends in a typed bilingual refusal that names the state. */
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import Ajv from"ajv";
import{renderRepoErrorV1,isRepoErrorV1}from"@semio-tech/process/build/language/error";
import{withProgressSinkV1,type ProgressEventV1}from"../../../../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";
import{DEPENDENCY_STORE_WAIT_V1,type DependencyStoreHoldersV1,type DependencyStoreWaitPortsV1}from"../../../💰️funding/🟦️.ts";
import{javascriptInstallSchemaV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import{materializeSchemaV1,schemaViewV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";
import fixture from"../🔣️.json"with{type:"json"};
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{retireObsoleteRepositoryDependencyGenerationsV1,type RetirementOwnersV1}from"../../🟦️.ts";

const selectedSchema=materializeSchemaV1(schema,schemaViewV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor))as typeof schema,wire={...selectedSchema,$defs:{...selectedSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}},ajv=new Ajv({strict:true}),acquiredPlan=ajv.compile({...wire,$ref:"#/$defs/AcquiredPlan"}),acquiredSettlement=ajv.compile({...wire,$ref:"#/$defs/AcquiredSettlement"});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish(){},yieldContinuation:async()=>{}}),invocation={}as never;
const parent={version:1,purpose:"mounted-parent-maintenance",wait:false,receiptSha256:"e".repeat(64),directory:{dev:1,ino:2,mode:16877},backing:{dev:1,ino:3,size:1024,allocatedBytes:1024},root:{dev:1,ino:4,mode:16877}};
const row=(prefix:string,status:string,over:Record<string,unknown>={})=>({prefix,generation:null,status,receiptFingerprint:null,rootStamp:"stamp-"+prefix.slice(0,4),roles:[],ownerReferences:[],payloadQualified:false,...over});
const obsolete={capsule:()=>row(".p00000000","retained-unready",{generation:"d".repeat(64),receiptFingerprint:"fp-capsule",roles:["i","p","r"]}),metadata:()=>row(".p00000001","unready-metadata",{roles:["r"]}),unready:()=>row("0".repeat(20),"unready",{roles:["package.json","w"]}),empty:()=>row("f".repeat(20),"unqualified")};
function planOf(candidates:readonly Record<string,unknown>[]){const plan:any=structuredClone(fixture.plan);plan.candidates=structuredClone(candidates);plan.generations.names=candidates.map(value=>value.prefix);return plan;}
type Model={rows:Map<string,any>;calls:{observe:number;plan:number;maintain:string[]};busyPlan:number;busyMaintain:number;failRetire?:(prefix:string,count:number)=>void;retiredCount:number};
function store(rows:readonly any[],over:Partial<Model>={}):Model{return{rows:new Map(rows.map(value=>[value.prefix,structuredClone(value)])),calls:{observe:0,plan:0,maintain:[]},busyPlan:0,busyMaintain:0,retiredCount:0,...over};}
const full=()=>[...structuredClone(fixture.plan.candidates),obsolete.capsule(),obsolete.metadata(),obsolete.unready(),obsolete.empty()];
const kindOf=(status:string)=>({inactive:"inactive-generation",unready:"unready-generation","retained-unready":"retained-unready","unready-metadata":"unready-metadata",unqualified:"empty-generation"}as Record<string,string>)[status]!;
function owners(model:Model,waits:DependencyStoreWaitPortsV1):RetirementOwnersV1{
 return{
  observe:async()=>{model.calls.observe++;return{parent};},
  plan:async()=>{model.calls.plan++;if(model.busyPlan>0){model.busyPlan--;return{acquired:false};}const plan=planOf([...model.rows.values()]),wrapped={acquired:true,value:plan};expect(acquiredPlan(wrapped),JSON.stringify(acquiredPlan.errors)).toBe(true);return wrapped;},
  maintain:async(_i,_c,command:any)=>{
   model.calls.maintain.push(command.operation+":"+command.selection.prefix);if(model.busyMaintain>0){model.busyMaintain--;return{acquired:false};}
   const found=model.rows.get(command.selection.prefix);if(!found)throw Error("Selected dependency prefix is gone");
   const selection=command.selection,empty=selection.kind==="empty-generation",snapshot={version:1,purpose:"current-selected-dependency-payload",kind:selection.kind,prefix:selection.prefix,generation:selection.generation??null,receiptFingerprint:selection.receiptFingerprint??null,rootStamp:selection.rootStamp,currentBindingsSha256:selection.currentBindingsSha256,inventorySha256:"b".repeat(64),rosterSha256:"c".repeat(64),entries:empty?0:3,directories:empty?0:1,files:empty?0:1,links:empty?0:1,logicalBytes:empty?"0":"4"};
   if(command.operation==="retire"){model.failRetire?.(selection.prefix,model.retiredCount);model.rows.delete(selection.prefix);model.retiredCount++;}
   const wrapped={acquired:true,value:{version:1,purpose:"current-selected-dependency-maintenance",operation:command.operation,selected:snapshot,retired:command.operation==="retire",wholeAllocationRetired:false,publicationRetirementDispatches:0,journalSha256:command.artifact.journalSha256,journalBytes:command.artifact.journalBytes}};
   expect(acquiredSettlement(wrapped),JSON.stringify(acquiredSettlement.errors)).toBe(true);return wrapped;
  },
  waits:()=>waits
 };
}
function clock(limitMilliseconds=DEPENDENCY_STORE_WAIT_V1.limitMilliseconds,rows:()=>DependencyStoreHoldersV1=()=>({live:[],gone:[]})){let time=0;const sleeps:number[]=[],says:number[]=[],ports:DependencyStoreWaitPortsV1={now:()=>time,sleep:async milliseconds=>{sleeps.push(milliseconds);time+=milliseconds;},say:seconds=>{says.push(seconds);},limitMilliseconds,observe:rows};return{ports,sleeps,says,at:()=>time};}
const expectedOrder=(rows:readonly any[])=>rows.filter(value=>value.status!=="active").map(value=>value.prefix).sort();
async function caught(run:()=>Promise<unknown>):Promise<{key:string;code:string;values:Record<string,string|number>}>{try{await run();}catch(error){expect(isRepoErrorV1(error)).toBe(true);return error as never;}throw Error("a refusal was expected");}

test("a fresh store with only the current generation retires nothing and issues no owner command",async()=>{const model=store(fixture.plan.candidates.filter(value=>value.status==="active")),fake=clock();expect(await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,fake.ports))).toEqual([]);expect(model.calls.plan).toBe(1);expect(model.calls.maintain).toEqual([]);expect(fake.sleeps).toEqual([]);});

test("a half-funded store with capsule, metadata, never-ready, empty and inactive leftovers is retired completely in prefix order",async()=>{const rows=full(),model=store(rows),fake=clock(),result=await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,fake.ports));expect(result).toEqual(expectedOrder(rows));expect([...model.rows.keys()]).toEqual(["cccccccccccccccccccc"]);expect(model.calls.maintain.filter(value=>value.startsWith("retire:"))).toHaveLength(result.length);expect(model.calls.maintain.filter(value=>value.startsWith("observe:"))).toHaveLength(result.length);});

test("a busy plan is waited for and every attempt observes a fresh parent, it never becomes an error",async()=>{const model=store(full(),{busyPlan:5}),fake=clock(),result=await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,fake.ports));expect(result).toEqual(expectedOrder(full()));expect(fake.sleeps).toHaveLength(5);expect(model.calls.plan).toBe(6+result.length);expect(model.calls.observe).toBe(model.calls.plan+2*result.length);});

test("a store busy during an observe or a retire is waited for and each prefix is retired exactly once",async()=>{const model=store(full(),{busyMaintain:7}),fake=clock(),result=await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,fake.ports));expect(result).toEqual(expectedOrder(full()));expect(new Set(result).size).toBe(result.length);expect(fake.sleeps).toHaveLength(7);expect(model.calls.maintain.filter(value=>value.startsWith("retire:")).length).toBeGreaterThanOrEqual(result.length);expect(model.rows.size).toBe(1);});

test("the wait says what it waits for: the progress line names the holder and a retirement line is said for every generation",async()=>{const events:ProgressEventV1[]=[],holder={pid:4242,command:"bun script.ts repo run workspace:setup",sinceMs:Date.UTC(2026,10,15,1,0,0),mode:"shared"as const,ancestor:false},model=store(full(),{busyPlan:3}),fake=clock(DEPENDENCY_STORE_WAIT_V1.limitMilliseconds,()=>({live:[holder],gone:[]})),ports={...fake.ports,say:(seconds:number,limit:number,named?:typeof holder)=>{expect(named).toBe(holder);fake.says.push(seconds);}};const result=await withProgressSinkV1(event=>events.push(event),()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,ports)));expect(fake.says.length).toBeGreaterThanOrEqual(1);const retiring=events.filter(event=>event.key==="caching.dependency-store.retiring");expect(retiring).toHaveLength(result.length);expect(retiring[0]!.values.prefix).toBe(result[0]);expect(retiring[1]!.values.retired).toBe(1);});

test("a store that stays busy past the retirement bound defers the rest with a bilingual warning instead of failing or hanging",async()=>{
 const events:ProgressEventV1[]=[],model=store(full()),fake=clock(60000);let calls=0;const base=owners(model,fake.ports),limited:RetirementOwnersV1={...base,plan:async(...args)=>{if(++calls>3)return{acquired:false};return base.plan(...args);}};
 const result=await withProgressSinkV1(event=>events.push(event),()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),limited));
 expect(result).toEqual(expectedOrder(full()).slice(0,3));expect(fake.at()).toBeLessThanOrEqual(60000);
 const deferred=events.filter(event=>event.key==="caching.dependency-store.retirement-deferred");expect(deferred).toHaveLength(1);expect(deferred[0]!.level).toBe("warn");expect(deferred[0]!.values.retired).toBe(3);expect(Number(deferred[0]!.values.seconds)).toBeGreaterThanOrEqual(1);
 expect(model.rows.size).toBe(full().length-3);
 const again=await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,clock().ports));expect(again).toEqual(expectedOrder(full()).slice(3));expect([...model.rows.keys()]).toEqual(["cccccccccccccccccccc"]);
});

test("a holder that is an ancestor is never waited for: after the grace the refusal names it, in both languages",async()=>{const holder={pid:4242,command:"bun script.ts repo run workspace:setup",sinceMs:Date.UTC(2026,10,15,1,0,0),mode:"shared"as const,ancestor:true},fake=clock(DEPENDENCY_STORE_WAIT_V1.limitMilliseconds,()=>({live:[holder],gone:[]})),error=await caught(()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(store(full(),{busyPlan:1e9}),fake.ports)));expect(error.key).toBe("caching.dependency-store.ancestor-holds");expect(error.values.pid).toBe(4242);expect(fake.at()).toBeGreaterThanOrEqual(DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds);expect(renderRepoErrorV1(error as never,"en")).toContain("4242");expect(renderRepoErrorV1(error as never,"de")).toContain("4242");});

test("a run interrupted between the observe and the retire, and one interrupted after a retire, resume on the next run without retiring anything twice",async()=>{
 const rows=full(),model=store(rows),first=new AbortController(),done:string[]=[];
 model.failRetire=(prefix,count)=>{if(count===2){first.abort(new Error("cancelled by SIGINT"));throw first.signal.reason;}};
 await expect(retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(first.signal),owners(model,clock().ports))).rejects.toThrow("cancelled by SIGINT");
 const left=[...model.rows.keys()].filter(prefix=>prefix!=="cccccccccccccccccccc");expect(left.length).toBe(rows.length-1-2);
 model.failRetire=undefined;const again=await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model,clock().ports));done.push(...again);
 expect(again).toEqual(left.sort());expect([...model.rows.keys()]).toEqual(["cccccccccccccccccccc"]);
 const cold=new AbortController();cold.abort(new Error("cancelled before start"));await expect(retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(cold.signal),owners(store(full()),clock().ports))).rejects.toThrow("cancelled before start");
});

test("an answer that is not a complete acquired plan ends in a typed bilingual refusal naming the state, never in a bare assertion",async()=>{
 const cases:Array<[string,(model:Model)=>RetirementOwnersV1,string]>=[
  ["not-acquired",model=>({...owners(model,clock().ports),plan:async()=>({acquired:false,callback:true})}),"not-acquired"],
  ["malformed",model=>({...owners(model,clock().ports),plan:async()=>({acquired:true,value:{...planOf([...model.rows.values()]),version:2}})}),"malformed"],
  ["oversized",model=>({...owners(model,clock().ports),plan:async()=>({acquired:true,value:{...planOf([]),padding:"x".repeat(70000000)}})}),"oversized"],
  ["empty-answer",model=>({...owners(model,clock().ports),plan:async()=>undefined}),"malformed"]
 ];
 for(const[name,make,reason]of cases){const error=await caught(()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),make(store(full()))));expect(error.key,name).toBe("caching.dependency-store.retirement-plan-incomplete");expect(error.code,name).toBe("failed");expect(error.values.reason,name).toBe(reason);expect(typeof error.values.bytes).toBe("number");const en=renderRepoErrorV1(error as never,"en"),de=renderRepoErrorV1(error as never,"de");expect(en).toContain(reason);expect(de).toContain(reason);expect(en).not.toContain("Complete acquired");expect(de).not.toBe(en);}
});

test("an unusable settlement of one generation ends in a typed bilingual refusal that names the operation and the generation",async()=>{const model=store(full()),base=owners(model,clock().ports),error=await caught(()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),{...base,maintain:async()=>({acquired:true,value:{operation:"observe"}})}));expect(error.key).toBe("caching.dependency-store.retirement-settlement-incomplete");expect(error.code).toBe("failed");expect(error.values.operation).toBe("observe");expect(error.values.prefix).toBe(expectedOrder(full())[0]);expect(renderRepoErrorV1(error as never,"de")).toContain(String(error.values.prefix));});

test("the production owners are the genuine ones with a bounded retirement wait, and the bare assertion is gone",()=>{const source=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8");expect(source).toContain("observe:observeRepositoryDependencyMaintenanceParentV1,plan:planSelectedRepositoryDependencyGenerationsV1,maintain:maintainSelectedRepositoryDependencyGenerationV1");expect(source).toContain("limitMilliseconds:retirementProtocol.maximumBusyWaitMilliseconds");expect(source).not.toContain("Complete acquired dependency retirement plan required");expect(source).not.toContain("Acquired closed dependency retirement settlement required");expect(selectedSchema.$defs.Protocol.const.maximumBusyWaitMilliseconds).toBeLessThanOrEqual(300000);expect(selectedSchema.$defs.Protocol.const.maximumBusyWaitMilliseconds).toBeLessThan(DEPENDENCY_STORE_WAIT_V1.limitMilliseconds);});

test("the setup says that a funding installs every scope before the long silent producer step",()=>{const setup=readFileSync(join(import.meta.dir,"..","..","..","🟦️.ts"),"utf8"),say=setup.indexOf('sayCachingV1("dependency-store.preparing")'),wait=setup.indexOf("retryWhileDependencyStoreBusyV1(control,async()=>{const parent=await observeRepositoryDependencyMaintenanceParentV1");expect(say).toBeGreaterThan(0);expect(say).toBeLessThan(wait);});
