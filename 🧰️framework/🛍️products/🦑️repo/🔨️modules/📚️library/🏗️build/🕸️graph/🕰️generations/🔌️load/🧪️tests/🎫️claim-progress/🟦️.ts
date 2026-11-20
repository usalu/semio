import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {GENERATION_POLICY_V1,createNodeFilePortV1} from "../../../🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../../🧩️claims/🟨️.mjs";
import {CUSTODY_POLICY_V1,createSliceCustodyV1} from "../../../🔐️custody/🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import fixture from "./🔣️.json" with {type:"json"};

test("cold owner progress updates its own live custody before the first seal",async()=>{
 const workspace=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"claim-progress-")),root=join(workspace,".gen"),clock={at:1_000_000};
 const custody=createSliceCustodyV1({root,fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(workspace,"claim-leases")),now:()=>clock.at});
 const mock=createMockHost(workspace),host={...mock.host};
 const db=new Database(":memory:");
 try{
  await seed(workspace,ringFiles(fixture.owners));
  const result=await loadGenerationGraphV1({...host,
   async withBuilder(body:Function){const claim=await custody.hold({structural:true});try{return await body({root},{published:mock.published},claim);}finally{await claim.release();}},
   async discoverPart(spec:any,signal?:AbortSignal,_control?:unknown,_done?:unknown,_running?:unknown,ownerProgress?:(done:number,total:number)=>Promise<void>){
    expect(spec.cold).toBe(true);
    for(const row of fixture.samples){clock.at+=10000;await ownerProgress?.(row.done,row.total);}
    const [active]=await custody.list();
    expect(active?.progress).toMatchObject(fixture.progress);
    const oracle=db.query<{done:number},[string]>("SELECT max(json_extract(value,'$.done')) AS done FROM json_each(?)").get(JSON.stringify(fixture.samples))!;
    expect(active?.progress?.done).toBe(oracle.done);
    return host.discoverPart(spec,signal);
   }
  } as never);
  expect(Object.keys((result.graph as {projects:object}).projects)).toHaveLength(fixture.owners);
 }finally{db.close();rmSync(workspace,{recursive:true,force:true});}
});

test("an expired writer cannot seal a generation after another writer actually enters custody",async()=>{
 const workspace=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"publication-fence-")),root=join(workspace,".gen"),clock={at:1_000_000};
 const newCustody=()=>createSliceCustodyV1({root,fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(workspace,"claim-leases")),now:()=>clock.at});
 const first=newCustody(),second=newCustody(),mock=createMockHost(workspace),host={...mock.host};
 let successor:Awaited<ReturnType<typeof first.hold>>|undefined,request:Promise<Awaited<ReturnType<typeof first.hold>>>|undefined;
 const consent=new AbortController(),reserved={published:mock.published};
 let retired:Awaited<ReturnType<typeof first.hold>>|undefined;
 try{
  await seed(workspace,ringFiles(fixture.owners));
  const previous=loadGenerationGraphV1({...host,
   async withBuilder(body:Function){const claim=retired=await first.hold({structural:true});try{return await body({root},reserved,claim);}finally{await claim.release();}},
   async discoverPart(spec:any,signal?:AbortSignal){
    let asked!:()=>void;
    const requested=new Promise<void>(resolve=>{asked=resolve;});
    clock.at+=CUSTODY_POLICY_V1.stallMilliseconds+1;
    request=second.hold({structural:true,signal:consent.signal,say:key=>{if(key==="askedStalled")asked();}});
    await Promise.race([requested,request.then(()=>{throw Error("successor overlapped before its wait");})]);
    clock.at+=CUSTODY_POLICY_V1.askGraceMilliseconds+1;
    successor=await request;
    await expect(retired!.fence(()=>{throw Error("fenced publication ran");})).rejects.toMatchObject({code:"SEMIO_STALE_IMPLEMENTATION",handover:{reason:"stalled"}});
    return host.discoverPart(spec,signal);
   },
  } as never);
  await expect(previous).rejects.toMatchObject({code:"SEMIO_STALE_IMPLEMENTATION",handover:{reason:"stalled"}});
  expect(existsSync(join(root,GENERATION_POLICY_V1.pointer))).toBe(false);
  const winner=await loadGenerationGraphV1({...host,withBuilder:(body:Function)=>body({root},reserved,successor)} as never);
  expect(Object.keys((winner.graph as {projects:object}).projects)).toHaveLength(fixture.owners);
  const same=await loadGenerationGraphV1({...host,withBuilder:()=>{throw Error("a verified follower must not rebuild");}} as never);
  expect(same.graph).toEqual(winner.graph);
 }finally{consent.abort();await successor?.release();await request?.catch(()=>undefined);rmSync(workspace,{recursive:true,force:true});}
},20000);

