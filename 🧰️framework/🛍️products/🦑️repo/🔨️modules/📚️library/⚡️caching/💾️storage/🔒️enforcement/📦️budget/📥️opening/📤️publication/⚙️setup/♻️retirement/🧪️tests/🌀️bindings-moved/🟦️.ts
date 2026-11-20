/** 🌀️ A retirement plan that went stale because the dependency bindings moved (a peer or the replica changed a pinned package directory during a read that takes minutes on a slow machine) is planned again within a bound, in both languages, and never fails the setup of the tools for housekeeping. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import aggregateCases from"./🔣️.json"with{type:"json"};
import{renderRepoErrorV1,isRepoErrorV1}from"@semio-tech/process/build/language/error";
import{withProgressSinkV1,type ProgressEventV1}from"../../../../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";
import{DEPENDENCY_STORE_WAIT_V1,type DependencyStoreWaitPortsV1}from"../../../💰️funding/🟦️.ts";
import{javascriptInstallSchemaV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import{materializeSchemaV1,schemaViewV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";
import{refuseCachingV1}from"../../../../../../../../../📖️texts/🟦️.ts";
import fixture from"../🔣️.json"with{type:"json"};
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{retireObsoleteRepositoryDependencyGenerationsV1,bindingsMovedV1,type RetirementOwnersV1}from"../../🟦️.ts";
import{requireRepositoryDependencySelectionCurrentV1}from"../../../../🟦️.ts";

const selectedSchema=materializeSchemaV1(schema,schemaViewV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor))as typeof schema,wire={...selectedSchema,$defs:{...selectedSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}},ajv=new Ajv({strict:true}),acquiredPlan=ajv.compile({...wire,$ref:"#/$defs/AcquiredPlan"}),acquiredSettlement=ajv.compile({...wire,$ref:"#/$defs/AcquiredSettlement"});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish(){},yieldContinuation:async()=>{}}),invocation={}as never;
const parent={version:1,purpose:"mounted-parent-maintenance",wait:false,receiptSha256:"e".repeat(64),directory:{dev:1,ino:2,mode:16877},backing:{dev:1,ino:3,size:1024,allocatedBytes:1024},root:{dev:1,ino:4,mode:16877}};
const row=(prefix:string,status:string,over:Record<string,unknown>={})=>({prefix,generation:null,status,receiptFingerprint:null,rootStamp:"stamp-"+prefix.slice(0,4),roles:[],ownerReferences:[],payloadQualified:false,...over});
const unready=(prefix:string)=>row(prefix,"unready",{roles:["package.json","w"]});
const waits:DependencyStoreWaitPortsV1={now:()=>0,sleep:async()=>{},say:()=>{},limitMilliseconds:DEPENDENCY_STORE_WAIT_V1.limitMilliseconds,observe:()=>({live:[],gone:[]})};
const moved=(planned="aaaaaaaa",current="bbbbbbbb")=>refuseCachingV1("failed","dependency-store.bindings-moved",{planned,current,scopes:591});
type Model={rows:Map<string,any>;plans:number;maintain:string[];moves:(operation:string,prefix:string,call:number)=>unknown};
function store(prefixes:readonly string[],moves:Model["moves"]=()=>undefined):Model{return{rows:new Map([...fixture.plan.candidates.filter(value=>value.status==="active").map(value=>[value.prefix,structuredClone(value)]as[string,any]),...prefixes.map(prefix=>[prefix,unready(prefix)]as[string,any])]),plans:0,maintain:[],moves};}
function owners(model:Model):RetirementOwnersV1{
 return{
  observe:async()=>({parent}),
  plan:async()=>{model.plans++;const plan:any=structuredClone(fixture.plan);plan.candidates=[...model.rows.values()].map(value=>structuredClone(value));plan.generations.names=plan.candidates.map((value:any)=>value.prefix);const wrapped={acquired:true,value:plan};expect(acquiredPlan(wrapped),JSON.stringify(acquiredPlan.errors)).toBe(true);return wrapped;},
  maintain:async(_i,_c,command:any)=>{
   const selection=command.selection;model.maintain.push(command.operation+":"+selection.prefix);const thrown=model.moves(command.operation,selection.prefix,model.maintain.length);if(thrown!==undefined)throw thrown;
   const snapshot={version:1,purpose:"current-selected-dependency-payload",kind:selection.kind,prefix:selection.prefix,generation:null,receiptFingerprint:null,rootStamp:selection.rootStamp,currentBindingsSha256:selection.currentBindingsSha256,inventorySha256:"b".repeat(64),rosterSha256:"c".repeat(64),entries:3,directories:1,files:1,links:1,logicalBytes:"4"};
   if(command.operation==="retire")model.rows.delete(selection.prefix);
   const wrapped={acquired:true,value:{version:1,purpose:"current-selected-dependency-maintenance",operation:command.operation,selected:snapshot,retired:command.operation==="retire",wholeAllocationRetired:false,publicationRetirementDispatches:0,journalSha256:command.artifact.journalSha256,journalBytes:command.artifact.journalBytes}};
   expect(acquiredSettlement(wrapped),JSON.stringify(acquiredSettlement.errors)).toBe(true);return wrapped;
  },
  waits:()=>waits
 };
}
const run=(model:Model)=>{const events:ProgressEventV1[]=[];return withProgressSinkV1(event=>events.push(event),()=>retireObsoleteRepositoryDependencyGenerationsV1(invocation,control(),owners(model))).then(result=>({result,events}));};
const keyed=(events:readonly ProgressEventV1[],key:string)=>events.filter(event=>event.key==="caching.dependency-store."+key);
const A="0".repeat(20),B="1".repeat(20);

test("a selection whose bindings moved is the typed bilingual refusal naming both hashes and the scopes, an unqualified scope stays the plain refusal",()=>{
 const selection={kind:"unready-generation",prefix:A,rootStamp:"stamp-0000",currentBindingsSha256:"a".repeat(64)},request:any={operation:"observe",selection,expected:null},scope={scope:".",members:[],generation:"c".repeat(64),receiptFingerprint:"fp",bindings:[{}]},selected:any={prefix:A,rootStamp:"stamp-0000"};
 expect(()=>requireRepositoryDependencySelectionCurrentV1(request,{version:1,sha256:"a".repeat(64),scopes:[scope]}as never,selected,new Map([[".",1]]))).not.toThrow();
 let caught:unknown;try{requireRepositoryDependencySelectionCurrentV1(request,{version:1,sha256:"b".repeat(64),scopes:[scope]}as never,selected,new Map([[".",1]]));}catch(error){caught=error;}
 expect(isRepoErrorV1(caught)).toBe(true);const error=caught as never as{key:string;code:string;values:Record<string,unknown>};
 expect(error.key).toBe("caching.dependency-store.bindings-moved");expect(error.code).toBe("failed");expect(error.values).toEqual({planned:"aaaaaaaa",current:"bbbbbbbb",scopes:1});
 const en=renderRepoErrorV1(error as never,"en"),de=renderRepoErrorV1(error as never,"de");expect(en).toContain("aaaaaaaa");expect(en).toContain("bbbbbbbb");expect(de).toContain("aaaaaaaa");expect(de).toContain("bbbbbbbb");expect(en).not.toBe(de);
 for(const scopes of[[],[{...scope,generation:null}],[{...scope,receiptFingerprint:null}],[{...scope,bindings:[]}]])expect(()=>requireRepositoryDependencySelectionCurrentV1(request,{version:1,sha256:"a".repeat(64),scopes}as never,selected,new Map([[".",1]]))).toThrow("must be qualified before inactivity");
});

test("a moved bindings refusal is found at the top and inside an aggregate, every other failure is not",()=>{
 const refusal=moved();expect(bindingsMovedV1(refusal)).toBe(refusal);expect(bindingsMovedV1(new AggregateError([Error("x"),refusal],"two"))).toBeNull();expect(bindingsMovedV1(new AggregateError([refusal,refusal]))).toBe(refusal);expect(bindingsMovedV1(new AggregateError([Error("x")],"one"))).toBeNull();
 expect(bindingsMovedV1(Error("Complete current Dependencies bindings changed"))).toBeNull();expect(bindingsMovedV1(refuseCachingV1("failed","dependency-store.retirement-plan-incomplete",{reason:"r",bytes:1,violation:"v"}))).toBeNull();expect(bindingsMovedV1(undefined)).toBeNull();
});

test("a plan that went stale twice is planned again and the generation is retired by the third plan, with a bilingual line per replan",async()=>{
 const model=store([A],(operation,prefix,call)=>operation==="observe"&&call<=2?moved("a1a1a1a1","b2b2b2b2"):undefined),{result,events}=await run(model);
 expect(result).toEqual([A]);expect(model.rows.has(A)).toBe(false);expect(model.maintain).toEqual(["observe:"+A,"observe:"+A,"observe:"+A,"retire:"+A]);expect(model.plans).toBe(4);
 const lines=keyed(events,"bindings-replanned");expect(lines).toHaveLength(2);expect(lines[0]!.level).toBe("warn");expect(lines.map(line=>line.values.attempt)).toEqual([2,3]);expect(lines[0]!.values).toMatchObject({planned:"a1a1a1a1",current:"b2b2b2b2",limit:4});
 expect(keyed(events,"retirement-moved")).toHaveLength(0);
});

test("bindings that move in every plan end the retirement with a warning and the rest for the next setup, never in a failure",async()=>{
 const model=store([A,B],operation=>operation==="observe"?moved():undefined),{result,events}=await run(model);
 expect(result).toEqual([]);expect(model.rows.size).toBeGreaterThanOrEqual(3);expect(model.plans).toBe(4);
 expect(keyed(events,"bindings-replanned")).toHaveLength(3);const end=keyed(events,"retirement-moved");expect(end).toHaveLength(1);expect(end[0]!.level).toBe("warn");expect(end[0]!.values).toEqual({limit:3,retired:0});
 const again=await run({...model,moves:()=>undefined,maintain:[],plans:0});expect(again.result).toEqual([A,B]);
});

test("the bound counts plans in a row: a retirement in between starts the count again",async()=>{
 const model=store([A,B],(operation,prefix,call)=>operation==="observe"&&[1,2,5,6].includes(call)?moved():undefined),{result,events}=await run(model);
 expect(result).toEqual([A,B]);expect(keyed(events,"bindings-replanned")).toHaveLength(4);expect(keyed(events,"retirement-moved")).toHaveLength(0);
});

test("a terminal observation failure retains the original aggregate",async()=>{
 const failure=new AggregateError([Error("terminal observation failed"),moved()],"Selected Dependencies body and terminal observation failed");
 const model=store([A],()=>failure);
 await expect(run(model)).rejects.toBe(failure);expect(model.rows.has(A)).toBe(true);
});

test("every other failure of a selected retirement still fails the setup with its own error",async()=>{
 const model=store([A],()=>Error("Complete current Dependencies bindings changed"));
 await expect(run(model)).rejects.toThrow("Complete current Dependencies bindings changed");expect(model.rows.has(A)).toBe(true);
});

test("only aggregates containing exclusively moved bindings qualify for another plan",()=>{
 const database=new Database(":memory:");
 try{
  const qualifies=database.query("select count(*) > 0 and count(*) = count(case when value = 'moved' then 1 end) as retry from json_each(?)");
  for(const {causes,retry} of aggregateCases){
   const error=new AggregateError(causes.map(cause=>cause==="moved"?moved():Error("terminal")));
   expect(qualifies.get(JSON.stringify(causes))).toEqual({retry:Number(retry)});
   expect(bindingsMovedV1(error)!==null).toBe(retry);
  }
 }finally{database.close();}
});
