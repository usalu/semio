/** 📈️ Tests for the re-quote of a cache charge after an in-place regrow of its image. */
import{test,expect}from"bun:test";
import{mkdtempSync,readFileSync,writeFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{loadProductFunctionsSyncV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
const OLD=19326304256,GROWN=28990021632,OWNER="repository:graph-store",ALLOCATION="57b1bb5aa98e86a1047a88fa4f7982da";
const productUrl=new URL("../../🟦️.ts",import.meta.url);
type Row={id:string;owner:string;kind:string;bytes:number;phase:string};
const build=(options:{receiptOwner?:string;rowOwner?:string;rowId?:string;journalBytes?:number;policyBytes?:number})=>{
 const root=mkdtempSync(join(tmpdir(),"requote-")),policyBytes={value:options.policyBytes??GROWN},calls:string[]=[],said:string[]=[],receipt=join(root,"graph.json");
 writeFileSync(receipt,JSON.stringify({version:1,allocation:ALLOCATION,owner:options.receiptOwner??OWNER,bytes:OLD})+"\n");
 const state={sequence:7,allocations:[{id:options.rowId??ALLOCATION,owner:options.rowOwner??OWNER,kind:"artifact-publication",bytes:options.journalBytes??OLD,phase:"committed"}as Row]};
 const apply=(label:string)=>async(request:{request:{bytes:number}})=>{calls.push(label+":"+request.request.bytes);state.allocations[0]!.bytes=request.request.bytes;state.sequence++;};
 const ports={checkResourceLeaseControl:()=>{},repositoryCacheBudgetV1:()=>({partitions:{graph:{capacityBytes:policyBytes.value+1048576,filesystemBytes:policyBytes.value,maximumArtifactBytes:policyBytes.value}}}),persistentStateArtifactBudgetV1:(value:number)=>value,machinePathV1:(_root:string,path:string)=>path,requireNativeStoragePathV1:(_root:string,path:string)=>path,
  observeNativeStorageStateV1:async()=>({sequence:state.sequence,allocations:state.allocations.map(row=>({...row}))}),regrowNativeStorageAllocationV1:apply("regrow"),rebudgetNativeStorageAllocationV1:apply("rebudget")};
 const api=loadProductFunctionsSyncV1(productUrl,["repositoryCacheChargeStaleV1","requoteRepositoryCacheChargeV1","openRepositoryElasticChargeV1"],ports) as{repositoryCacheChargeStaleV1:Function;requoteRepositoryCacheChargeV1:Function;openRepositoryElasticChargeV1:Function};
 const image={root,withExclusiveMaintenance:async(_request:unknown,run:Function)=>({acquired:true,value:await run({observe:async()=>({identity:"1:2:3"}),syncMetadata:async()=>{},recheck:async()=>{}},{kind:"kernel-ex",control:{},proof:{}})})};
 return{root,receipt,state,calls,said,api,image,policyBytes};
};
test("a regrown image: the stale charge is detected, re-quoted under EX with a bilingual line, and then agrees with the policy",async()=>{
 const fixture=build({});try{
  expect(fixture.api.repositoryCacheChargeStaleV1("/w","graph",fixture.image)).toBe(true);
  expect(await fixture.api.requoteRepositoryCacheChargeV1("/w","graph",fixture.image,{},(line:string)=>fixture.said.push(line))).toBe(true);
  expect(fixture.calls).toEqual(["regrow:"+GROWN]);
  expect(JSON.parse(readFileSync(fixture.receipt,"utf8"))).toEqual({version:1,allocation:ALLOCATION,owner:OWNER,bytes:GROWN});
  expect(fixture.api.repositoryCacheChargeStaleV1("/w","graph",fixture.image)).toBe(false);
  expect(fixture.said.length).toBe(1);expect(fixture.said[0]).toContain("follows the regrown image: "+OLD+" -> "+GROWN);expect(fixture.said[0]).toContain("folgt dem gewachsenen Image");
  expect(await fixture.api.requoteRepositoryCacheChargeV1("/w","graph",fixture.image,{})).toBe(false);expect(fixture.calls.length).toBe(1);
 }finally{rmSync(fixture.root,{recursive:true,force:true});}
});
test("a journal that already grew only gets its receipt replaced; a shrink uses the rebudget event",async()=>{
 const done=build({journalBytes:GROWN});try{await done.api.requoteRepositoryCacheChargeV1("/w","graph",done.image,{});expect(done.calls).toEqual([]);expect(JSON.parse(readFileSync(done.receipt,"utf8")).bytes).toBe(GROWN);}finally{rmSync(done.root,{recursive:true,force:true});}
 const shrink=build({policyBytes:OLD-1048576});try{await shrink.api.requoteRepositoryCacheChargeV1("/w","graph",shrink.image,{});expect(shrink.calls).toEqual(["rebudget:"+(OLD-1048576)]);}finally{rmSync(shrink.root,{recursive:true,force:true});}
});
test("a foreign owner or allocation is refused bilingually with the remedy and nothing is changed",async()=>{
 for(const options of[{rowOwner:"repository:other-store"},{rowId:"b".repeat(32)}]){
  const fixture=build(options);try{
   const before=readFileSync(fixture.receipt,"utf8"),error=await fixture.api.requoteRepositoryCacheChargeV1("/w","graph",fixture.image,{}).then(()=>undefined,(reason:unknown)=>reason as Error);
   expect(error?.message).toContain("is foreign and is not re-quoted");expect(error?.message).toContain("cache-reset-graph");expect(error?.message).toContain("ist fremd");
   expect(fixture.calls).toEqual([]);expect(readFileSync(fixture.receipt,"utf8")).toBe(before);
  }finally{rmSync(fixture.root,{recursive:true,force:true});}
 }
 const foreign=build({receiptOwner:"repository:other-store"});try{expect(foreign.api.repositoryCacheChargeStaleV1("/w","graph",foreign.image)).toBe(false);}finally{rmSync(foreign.root,{recursive:true,force:true});}
});
test("the read path: open, detect, close, re-quote, reopen and the charge agrees",async()=>{
 const fixture=build({}),log:string[]=[];try{
  const open=async()=>{log.push("open");return{...fixture.image,close:async()=>{log.push("close");}};};
  const image=await fixture.api.openRepositoryElasticChargeV1("/w","graph",{},open);
  expect(log).toEqual(["open","close","open","close","open"]);expect(typeof image.close).toBe("function");
  expect(fixture.api.repositoryCacheChargeStaleV1("/w","graph",image)).toBe(false);
  log.length=0;await fixture.api.openRepositoryElasticChargeV1("/w","graph",{},open);expect(log).toEqual(["open"]);
 }finally{rmSync(fixture.root,{recursive:true,force:true});}
});
