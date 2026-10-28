/** 🧪️ Tests for inputs storage shared-writer. */
import{expect,test}from"bun:test";
import{AsyncLocalStorage}from"node:async_hooks";
import{readFile}from"node:fs/promises";
import ts from"typescript";
import{evaluateExtractedFunctionV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import componentSchema from"../../🧬️schema/🔣️.json";

/** 🤝️ Contract: the namespace writer lease of a graph fact writer is exclusive unless its options ask for a shared one, in the first try and in the queued wait alike, and the lease is released after the body in both modes. */
const source=async()=>{
 const file=ts.createSourceFile("storage.mjs",await readFile(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),definition=file.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="withGraphFactWriterV1")!;
 return ts.transpileModule(definition.getText(file),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
};

async function run(options:Record<string,unknown>|undefined,contend:boolean){
 const events:{kind:string;mode?:string;cohort?:boolean;timeoutMs?:number;resource?:string}[]=[],controller=new AbortController(),phase={signal:controller.signal},original={};let tries=0;
 const image={root:"/workspace/.cache/b/g/m",recheck:async()=>{}},store={root:image.root,image,invocation:original,recheck:image.recheck};
 const ports={schema:componentSchema,requireCurrentRepositoryBuildGraphConsumerV1:()=>{},requireIssuedResourceControlInvocationV1:()=>{},checkResourceLeaseControl:()=>{},requireIssuedResourceControlRefinementV1:()=>{},requireRepositoryBuildGraphWriterV1:()=>{},writers:new AsyncLocalStorage(),datasets:new AsyncLocalStorage(),graphDatasetWorkspaceV1:(value:string)=>value,
  openRepositoryCachePartitionV1:async()=>({root:image.root,recheck:async()=>{},close:async()=>{}}),
  withRepositoryCacheStoreV1:async(_workspace:string,_id:string,_original:unknown,_control:unknown,body:Function)=>body(store,phase),
  acquireQueuedResourceLease:async(request:{mode:string;timeoutMs?:number;resource:string;sharedCohort?:boolean})=>{
   events.push({kind:request.timeoutMs===0?"try":"wait",mode:request.mode,cohort:request.sharedCohort===true,timeoutMs:request.timeoutMs,resource:request.resource});
   if(request.timeoutMs===0&&contend&&++tries===1)throw Error("Resource lease timed out: "+request.resource);
   return{release:()=>{events.push({kind:"release"});}};
  },
  repositoryLeaseDirectoryV1:()=>"/workspace/.cache/b/l/m/data",buildGraphWriterResourceV1:(_workspace:string,kind:string,identity:string)=>"writer:"+kind+":"+identity,resolve:(value:string)=>value,randomBytes:()=>({toString:()=>"owner"}),
  quotaSchema:{policy:{minimumAvailableBytes:1,maximumBytes:100}},nativeStatfsSyncV1:()=>({bavail:100,bsize:1}),maintainBuildGraphOptionalCachesV1:async()=>false,statSync:()=>({dev:1,ino:2}),
  openOriginalGraphLedgerV1:async()=>({observe:()=>({usedBytes:10}),close:()=>{}}),buildGraphOptionalRetirementDueV1:()=>false,createBuildGraphOptionalQuotaV1:()=>({})};
 const fn=evaluateExtractedFunctionV1(await source(),"withGraphFactWriterV1",ports) as (...args:unknown[])=>Promise<number>;
 const value=await fn("/workspace",original,phase,async()=>{events.push({kind:"body"});return 17;},"selection:"+"a".repeat(64),options);
 return{value,events};
}

for(const[label,options,mode]of[["optionalCaches off",{optionalCaches:false},"exclusive"],["shared false",{optionalCaches:false,shared:false},"exclusive"],["shared true",{optionalCaches:false,shared:true},"shared"]]as const)
 test("the namespace writer lease is "+mode+" for "+label,async()=>{
  const{value,events}=await run(options as never,false);
  expect(value).toBe(17);expect(events.map(row=>row.kind)).toEqual(["try","body","release"]);expect(events[0]!.mode).toBe(mode);expect(events[0]!.cohort).toBe(mode==="shared");expect(events[0]!.timeoutMs).toBe(0);
 });

test("a contended shared writer waits in the queue in the same mode and releases after the body",async()=>{
 const{value,events}=await run({optionalCaches:false,shared:true},true);
 expect(value).toBe(17);expect(events.map(row=>row.kind)).toEqual(["try","wait","release","try","body","release"]);
 expect(events.filter(row=>row.kind==="try"||row.kind==="wait").map(row=>row.mode)).toEqual(["shared","shared","shared"]);expect(events.filter(row=>row.kind==="try"||row.kind==="wait").every(row=>row.cohort)).toBe(true);
 expect(events.filter(row=>row.kind==="wait")[0]!.timeoutMs).toBeUndefined();
});
