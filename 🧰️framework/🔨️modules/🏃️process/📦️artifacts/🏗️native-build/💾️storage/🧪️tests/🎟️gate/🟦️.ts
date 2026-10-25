import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{mkdtempSync}from"node:fs";
import{join}from"node:path";
import{acquireNativeStorageV1,ordinaryNativeStoragePortV1,requireNativeStorageReservationV1}from"../../🟦️.ts";
import{createScriptProcessEnvelope,createIssuedResourceControlV1,createRenewableIssuedResourceControlV1,createRequestedIssuedResourceControlV1,withRequestedIssuedResourceControlV1,withScriptProcessEnvelope}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../🔒️leases/🎛️control/🟦️.ts";
import schema from"../../🧬️schema/🎟️gate.json";
import corpus from"./🔣️.json";

const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifacts)throw Error("Original test artifact directory required");

/** 🎟️ Pins which controls may present a reservation: the exact control that issued it and every live control derived beneath it; the lineage oracle is an independent recursive SQLite query over the controls this law derived. @see ../../🟦️.ts */
test("a native storage reservation is presented only by its issuing control and live controls beneath it, as an independent lineage query proves",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const root=mkdtempSync(join(artifacts,"gate-")),policy={version:1 as const,root,leaseDirectory:join(root,"leases"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100},database=new Database(":memory:");
 database.run("create table edges(child text primary key,parent text)");database.run("create table live(id text primary key,live integer not null)");
 const port={...ordinaryNativeStoragePortV1(),withOperation:<T>(actual:ResourceLeaseControl,run:(control:ResourceLeaseControl)=>Promise<T>)=>run(actual),withJournal:<T>(actual:ResourceLeaseControl,run:(control:ResourceLeaseControl)=>Promise<T>)=>run(actual),observe:async()=>({root,identity:"gate-fixture",maximumPhysicalBytes:100}),recheck:async()=>{},syncMetadata:async()=>{},retire:async()=>{}};
 const edge=(child:string,parent:string|null)=>{database.run("insert into edges values(?,?)",[child,parent]);database.run("insert into live values(?,1)",[child]);};
 const controls=new Map<string,ResourceLeaseControl>();
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:gate",maximumElapsedMilliseconds:0},{fixture:"gate"},Date.now()),async original=>{
  const top=createIssuedResourceControlV1(original,"fixture:gate-top",600000),issuer=createRequestedIssuedResourceControlV1(top,500000),narrowed=createRequestedIssuedResourceControlV1(issuer,400000),renewable=createRenewableIssuedResourceControlV1(issuer,{idleMilliseconds:300000,maximumMilliseconds:450000},1),grandchild=createRequestedIssuedResourceControlV1(narrowed,300000),sibling=createRequestedIssuedResourceControlV1(top,400000),unrelated=createIssuedResourceControlV1(original,"fixture:gate-unrelated",350000);
  edge("top",null);edge("issuer","top");edge("narrowed-child","issuer");edge("renewable-child","issuer");edge("grandchild","narrowed-child");edge("sibling","top");edge("unrelated-original-control",null);edge("retired-child","issuer");
  for(const[name,control]of Object.entries({"top":top,"issuer":issuer,"narrowed-child":narrowed,"renewable-child":renewable,"grandchild":grandchild,"sibling":sibling,"unrelated-original-control":unrelated}))controls.set(name,control);
  await withRequestedIssuedResourceControlV1(issuer,450000,async phase=>{controls.set("retired-child",phase);});
  database.run("update live set live=0 where id='retired-child'");
  controls.set("fabricated",Object.freeze({signal:new AbortController().signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}}));
  const reservation=await acquireNativeStorageV1({policy,control:issuer,port,owner:"gate",kind:"compiler-workspace",bytes:10});
  for(const row of corpus.cases){
   const name=row.relation==="issuer-parent"?"top":row.relation,presented=row.relation==="issuer"?issuer:controls.get(name)!;
   const lineage=database.query("with recursive up(id) as (select ?1 union select edges.parent from edges join up on edges.child=up.id where edges.parent is not null) select exists(select 1 from up where id='issuer') as under, (select live from live where id=?1) as live").get(name==="issuer"?"issuer":name) as{under:number;live:number|null},oracle=name==="fabricated"?"refused":lineage.under&&lineage.live?"accepted":"refused";
   let actual:"accepted"|"refused";try{requireNativeStorageReservationV1(reservation,presented);actual="accepted";}catch(error){expect(String(error),JSON.stringify(row)).toContain((row as{failure?:string}).failure??"");actual="refused";}
   expect(actual,JSON.stringify(row)).toBe(oracle);expect<string>(actual).toBe(row.expected);
  }
 });
 database.close();
},60000);
