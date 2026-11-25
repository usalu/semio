import{expect}from"bun:test";
import{Database}from"bun:sqlite";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import{captureOwnedCommandResultV1}from"@semio-tech/process/execution";
import Ajv from"ajv";
import{createHash}from"node:crypto";
import{existsSync,lstatSync,mkdirSync,readFileSync,rmSync}from"node:fs";
import{join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withRepositoryBuildGraphFixtureV1,withRepositoryBuildGraphPublicationFixtureV1}from"../../../📥️inputs/💾️storage/🟨️.mjs";
import{repositoryBuildGraphDataDirectoryV1,repositoryBuildGraphWriterDirectoryV1,withRepositoryBuildGraphConsumerV1}from"../../../📥️inputs/💾️storage/🟨️.mjs";
import{compactSourceInputs,cargoLockInput}from"../../../📥️inputs/🟨️.mjs";
import{captureInferenceInventoryReferencesV1,inferenceInventoryReferencesHoldV1,inferenceInventoryPresenceStatsV1,inferenceInventoryLeafMissingV1}from"../../📥️reuse/🟨️.mjs";
import{observationLookupClockV1,observeProjectInferenceV1,sourceObservationStatsV1,useProjectFleetV1}from"../../🟨️.mjs";
import{withManagedGraphSourceReadBatchV1,readExactSourceInventoryV1}from"../../../📥️inputs/🧮️digest/📥️read/🟨️.mjs";
import schema from"../../📥️reuse/🧬️schema/🔣️.json";
import nativeTimingSchema from"../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/⏱️verification/🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🧫️cases.json";
import bodyCorpus from"./body.json";
import imports from"./📥️import.json";
import{SLICE_REPORT_POLICY_V1,registerSliceReportSourceV1,writeSliceReportV1}from"../../../🧠️memory/📋️slice-report/🟨️.mjs";
import{requireNativePlatformImageWriterV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import receiverCorpus from"../../../📥️inputs/💾️storage/🧪️fixture/🧫️cases.json";
import fixtureSchema from"../../../📥️inputs/💾️storage/🧪️fixture/🧬️schema/🔣️.json";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();
declareLibraryBuildTestEnvelopeV1({suite:"build-observation",maximumElapsedMilliseconds:900000});
const hash=(text:string)=>createHash("sha256").update(text).digest("hex"),workspace=resolve(repository.root),interruptedStatusV1=130;
const bounded=(path:string)=>{expect(path.length).toBeLessThanOrEqual(256);return path;};
const inventory=(kind:"source"|"lock")=>kind==="source"?compactSourceInputs(["{workspaceRoot}/Cargo.toml"],workspace):[cargoLockInput("Cargo.toml",workspace)];
const leaf=(value:unknown)=>{const name=captureInferenceInventoryReferencesV1(value)[0];if(!name)throw Error("Actual inventory reference required");const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),namespace=source?"build-input-closures":"build-lock-closures",key=name.slice(source?21:17);return{name,namespace,key,path:bounded(join(repositoryBuildGraphDataDirectoryV1(workspace,namespace),key+".json"))};};

