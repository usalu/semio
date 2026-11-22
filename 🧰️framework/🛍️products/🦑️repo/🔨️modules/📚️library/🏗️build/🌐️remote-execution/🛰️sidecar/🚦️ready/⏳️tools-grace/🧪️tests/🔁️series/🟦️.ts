import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {join} from "node:path";
import {mkdirSync,mkdtempSync} from "node:fs";
import {recordToolsProgress,sampleToolsProgress,toolsGraceActive,type ToolsCheckpoint,type ToolsSample} from "../../🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {leaseProcessStartedAtMsV1,leaseRegistryFolderV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

declareLibraryBuildTestEnvelopeV1({suite:"build-remote-execution",maximumElapsedMilliseconds:30000});

test("only a newer authenticated capture series can rebuild grace with two advances",()=>{
 const db=new Database(":memory:"),proof={pid:5,birth:100,parentSince:110,sourceSince:115};
 try{
  const oracle=db.query("select ?1 > ?2 and ?3 > ?4 and ?5 > ?3 and ?6 > ?5 and ?7 < ?8 and ?8 < ?9 as allowed");
  for(const row of fixture.cases){
   const [first,baseline,a,b]=row.counts,[initial,renewed]=row.series,[old,pinned,firstMove,secondMove]=row.registry;
   const allowed=Boolean((oracle.get(renewed!,initial!,pinned!,old!,firstMove!,secondMove!,baseline!,a!,b!) as {allowed:number}).allowed);
   expect(allowed,row.name).toBe(row.allowed);
   let state:ToolsCheckpoint|null=null;
   for(const [index,done] of [first,baseline,a,b].entries()){
    state=recordToolsProgress(state,{proof,phase:fixture.phase,series:row.series[index]!,done:done!,updatedAt:row.registry[index]!},[461,481,491,501][index]!);
    const at={stage:"tools",started:100,now:501,soft:100,quiet:45,sampleLimit:45};
    expect(toolsGraceActive(state,at),row.name+" step "+index).toBe(index===3&&allowed);
   }
   expect(toolsGraceActive(state,{stage:"tools",started:100,now:601,soft:100,quiet:200,sampleLimit:200})).toBe(false);
  }
 }finally{db.close();}
});

test("issued selected payload progress requalifies in a new series on the same dual-exclusive registry writer",()=>withBuildTestInvocationV1(async original=>{
 const beforeRoot=process.env.SEMIO_LEASE_REGISTRY_DIRECTORY,beforeOwner=process.env.SEMIO_LEASE_OWNER;
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"series-")),registry=join(base,"r"),directory=join(base,"l"),root=join(base,"s");
 mkdirSync(root);process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=registry;
 const started=leaseProcessStartedAtMsV1,owner="ready:abcdefgh12:tools:"+Date.now();
 process.env.SEMIO_LEASE_OWNER=owner;
 try{
  const phase=createIssuedResourceControlV1(original,"repository:command",0),parent="native-fixed-image:"+join(root,"d"),source="javascript-dependency-bindings:"+root,folder=leaseRegistryFolderV1(registry,directory);
  const acquire=(resource:string)=>acquireResourceLease({control:phase,signal:phase.signal,directory,resource,mode:"exclusive",timeoutMs:0,maximumDatabaseBytes:16384});
  const native=await acquire(parent);
  try{
   const bindings=await acquire(source);
   try{
    const observe=()=>sampleToolsProgress({folder,directory,owner,parent,source,started,now:Date.now(),tolerance:2000,members:[{pid:process.pid,startMs:started}]});
    const report=async(resource:string,done:number):Promise<ToolsSample>=>{
     await phase.publish({resource:resource+":"+done,mode:"shared",elapsedMs:0});
     for(let i=0;i<80;i++){
      const row=observe();
      if(row?.phase===resource&&row.done===done)return row;
      await Bun.sleep(20);
     }
     throw Error("Missing owned "+resource+" progress");
    };
    const old=await report(fixture.phase,fixture.high);let state=recordToolsProgress(null,old,Date.now());
    const middle=await report(fixture.between,1731);state=recordToolsProgress(state,middle,Date.now());
    await Bun.sleep(20);
    const first=await report(fixture.phase,fixture.renewed[0]!);
    state=recordToolsProgress(state,first,Date.now());
    const baseline=state;
    await Bun.sleep(5100);
    const advanced=await report(fixture.phase,fixture.renewed[1]!);
    state=recordToolsProgress(state,advanced,Date.now());
    const once=state;
    await Bun.sleep(5100);
    const final=await report(fixture.phase,fixture.renewed[2]!);
    const now=Date.now();state=recordToolsProgress(state,final,now);
    expect(first.proof).toEqual(old.proof);expect(final.proof).toEqual(old.proof);
    expect(first.series).toBeGreaterThan(old.series);expect(final.series).toBe(first.series);
    const decide={stage:"tools",started,now,soft:Math.floor((now-started)/4.3),quiet:20000,sampleLimit:15000};
    expect(toolsGraceActive(baseline,decide)).toBe(false);
    expect(toolsGraceActive(once,decide)).toBe(false);
    expect(toolsGraceActive(state,decide)).toBe(true);
   }finally{bindings.release();}
  }finally{native.release();}
 }finally{
  if(beforeRoot===undefined)delete process.env.SEMIO_LEASE_REGISTRY_DIRECTORY;else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=beforeRoot;
  if(beforeOwner===undefined)delete process.env.SEMIO_LEASE_OWNER;else process.env.SEMIO_LEASE_OWNER=beforeOwner;
 }
}),28500);