for(const scenario of fixture.handover)test("cold discovery acknowledges a "+scenario.reason+" handover"+(scenario.parentCancels?" and preserves parent cancellation during drain":" before the grace expires"),async()=>{
 const workspace=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"claim-handover-")),root=join(workspace,".gen"),clock={at:1_000_000};
 const newCustody=()=>createSliceCustodyV1({root,fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(workspace,"claim-leases")),now:()=>clock.at});
 const first=newCustody(),second=newCustody(),mock=createMockHost(workspace),host=mock.host,db=new Database(":memory:");
 db.exec("CREATE TABLE events (id INTEGER PRIMARY KEY, name TEXT NOT NULL)");
 const record=db.query("INSERT INTO events (name) VALUES (?)"),asked=Promise.withResolvers<void>(),cancelSuccessor=new AbortController(),parent=new AbortController(),parentReason=new Error("caller cancelled during handover");
 let request:ReturnType<typeof second.hold>|undefined,askedAt=0;
 try{
  await seed(workspace,ringFiles(fixture.owners));
  const previous=loadGenerationGraphV1({...host,...scenario.parentCancels?{signal:parent.signal}:{},
   async withBuilder(body:Function){
    const claim=await first.hold({structural:true,...scenario.reason==="stale-code"?{holder:{pid:process.pid,command:"old graph",digest:"1".repeat(64),startedMs:clock.at}}:{}});
    try{return await body({root},{published:mock.published},claim);}
    finally{await claim.release();record.run("release");}
   },
   async discoverPart(spec:any,signal?:AbortSignal,_control?:unknown,_done?:unknown,_running?:unknown,ownerProgress?:(done:number,total:number)=>Promise<void>){
    expect(spec.cold).toBe(true);
    record.run("discover");
    if(scenario.sampleProgress)for(const row of fixture.samples){clock.at+=CUSTODY_POLICY_V1.progressWriteMilliseconds;await ownerProgress?.(row.done,row.total);}
    clock.at+=scenario.reason==="stalled"?CUSTODY_POLICY_V1.stallMilliseconds+1:1;
    request=second.hold({structural:true,signal:cancelSuccessor.signal,...scenario.reason==="stale-code"?{holder:{pid:process.pid,command:"current graph",digest:"2".repeat(64),startedMs:clock.at},currentDigest:()=>"2".repeat(64)}:{},say:key=>{
     if(key==="askedStale"||key==="askedStalled"){askedAt=clock.at;record.run("ask");asked.resolve();}
    }});
    await Promise.race([asked.promise,request.then(()=>{throw Error("successor entered before handover");})]);
    expect(signal).toBeDefined();
    await new Promise<void>(resolve=>{if(signal!.aborted)resolve();else signal!.addEventListener("abort",()=>resolve(),{once:true});});
    record.run("abort");
    if(scenario.parentCancels)parent.abort(parentReason);
    await Promise.resolve();
    record.run("drain");
    if(!scenario.returnsAfterAbort)throw new DOMException("Inference cancelled","AbortError");
    const part=await host.discoverPart(spec,undefined);
    return {...part,dispose:()=>{record.run("dispose");return part.dispose?.();}};
   },
  } as never);
  const samples=scenario.sampleProgress?fixture.samples:[];
  const counts=db.query<{done:number,total:number},[string]>("SELECT coalesce(max(json_extract(value,'$.done')),0) AS done, coalesce(max(json_extract(value,'$.total')),0) AS total FROM json_each(?)").get(JSON.stringify(samples));
  if(scenario.parentCancels)await expect(previous).rejects.toBe(parentReason);
  else await expect(previous).rejects.toMatchObject({code:"SEMIO_STALE_IMPLEMENTATION",handover:{reason:scenario.reason,...counts}});
  expect(clock.at).toBe(askedAt);
  expect(existsSync(join(root,GENERATION_POLICY_V1.pointer))).toBe(false);
  const successor=await request!;
  try{record.run("successor");expect(successor.structural).toBe(true);}
  finally{await successor.release();}
  expect(db.query<{name:string},[]>("SELECT name FROM events ORDER BY id").all().map(row=>row.name)).toEqual(scenario.events);
 }finally{cancelSuccessor.abort();await request?.catch(()=>undefined);db.close();rmSync(workspace,{recursive:true,force:true});}
},20000);

test("parent cancellation keeps its original reason during cold discovery",async()=>{
 const workspace=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"claim-parent-abort-")),root=join(workspace,".gen");
 const fs=createNodeFilePortV1(),checking=Promise.withResolvers<void>(),unblock=Promise.withResolvers<void>(),stopping=Promise.withResolvers<void>();
 let paused=false;
 const custody=createSliceCustodyV1({root,fs:{...fs,async read(path:string){
  if(paused&&path.endsWith(".ask.json")){paused=false;checking.resolve();await unblock.promise;}
  return fs.read(path);
 }},lease:createFileClaimLeasePortV1(join(workspace,"claim-leases"))});
 const mock=createMockHost(workspace),parent=new AbortController(),reason=new Error("cancelled by the caller");
 try{
  await seed(workspace,ringFiles(fixture.owners));
  const previous=loadGenerationGraphV1({...mock.host,signal:parent.signal,
   async withBuilder(body:Function){const claim=await custody.hold({structural:true});try{return await body({root},{published:mock.published},claim);}finally{await claim.release();}},
   async discoverPart(_spec:any,signal:AbortSignal){
    paused=true;
    await checking.promise;
    parent.abort(reason);
    stopping.resolve();
    expect(signal.reason).toBe(reason);
    throw new DOMException("Inference cancelled","AbortError");
   }
  } as never);
  void previous.catch(()=>undefined);
  await stopping.promise;
  expect((await custody.list()).map(claim=>claim.state)).toEqual(["held"]);
  unblock.resolve();
  await expect(previous).rejects.toBe(reason);
  expect(await custody.list()).toEqual([]);
  expect(existsSync(join(root,GENERATION_POLICY_V1.pointer))).toBe(false);
 }finally{unblock.resolve();rmSync(workspace,{recursive:true,force:true});}
},20000);