registerBuildTestLawV1("inference reuse revalidates physical inventories inside the prior ten minute window","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"inference:presence",180000),db=new Database(":memory:"),ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.getSchema(schema.$id+"#/definitions/PresenceCorpus")?.(corpus)).toBe(true);
 try{await withRepositoryBuildGraphFixtureV1(workspace,original,phase,async fixture=>fixture.withWriter(async(_store,control)=>{
  for(const kind of["source","lock"]as const){let calls=0;const cachePath=bounded(join(repositoryBuildGraphWriterDirectoryV1(workspace,"inference"),"np",hash(kind).slice(0,24)+".json"));mkdirSync(join(repositoryBuildGraphWriterDirectoryV1(workspace,"inference"),"np"),{recursive:true});const options={cachePath,key:kind,owner:cachePath,reusable:(value:unknown)=>inferenceInventoryReferencesHoldV1(workspace,value,control)},compute=()=>({generation:++calls,namedInputs:{default:inventory(kind)}}),run=()=>observeProjectInferenceV1(options,compute),started=Date.now(),publications=fixture.optionalPublications(),first=await run(),record=leaf(first),before=lstatSync(record.path,{bigint:true}),counts:number[]=[calls];
   const declined=fixture.optionalPublications();expect(new Ajv({strict:true}).validate(fixtureSchema.definitions.PublicationFacts,declined)).toBe(true);expect(declined.attempts).toBeGreaterThan(publications.attempts);expect(declined.declined).toBeGreaterThan(publications.declined);expect(existsSync(cachePath)).toBe(false);expect(sourceObservationStatsV1().projectImages).toBeGreaterThan(0);const warm=await run();counts.push(calls);expect(warm.generation).toBe(1);expect(lstatSync(record.path,{bigint:true}).ino).toBe(before.ino);expect(await run()).toEqual(warm);counts.push(calls);expect(existsSync(cachePath)).toBe(false);rmSync(record.path);const rebuilt=await run();counts.push(calls);expect(rebuilt.generation).toBe(2);expect(existsSync(record.path)).toBe(true);expect(hash(readFileSync(record.path,"utf8"))).toBe(record.key);expect(await run()).toEqual(rebuilt);counts.push(calls);expect(Date.now()-started).toBeLessThan(corpus.validityMilliseconds);expect(counts).toEqual([1,1,1,2,2]);expect(db.query<{generation:number},[string]>("select sum(case when cast(key as integer) in(0,3) then 1 else 0 end) over(order by cast(key as integer)) generation from json_each(?)").all(JSON.stringify(counts)).map(row=>row.generation)).toEqual(counts);
   rmSync(record.path);await fixture.withConsumer(async(_current,actual)=>{const missing=await withManagedGraphSourceReadBatchV1(workspace,actual,batch=>readExactSourceInventoryV1(batch,record.namespace as Parameters<typeof readExactSourceInventoryV1>[1],record.key));expect(missing).toBeNull();});
  }
 }));}finally{db.close();}
});

registerBuildTestLawV1("fleet owner seed reports actual graph publication", "sdk-runtime", async original=>{
 const control=createIssuedResourceControlV1(original,"observation:fleet-seed",180000),db=new Database(":memory:");
 try{
  for(const row of imports){
   const verify=row.funded?withRepositoryBuildGraphPublicationFixtureV1:withRepositoryBuildGraphFixtureV1;
   await verify(workspace,original,control,async fixture=>fixture.withWriter(async()=>{
    const cachePath=bounded(join(repositoryBuildGraphWriterDirectoryV1(workspace,"inference"),"np",hash(row.id).slice(0,24)+".json")),before=observationLookupClockV1().count;
    let stored=false,seeded=false,computed=0,requested=0;
    useProjectFleetV1({offer(){},async gate(offered){
     const info=offered as {keyText:string;import(record:unknown):Promise<boolean>};
     requested++;
     seeded=await info.import({digest:hash(info.keyText),image:JSON.stringify("remote"),authority:[],observations:[]});
     stored=existsSync(cachePath);
     return{seeded,publish(){}};
    }});
    try{
     const value=await observeProjectInferenceV1({cachePath,key:row.id,owner:row.id,fleet:row.id},()=>{computed++;return "local";});
     const actual={stored:Number(stored),seeded:Number(seeded),lookups:observationLookupClockV1().count-before,computed,value};
     const oracle=db.query("select ?1 stored,?1 seeded,1+?1 lookups,1-?1 computed,case ?1 when 1 then 'remote' else 'local' end value").get(Number(row.funded));
     expect(requested).toBe(1);
     expect(actual).toEqual(row.expected);
     expect(actual).toEqual(oracle);
    }finally{useProjectFleetV1(undefined);}
   }));
  }
 }finally{db.close();}
});

/** 📨️ Launches only the literal compiler-captured static child through the same genuine Original. */
async function childOutcome(original:Parameters<typeof createIssuedResourceControlV1>[0],control:ResourceLeaseControl,mode:string,kind:string,fault:string){const path=fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),result=await captureOwnedCommandResultV1(process.execPath,[path,mode,kind,fault],workspace,{control,env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:32768,maximumLines:256}}),line=result.stdout.trim().split("\n").at(-1);expect(result.signal,result.stderr).toBeNull();if(!line)throw Error("Fixture child produced no result (exit "+result.status+")\n"+result.stderr.trim());const value=JSON.parse(line);expect(value.version).toBe(1);expect(value.mode).toBe(mode);expect(value.kind).toBe(kind);expect(value.fault).toBe(fault);return{result,value};}

