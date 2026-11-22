import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {join,dirname,relative,resolve,sep} from "node:path";
import {loadProductFunctionsV1} from "../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
import {cargoRecipeInputsV1} from "../../../../../🦀️cargo-workspace/🔒️lock/📐️closure/🟦️.ts";
import {ConcurrentEditErrorV1,concurrentEditV1} from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {describeCompileUnitsV1} from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {nativeProgressLanguagesV1} from "../../../../../🎛️owned-execution/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

test("actual Cargo capture checks the retained recipe after capacity and lock reconciliation",async()=>{
 const root=resolve("fixture"),lock=join(root,"Cargo.lock"),manifest=join(root,"Cargo.toml"),source=join(root,"lib.rs");
 const db=new Database(":memory:");
 try{
  db.exec("create table dependency(parent text, child text)");
  for(const [parent,child] of corpus.dependencies)db.query("insert into dependency values(?,?)").run(parent!,child!);
  const oracle=db.query<{outcome:string},[string,string|null]>("with recursive selected(package) as (select 'descriptor' union select child from dependency join selected on parent=package) select case when ?1 like 'cancel%' then 'cancel' when ?1 = 'source' or ?2 in selected then 'retry' else 'spawn' end as outcome");
  const capacity=corpus.capacity,request={kind:"available",maximumParallelism:capacity.preferred,minimumParallelism:capacity.minimum};
  for(const row of corpus.cases){
   expect(oracle.get(row.mutation,row.package)?.outcome,row.id).toBe(row.outcome);
   const events:string[]=[],diagnostics:string[]=[],abort=new AbortController(),control={signal:abort.signal},capture=join(root,"capture"),spawned=Error("fake spawn"),cancelled=Error("stop");
   const usesSource=row.mutation==="source"||row.mutation==="cancel-source",sourceEdit=concurrentEditV1("bytes",source,"Admitted compiler source changed before capture","build");
   let hostReads=0,now=0;
   let changed:string|null=null;
   const witness=await cargoRecipeInputsV1([lock],["descriptor"],{async digest(){throw Error("expected projection");},async recheck(){}},undefined,{async read(){
    const entry=(name:string)=>'[[package]]\nname = "'+name+'"\nversion = "'+(name===changed?"2":"1")+'"\n';
    const bytes=new TextEncoder().encode('version = 4\n\n'+entry("descriptor")+'dependencies = [\n "used",\n]\n\n'+entry("used")+'\n'+entry("unrelated"));
    return {bytes,before:"stable",after:"stable"};
   }});
   const ports={resolve,join,dirname,relative,sep,ConcurrentEditErrorV1,
    requireNativeStoragePathV1(){},requireMaterializedPathV1(file:string){return file;},mkdirSync(){},mkdtempSync:()=>capture,
    rmSync(){events.push("cleanup");},phaseLedgerV1:()=>({begin:()=>()=>{}}),
    measureV1:async(_phase:string,body:()=>Promise<unknown>)=>body(),
    async compileUnitsForHostV1(){hostReads++;return {request,host:{cores:capacity.cores},decision:capacity};},
    compilerAdmissionBudgetV1(){},describeCompileUnitsV1,nativeProgressLanguagesV1,
    cargoProvenancePhysicalControlV1:()=>({}),
    CargoSourceWitnessV1:class {
     async captureAdmitted(){events.push("source");if(row.mutation==="cancel-source")abort.abort(cancelled);throw sourceEdit;}
    },
    performance:{now:()=>now},
    process:{platform:process.platform,env:{LANG:"en"},stderr:{write:(line:string)=>diagnostics.push(line.trimEnd())},stdout:{write(){throw Error("capacity diagnostics must use stderr");}}},
    async withCompilerCapacityScopeV1(input:{request:unknown},body:(facts:{parallelism:number},issued:typeof control)=>Promise<unknown>){
     expect(input.request).toBe(request);
     expect(diagnostics).toEqual([capacity.planned]);
     if(row.mutation==="cancel-wait"){abort.abort(cancelled);throw cancelled;}
     if(row.mutation==="capacity")changed=row.package;
     now+=capacity.waitMilliseconds;
     events.push("charged");
     try{return await body({parallelism:capacity.granted},control);}catch(error){events.push("failed inside capacity");throw error;}finally{events.push("released");}
    },
    async ensureCargoLockCurrentV1({environment}: {environment: Record<string,string|undefined>}){
     expect(diagnostics).toEqual([capacity.planned,capacity.leased]);
     expect(environment.CARGO_MAKEFLAGS,row.id).toBe(row.jobserver);
     expect(environment.CARGO_BUILD_JOBS,row.id).toBe(row.jobserver?undefined:String(capacity.granted));
     events.push("lock");
     if(row.mutation==="lock")changed=row.package;
     if(row.mutation==="cancel")abort.abort(cancelled);
    },
    checkResourceLeaseControl:()=>control.signal.throwIfAborted(),
    cargoTimingsArgumentsV1:()=>[],COMPILER_RESOURCE_DIRECTORY_V1:"resource",
    spawn(){events.push("spawn");throw spawned;},
   };
   const run=(await loadProductFunctionsV1(new URL("../../../🟦️.ts",import.meta.url),["captureCargoArtifacts"],ports)).captureCargoArtifacts;
   const recheckRecipe=async()=>{events.push("recipe");await witness.recheck();};
   const expected=row.outcome==="retry"?(usesSource?sourceEdit:concurrentEditV1("input",lock,undefined,"build")):row.outcome==="cancel"?cancelled:spawned;
   let failure:unknown;
   try{
    const environment=row.ambientLanguage?undefined:{LANG:"en",CARGO_MAKEFLAGS:row.jobserver};
    failure=await run(manifest,[],{cwd:root,captureDirectory:capture,buildDirectory:root},{control,storage:{backing:{root}},output:"out",environment,retainConsumedSources:usesSource,retainCompilationBytes:1024,sourceBoundary:{maxSourceBytes:8}},root,async(body:()=>Promise<unknown>)=>body(),0,()=>usesSource?[{path:source,digest:"digest"}]:[],async()=>{throw Error("extra full plan recheck");},async()=>{},recheckRecipe);
   }catch(error){failure=error;}
   if(expected instanceof ConcurrentEditErrorV1)expect(failure,row.id).toMatchObject({reason:expected.reason,path:expected.path,rank:expected.rank});
   else expect(failure,row.id).toBe(expected);
   expect(hostReads,row.id).toBe(1);
   expect(diagnostics,row.id).toEqual(row.mutation==="cancel-wait"?[capacity.planned]:[capacity.planned,capacity.leased]);
   expect(events,row.id).toEqual(row.mutation==="cancel-wait"?["cleanup"]:usesSource?["charged","lock","recipe","source",...(row.outcome==="cancel"?["failed inside capacity"]:[]),"released","cleanup"]:row.outcome==="retry"?["charged","lock","recipe","released","cleanup"]:
    row.outcome==="cancel"?["charged","lock","failed inside capacity","released","cleanup"]:
    ["charged","lock","recipe","spawn","failed inside capacity","released","cleanup"]);
  }
 }finally{db.close();}
});
