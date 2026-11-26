import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{AsyncLocalStorage}from"node:async_hooks";
import{loadProductFunctionsV1}from"@semio-tech/process/testing/extracted-function";
import{awaitPartitionSlotV1,partitionFailureTextV1,partitionHasRoomV1,partitionOperationSlotsV1,partitionRefusalV1,partitionTextsV1,type PartitionSlotPortsV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-opening",maximumElapsedMilliseconds:15000});

const opening=new URL("../../🟦️.ts",import.meta.url);
const control=(signal:AbortSignal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}});
const names=["orderedPartitionFailures","withRepositoryPartitionOperationV1"];

async function operations(scope:Record<string,unknown>,owners:WeakMap<object,unknown>){
 const frames=new AsyncLocalStorage<unknown>(),ports:Record<string,unknown>={partitionOwners:owners,partitionFrames:frames,requireIssuedResourceControlRefinementV1:()=>{},withRequestedIssuedResourceControlV1:(_c:unknown,_w:unknown,body:(actual:unknown)=>unknown)=>body(scope.control),awaitPartitionSlotV1,partitionFailureTextV1,partitionHasRoomV1,partitionOperationSlotsV1,partitionRefusalV1};
 const api=await loadProductFunctionsV1(opening,names,ports) as Record<string,Function>;
 return api.withRepositoryPartitionOperationV1!;
}
const newScope=(over:Record<string,unknown>={})=>{const c=control(),scope:Record<string,unknown>={id:"graph",image:{},leases:{},control:c,live:true,accepting:true,failed:false,sequence:0,pending:new Set(),outcomes:new Set(),roots:0,waiters:new Set(),...over},facade={};const owners=new WeakMap<object,unknown>();owners.set(facade,scope);return{scope,facade,owners,c};};

test("the m12 sequence: forty concurrent submissions wait for a slot, never exceed thirty-two and none is refused",async()=>{
 const{scope,facade,owners,c}=newScope(),operate=await operations(scope,owners);let running=0,peak=0;const releases:Array<()=>void>=[];
 const submitted=Array.from({length:40},(_,index)=>operate(facade,c,async()=>{running++;peak=Math.max(peak,running);await new Promise<void>(done=>releases.push(done));running--;return index;}));
 await Bun.sleep(20);expect(running).toBe(32);expect(scope.roots).toBe(32);expect((scope.waiters as Set<unknown>).size).toBe(8);
 while(releases.length){releases.splice(0).forEach(release=>release());await Bun.sleep(5);}
 expect(await Promise.all(submitted)).toEqual(Array.from({length:40},(_,index)=>index));expect(peak).toBe(partitionOperationSlotsV1);expect(scope.roots).toBe(0);expect((scope.waiters as Set<unknown>).size).toBe(0);
});

test("a nested operation never waits for the slot its own parent holds",async()=>{
 const{scope,facade,owners,c}=newScope(),operate=await operations(scope,owners);let inner=0;
 await Promise.all(Array.from({length:partitionOperationSlotsV1},()=>operate(facade,c,async()=>{await Bun.sleep(5);await operate(facade,c,async()=>{inner++;});})));
 expect(inner).toBe(partitionOperationSlotsV1);
});

test("a waiting submission is cancellable and ends with the control's own refusal",async()=>{
 const{scope,facade,owners,c}=newScope(),operate=await operations(scope,owners),abort=new AbortController(),holders:Array<()=>void>=[];
 const busy=Array.from({length:partitionOperationSlotsV1},()=>operate(facade,c,()=>new Promise<void>(done=>holders.push(done))));
 const waiting=operate(facade,control(abort.signal),async()=>1);await Bun.sleep(10);abort.abort();await expect(waiting).rejects.toThrow();expect((scope.waiters as Set<unknown>).size).toBe(0);
 holders.forEach(release=>release());await Promise.all(busy);
});

test("each of the distinct causes is refused with its own English and German text, never the old catch-all",async()=>{
 const text=(error:unknown)=>(error as{texts:{en:string;de:string}}).texts;
 const unissued=newScope(),op1=await operations(unissued.scope,unissued.owners);let error:unknown;try{op1({...unissued.facade},unissued.c,async()=>1);}catch(caught){error=caught;}
 expect((error as{kind:string}).kind).toBe("notIssued");expect(text(error).en).toContain("not issued");expect(text(error).de).toContain("nicht von seinem Cache-Eigner ausgegeben");
 const closed=newScope({live:false}),op2=await operations(closed.scope,closed.owners);try{op2(closed.facade,closed.c,async()=>1);error=undefined;}catch(caught){error=caught;}
 expect((error as{kind:string}).kind).toBe("closed");expect(text(error).en).toContain("already closed");expect(text(error).de).toContain("bereits geschlossen");
 const failed=newScope(),op3=await operations(failed.scope,failed.owners),original=Error("the first disk failure");await expect(op3(failed.facade,failed.c,async()=>{throw original;})).rejects.toBe(original);
 try{op3(failed.facade,failed.c,async()=>1);error=undefined;}catch(caught){error=caught;}
 expect((error as{kind:string}).kind).toBe("failed");expect((error as Error).cause).toBe(original);expect(text(error).en).toContain("the first disk failure");expect(text(error).de).toContain("the first disk failure");expect(text(error).de).toContain("fehlgeschlagen");
 for(const row of[unissued,closed,failed])expect(String(text(error).en)).not.toContain("Exact live repository partition cache owner required");
});

test("a waiting submission gives up with the wait text after the bounded wait and reports progress on both languages",async()=>{
 const lines:string[]=[],languageNames=["SEMIO_LANGUAGE","LANGUAGE","LC_ALL","LC_MESSAGES","LANG"],saved=languageNames.map(name=>process.env[name]);for(const name of languageNames)delete process.env[name];
 try{
 let clock=0;const ports:PartitionSlotPortsV1={now:()=>clock,sleep:async()=>{clock+=schema.definitions.Protocol.const.maximumWaitMilliseconds/4;},write:line=>{lines.push(line);}};
 const wait=awaitPartitionSlotV1({id:"graph",busy:()=>partitionOperationSlotsV1,recheck:()=>{},onSettled:()=>()=>{}},control(),ports);
 await expect(wait).rejects.toMatchObject({kind:"waitTimeout"});const error=await wait.then(()=>{throw Error("The bounded wait must refuse");},(caught:{texts:{en:string;de:string}})=>caught);
 expect(error.texts.en).toContain("Gave up after 600 s");expect(error.texts.de).toContain("aufgegeben");expect(lines.length).toBeGreaterThan(0);expect(lines.join("")).toContain("Waiting for a free cache operation slot");expect(lines.join("")).toContain("Warte auf einen freien Cache-Operationsplatz");
 }finally{languageNames.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});}
});

test("every text equals the independent Python formatting of the schema text",()=>{
 const values={id:"graph",cause:"boom",seconds:7,busy:32,limit:32},messages=schema.definitions.Messages.const as Record<string,{en:string;de:string}>;
 for(const kind of Object.keys(messages)){
  const run=spawnSync("python3",["-I","-c","import json,sys;row=json.loads(sys.argv[1]);v=json.loads(sys.argv[2]);print(json.dumps({k:row[k].format(**v) for k in('en','de')},ensure_ascii=False))",JSON.stringify(messages[kind]),JSON.stringify(values)],{encoding:"utf8"});
  expect(run.status).toBe(0);expect(partitionTextsV1(kind as"closed",values)).toEqual(JSON.parse(run.stdout));
 }
});