registerBuildTestLawV1("inference reuse classifies only an exact leaf ENOENT as a missing inventory","sdk-runtime",async()=>{
 const db=new Database(":memory:");try{for(const row of corpus.errorCases){expect(new Ajv({strict:true}).validate(schema.definitions.ErrorClassificationCase,row)).toBe(true);expect(inferenceInventoryLeafMissingV1(row.code)).toBe(row.missing);expect(db.query<{missing:number},[string]>("select ?1='ENOENT' missing").get(row.code)?.missing===1).toBe(row.missing);}}finally{db.close();}
});

for(const kind of["source","lock"]as const)for(const row of corpus.cases.filter(row=>row.outcome==="refuse"))registerBuildTestLawV1("inference reuse refuses a "+row.fault+" "+kind+" inventory under managed custody","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"inference:refusals",180000),db=new Database(":memory:");try{const{result,value}=await childOutcome(original,phase,"refusal",kind,row.fault),expected=row.fault!=="permission"||value.permissionDenied;const diagnostic=result.stdout+result.stderr;expect(result.status,diagnostic).toBe(expected?1:0);expect(value.warm,diagnostic).toBe(true);expect(value.calls,diagnostic).toBe(1);expect(value.failed,diagnostic).toBe(expected);expect(value.predicateFailed,diagnostic).toBe(expected);if(expected)expect(value.samePredicate,diagnostic).toBe(true);if(row.fault==="cold-close")expect(value.samePhysicalPrimary).toBe(true);const oracle=value.oracle,sql=db.query<{outcome:string},[number,number,number,number,number]>("select case when ?1=0 or ?2=0 or ?3=0 or ?4=0 or ?5=0 then 'refuse' else 'reuse' end outcome").get(Number(oracle.readable),Number(oracle.content),Number(oracle.contract),Number(oracle.safe),Number(oracle.stable));expect(sql?.outcome).toBe(expected?"refuse":"reuse");if(["ancestor-absent","ancestor-replaced","leaf-disappears","absence-reappears","prior-leaf-disappears","cold-fifo","cold-close"].includes(row.fault))expect(value.mutated).toBe(true);}finally{db.close();}
});

for(const kind of["source","lock"]as const)for(const row of corpus.cases.filter(row=>row.outcome==="recompute"))registerBuildTestLawV1("inference reuse recomputes a "+row.fault+" "+kind+" inventory under managed custody","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"inference:recomputes",180000),db=new Database(":memory:");
 try{
  const{result,value}=await childOutcome(original,phase,"refusal",kind,row.fault),diagnostic=result.stdout+result.stderr,oracle=value.oracle;
  expect(result.status,diagnostic).toBe(0);expect(value.warm,diagnostic).toBe(true);expect(value.calls,diagnostic).toBe(2);expect(value.failed,diagnostic).toBe(false);expect(value.predicateFailed,diagnostic).toBe(false);
  expect(db.query<{outcome:string},[number,number,number,number]>("select case when ?1=0 or ?2=0 or ?3=0 or ?4=0 then 'unusable' else 'usable' end outcome").get(Number(oracle.readable),Number(oracle.content),Number(oracle.contract),Number(oracle.stable))?.outcome,diagnostic).toBe("unusable");
  if(["ancestor-replaced","absence-reappears"].includes(row.fault)){expect(value.mutated,diagnostic).toBe(true);expect(diagnostic).toContain("looking again 1 of");}
 }finally{db.close();}
});

registerBuildTestLawV1("inference reuse settles a replaced namespace under managed custody","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"inference:settled-ancestry",180000);
 const {result,value}=await childOutcome(original,phase,"refusal","source",corpus.cases.find(row=>row.id==="settled-ancestry-replacement")!.fault);
 const diagnostic=result.stdout+result.stderr;
 expect(result.status,diagnostic).toBe(0);
 expect(value.mutated,diagnostic).toBe(true);
 expect(value.warm,diagnostic).toBe(true);
 expect(value.calls,diagnostic).toBe(1);
 expect(value.predicateFailed,diagnostic).toBe(false);
 expect(value.failed,diagnostic).toBe(false);
 const db=new Database(":memory:");
 try{
  const {readable,content,contract,safe}=value.oracle;
  expect(db.query<{reuse:number},[number,number,number,number]>("select ?1 and ?2 and ?3 and ?4 reuse").get(+readable,+content,+contract,+safe)?.reuse).toBe(1);
 }finally{db.close();}
});

registerBuildTestLawV1("fixed graph fixtures isolate managed scopes and drain accepted children preserving falsey primaries","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"graph:fixture-lifetime",180000);let release!:()=>void,finished=false,seen=false;const pending=new Promise<void>(resolve=>{release=resolve;});let retained:undefined|{withConsumer:Function};const result=await withRepositoryBuildGraphFixtureV1(workspace,original,phase,async fixture=>{retained=fixture;void fixture.withConsumer(async()=>{seen=true;await pending;finished=true;});await expect(fixture.withMaintenance(async()=>null)).rejects.toThrow("Graph fixture has accepted readers or writers");queueMicrotask(release);return 42;});expect(result).toBe(42);expect(seen&&finished).toBe(true);await expect(retained?.withConsumer(async()=>null)).rejects.toThrow("Graph fixture lifetime ended");
 const receiverDatabase=new Database(":memory:");try{for(const row of receiverCorpus.receiverTransitions){expect(new Ajv({strict:true}).validate(fixtureSchema.definitions.ReceiverTransitionCase,row)).toBe(true);const oracle=receiverDatabase.query<{renewed:number;admitted:number},[string]>("select ?1 in('acquired','deferred') renewed,?1 in('acquired','deferred') admitted").get(row.outcome)!;expect({renewed:Number(row.renewed),admitted:Number(row.admitted)}).toEqual(oracle);}let disposedRoot="";await withRepositoryBuildGraphFixtureV1(workspace,original,phase,async fixture=>{disposedRoot=fixture.root;let allocation="",backing="",owner="",kind="",bytes=0,value:unknown;const receivers=new Set<object>(),capture=async()=>fixture.withConsumer(async(store,actual)=>{expect(store.image).toBe(fixture.image);const identity=JSON.stringify([store.storage.allocation.id,store.storage.backing.identity,store.storage.backing.root,store.storage.backing.maximumPhysicalBytes]);if(!allocation){allocation=store.storage.allocation.id;backing=identity;owner=store.storage.allocation.owner;kind=store.storage.allocation.kind;bytes=store.storage.allocation.bytes;}expect(store.storage.allocation.phase).toBe("committed");expect(store.storage.allocation.id).toBe(allocation);expect(identity).toBe(backing);expect(store.storage.allocation.owner).toBe(owner);expect<unknown>(store.storage.allocation.kind).toBe(kind);expect(store.storage.allocation.bytes).toBe(bytes);expect(owner).toBe("fixture:graph-consumer");expect(kind).toBe("artifact-publication");expect(bytes).toBe(8388608);await store.recheck();if(value!==undefined)expect(await inferenceInventoryReferencesHoldV1(workspace,value,actual)).toBe(true);});await fixture.withWriter(async()=>{value={namedInputs:{default:inventory("source")}};});await capture();for(let index=0;index<2;index++){const before=fixture.image;receivers.add(before);let issued:Parameters<typeof requireNativePlatformImageWriterV1>[1]|undefined;const result=await fixture.withMaintenance(async(image,custody)=>{expect(image).toBe(before);issued=custody;return index;});expect(result).toBe(index);expect(fixture.image).not.toBe(before);expect(()=>requireNativePlatformImageWriterV1(before,issued!)).toThrow();await capture();await fixture.withWriter(async()=>{expect(captureInferenceInventoryReferencesV1({namedInputs:{default:inventory("source")}})).toEqual(captureInferenceInventoryReferencesV1(value));});}const before=fixture.image,result=await fixture.probeMaintenanceDeferral();expect(new Ajv({strict:true}).validate(fixtureSchema.definitions.MaintenanceDeferralProbeResult,result)).toBe(true);expect(result).toEqual({consumed:true,acquired:false});expect(fixture.image).not.toBe(before);await fixture.withWriter(async()=>{expect(captureInferenceInventoryReferencesV1({namedInputs:{default:inventory("source")}})).toEqual(captureInferenceInventoryReferencesV1(value));});await capture();receivers.add(fixture.image);expect(receivers.size).toBe(3);});expect(existsSync(disposedRoot)).toBe(false);}finally{receiverDatabase.close();}

 let resume!:()=>void;const ended=new Promise<void>(resolve=>{resume=resolve;});let stale!:Promise<string>;await withRepositoryBuildGraphFixtureV1(workspace,original,phase,async outer=>{const outerPath=repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures");stale=(async()=>{await ended;return repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures");})();await outer.withConsumer(async(_store,actual)=>{await withRepositoryBuildGraphFixtureV1(workspace,original,actual,async inner=>{expect(inner.root).not.toBe(outer.root);expect(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures")).toBe(join(inner.root,"build-input-closures"));await inner.withConsumer(async()=>{await inner.withConsumer(async()=>{expect(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures")).toBe(join(inner.root,"build-input-closures"));});});});expect(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures")).toBe(outerPath);});expect(()=>repositoryBuildGraphDataDirectoryV1(workspace+"/other","build-input-closures")).toThrow("Graph fixture workspace identity required");});resume();await expect(stale).rejects.toThrow("Graph fixture lifetime ended");
 const roots=await Promise.all([0,1].map(()=>withRepositoryBuildGraphFixtureV1(workspace,original,phase,async fixture=>{await Promise.resolve();expect(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures")).toBe(join(fixture.root,"build-input-closures"));return fixture.root;})));expect(roots[0]).not.toBe(roots[1]);});

for(const kind of corpus.primaryFailures)registerBuildTestLawV1("fixed graph fixtures keep a "+kind+" primary first beneath accepted child failures","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"graph:fixture-primary",180000),{result,value}=await childOutcome(original,phase,"primary","source",kind);expect(result.status,result.stderr).toBe(1);expect(value.bodyStarted).toBe(true);expect(value.childFinished).toBe(true);expect(value.samePrimary).toBe(true);
});

for(const row of receiverCorpus.receiverTransitions.filter(row=>!row.admitted))registerBuildTestLawV1("fixed graph fixtures refuse admission after a "+row.outcome+" maintenance receiver","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"graph:fixture-maintenance",180000),{result,value}=await childOutcome(original,phase,"maintenance","source",row.outcome);expect(result.status,result.stderr).toBe(row.outcome==="cancel"?interruptedStatusV1:1);expect(value.bodyStarted).toBe(true);expect(value.maintenanceStarted).toBe(true);expect(value.admissionRefused).toBe(true);expect(value.childFinished).toBe(false);expect(value.samePrimary).toBe(true);
});

registerBuildTestLawV1("inference reuse preserves issued cancellation and deadlines after warm validation","sdk-runtime",async original=>{
 const phase=createIssuedResourceControlV1(original,"inference:terminal-reuse",180000);for(const kind of["cancel","deadline"]){const{result,value}=await childOutcome(original,phase,"terminal","lock",kind);expect(result.status,result.stderr).toBe(kind==="cancel"?interruptedStatusV1:1);expect(value.warm).toBe(true);expect(value.calls).toBe(1);expect(value.predicateFailed).toBe(true);expect(value.samePredicate).toBe(true);expect(value.failed).toBe(true);expect(value.message).toMatch(/deadline|abort|cancel|SIGINT|interrupt|terminat|expir|budget|timeout/iu);}
});

/** 🧘️ Waits for the session tail of the last ask: callers are answered when the batch ends, the session's post verification may still run (bounded overlap), so a census of its phases is taken once every phase is idle. */
const presenceSettledV1=async(control:Parameters<typeof inferenceInventoryPresenceStatsV1>[1]):Promise<void>=>{for(const until=Date.now()+60000;Object.values(inferenceInventoryPresenceStatsV1(workspace,control).timings).some(row=>row.active>0);){if(Date.now()>until)throw Error("Presence session tail did not settle within 60 s");await Bun.sleep(20);}};

/** 📊️ Exposes the measured Store phases in the slice report for a real managed inventory miss. */
registerBuildTestLawV1("inventory reader phase counters reach slice memory telemetry","sdk-runtime",async original=>{
 const control=createIssuedResourceControlV1(original,"inference:presence-timing",180000),ajv=new Ajv({strict:true});ajv.addSchema(schema);
 const validate=ajv.getSchema(schema.$id+"#/definitions/ConsumerTimings")!,validateNative=ajv.compile({...nativeTimingSchema,$ref:"#/definitions/Snapshot"});
 await withRepositoryBuildGraphFixtureV1(workspace,original,control,async fixture=>fixture.withConsumer(async(_store,actual)=>{
  const stop=registerSliceReportSourceV1("owners",()=>({presence:inferenceInventoryPresenceStatsV1(workspace,actual)}));
  try{
   const start=process.hrtime.bigint();
   const result=await inferenceInventoryReferencesHoldV1(workspace,{env:"SEMIO_SOURCE_CLOSURE_"+"e".repeat(64)},actual);
   await presenceSettledV1(actual);
   const elapsed=Math.ceil(Number(process.hrtime.bigint()-start)/1e6),{directory,name}=SLICE_REPORT_POLICY_V1;
   writeSliceReportV1(workspace,{reason:"interval"});
   const presence=JSON.parse(readFileSync(join(workspace,directory,name),"utf8")).sources.owners.presence,timings:Record<string,{started:number;active:number;milliseconds:number}>=presence.timings;
   expect(result).toBe(false);expect(validate(timings)).toBe(true);
   expect(presence.names).toBe(1);
   expect(Object.values(timings).every(row=>row.active===0&&row.started===1),JSON.stringify(timings)).toBe(true);
   expect(Object.values(timings).reduce((sum,row)=>sum+row.milliseconds,0)).toBeLessThanOrEqual(elapsed);
   expect(Object.keys(presence.nativeVerification)).toEqual(["preVerify","postVerify"]);
   const backends=Object.keys(presence.nativeVerification.preVerify);expect(backends).toHaveLength(1);
   for(const snapshot of Object.values(presence.nativeVerification)as Record<string,Record<string,{started:number;active:number;milliseconds:number}>>[]){
    expect(validateNative(snapshot)).toBe(true);expect(Object.keys(snapshot)).toEqual(backends);
    const phases=snapshot[backends[0]!]!;
    for(const phase of ["storeIdentity","reservationOperation","allocationJournal"])expect(phases[phase]?.started).toBe(1);
    expect(Object.values(phases).every(row=>row.active===0)).toBe(true);
    if(backends[0]==="directory-partition")expect(phases.allocatedUsage?.started).toBe(1);
    else expect((phases.physicalGateVerify?.started??0)+(phases.physicalGateJoin?.started??0)+(phases.physicalGateCached?.started??0)).toBeGreaterThan(0);
   }
  }finally{stop();}
 }));
});

registerBuildTestLawV1("inventory body operations reach the owned slice receipt","sdk-runtime",async original=>{
 const control=createIssuedResourceControlV1(original,"inference:presence-body-timing",180000),ajv=new Ajv({strict:true}),db=new Database(":memory:");
 ajv.addSchema(schema);
 const validate=ajv.getSchema(schema.$id+"#/definitions/BodyTimings")!;
 try{await withRepositoryBuildGraphFixtureV1(workspace,original,control,async fixture=>{
  let present:unknown;
  await fixture.withWriter(async()=>{present={namedInputs:{default:inventory("source")}};expect(existsSync(leaf(present).path)).toBe(true);});
  await fixture.withConsumer(async(_store,actual)=>{
   const sample=(value:unknown)=>inferenceInventoryReferencesHoldV1(workspace,value,actual);
   const stop=registerSliceReportSourceV1("owners",()=>({presence:inferenceInventoryPresenceStatsV1(workspace,actual)}));
   let previous:Record<string,number>={};
   try{for(const row of bodyCorpus.cases){
    expect(await sample(row.present?present:{env:"SEMIO_SOURCE_CLOSURE_"+"e".repeat(64)})).toBe(row.present);
    await presenceSettledV1(actual);
    const {directory,name}=SLICE_REPORT_POLICY_V1;
    writeSliceReportV1(workspace,{reason:"interval"});
    const presence=JSON.parse(readFileSync(join(workspace,directory,name),"utf8")).sources.owners.presence;
    const operations:Record<string,{started:number;active:number;milliseconds:number}>=presence.bodyOperations;
    expect(operations).toBeDefined();
    expect(validate(operations)).toBe(true);
    expect(Object.values(operations).every(op=>op.active===0)).toBe(true);
    expect(Object.values(operations).reduce((sum,op)=>sum+op.milliseconds,0)).toBeLessThanOrEqual(presence.timings.body.milliseconds);
    const counts=Object.fromEntries(Object.entries(row.expected).map(([key])=>[key,(operations[key]?.started??0)-(previous[key]??0)]));
    const oracle=db.query("SELECT 2 + ?1 AS storeRecheck, ?1 AS continuation, 3 + (1 - ?1) AS fileChecks, 1 - ?1 AS recordRead, 1 AS implementationCheck").get(Number(!row.present));
    expect(counts).toEqual(row.expected);
    expect(counts).toEqual(oracle);
    previous=Object.fromEntries(Object.entries(operations).map(([key,op])=>[key,op.started]));
   }}finally{stop();}
  });
 });}finally{db.close();}
});
