import{afterAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{classifySlotsV1}from"../../../../../🪢️slots/🟦️.ts";
import{fetchGraphEntryV1,graphEntrySlotV1,graphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{recordPlatformV1}from"../../../../../🪪️graph-state/🌍️platform-class/🟦️.ts";
import{hostOfRootV1,localRecordV1,neutralRecordV1}from"../../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import{createProjectFleetPortV1,projectRecordStateV1,runTakeLookV1,type ProjectRecordInfoV1}from"../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import{fleetAdmittedMegabytesV1,fleetGrantMegabytesV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{fleetReadV1}from"../../../../../🪪️graph-state/🙋️wanted/📖️read/🟦️.ts";
import observationSchema from"../../../../../👁️observation/🧬️schema/🔣️.json"with{type:"json"};
import{ownerPoolStateV1}from"../../🌐️owner-pool/🟦️.ts";
import{ownerJobV1,ownerRecordKeyTextV1,projectRecordKeyV1}from"../../🟨️.mjs";
import{TAKE_POLICY_V1,createTakeLedgerV1,takePassV1,takePollV1}from"../../📥️take/🟨️.mjs";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

/** 📥️ Contract: a reader takes what the fleet holds before it infers anything. Against a real cache server and real ports of several machines: a reader whose owners are all stored infers none, with nine tenths stored it infers exactly the tenth that is missing, an owner another machine stores while the reader polls is taken before the reader starts it, a stored record that no longer holds in the tree of a puller is refreshed by that puller and replaces the old one (the newest writer wins), a reader that finds a stored record stale replaces it with its own, a first answer of the cache that is a fault is asked again, the manifests of two waves never share a retention slot, the key of a record is the same for the pool, for the reader and for an independent Python model, and a too heavy owner is never left to the serving machine alone.
 * @see ../../📥️take/🟨️.mjs */
const scratch:string[]=[],here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`take-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the key of a record is the same for the pool and for the reader of a settlement round (the record stamp, not the revision of the observation module), on another machine, and equal to the independent Python model",async()=>{
 const revision=observationSchema.$defs.record.const.stamp,file=join(temp("key"),"fixture.json");
 writeFileSync(file,JSON.stringify({...fixture,revision}));
 const run=spawnSync("python3",["-I",here("./🐍️.py"),file],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {keys:string[];digests:string[];tooling:string[]};
 const observation=readFileSync(fileURLToPath(new URL("../../../../../👁️observation/🟨️.mjs",import.meta.url)),"utf8"),pool:string[]=[],reader:string[]=[],digests:string[]=[],tooling:string[]=[];
 expect(recordPlatformV1("x","darwin-arm64")).toBe(fixture.platform);
 expect(observation).toContain("keyText:JSON.stringify([recordStampV1(),null,key])");expect(observation).not.toContain("OBSERVATION_REVISION,null,key");expect(observation).toContain("Boolean(parent&&inferenceValidationScopes.getStore())");expect(observation).toContain("const recordStampV1=()=>sourceAnalysisSchemaV1.$defs.record.const.stamp");
 for(const row of fixture.cases){
  const selected=(row as {selectedTargets?:string[]}).selectedTargets,poolKey=projectRecordKeyV1(row.head,row.json.root,row.json,undefined,row.selectedInputs),readerKey=projectRecordKeyV1(row.head,row.json.root,row.json,selected,row.selectedInputs);
  pool.push(ownerRecordKeyTextV1(selected===undefined?poolKey:readerKey));
  const text=JSON.stringify([revision,null,readerKey]);reader.push(text);
  const state=projectRecordStateV1({root:row.workspace,fleet:row.fleet,keyText:text,platform:"darwin-arm64"});digests.push(state.digest);tooling.push(state.frame.tooling);
 }
 expect(pool).toEqual(oracle.keys);expect(reader).toEqual(oracle.keys);expect(digests).toEqual(oracle.digests);expect(tooling).toEqual(oracle.tooling);
 expect(digests[0]).toBe(digests[1]);expect(digests[0]).not.toBe(digests[2]);expect(digests[0]).not.toBe(digests[3]);
 expect(typeof ownerJobV1).toBe("function");
});

type Machine=ReturnType<typeof machine>;
const content=(label:string)=>label;
async function cache(){
 const store=temp("store"),server=await startRemoteCacheServerV1({directory:store,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"complete"});
 return server;
}
function machine(server:Awaited<ReturnType<typeof cache>>,label:string,tree:string,input:{now?:()=>number;fetch?:typeof fetch}={}){
 const root=temp("root-"+label),state=temp("state-"+label),local=new Map<string,unknown>(),client=graphFleetClientV1({endpoint:server.url,instance:"main",token:"w",upload:true,stateDirectory:state},input.fetch===undefined?undefined:{fetch:input.fetch});
 const path=(name:string)=>join(root,"libs",name,"src","a.ts"),write=(name:string,text:string)=>{mkdirSync(join(root,"libs",name,"src"),{recursive:true});writeFileSync(path(name),text);};
 const digestOf=(name:string)=>{try{return sha(readFileSync(path(name),"utf8"));}catch{return"gone";}};
 const portable=(name:string)=>({digest:sha(name),authority:[],observations:[{kind:"file",path:path(name),fact:{sha256:digestOf(name),size:2,mtimeMs:1,ctimeMs:1,ino:1}}],image:JSON.stringify({name,tree})});
 const holds=(name:string)=>{const row=local.get(name) as {observations:{path:string;fact:{sha256:string}}[]}|undefined;return row!==undefined&&row.observations.every(each=>sha(readFileSync(each.path,"utf8"))===each.fact.sha256);};
 const info=(name:string):ProjectRecordInfoV1=>({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,[["authority"],"libs/"+name,{name}],null,[]]),exportRecord:()=>portable(name),async import(record){local.set(name,record);return true;}});
 const port=createProjectFleetPortV1({root,environment:{},client,owner:label+":1",sleep:async milliseconds=>{for(const name of["down","denied"]){const file=join(state,name);try{const at=new Date(statSync(file).mtimeMs-milliseconds);utimesSync(file,at,at);}catch{}}},random:()=>0,unsentDirectory:join(state,"unsent"),...input.now===undefined?{}:{now:input.now}})!;
 const attempt=async(name:string):Promise<"present"|"fetched"|"stale"|"miss"|"fault">=>{
  if(holds(name))return"present";
  return runTakeLookV1(async look=>{const gate=await port.gate(info(name));if(look.outcome==="fault")return"fault";if(look.outcome==="miss")return"miss";return gate.seeded&&holds(name)?"fetched":"stale";});
 };
 const publish=async(name:string)=>{const answer=await port.claim(info(name));if(answer.state!=="granted")return answer.state;return await answer.publish()?"stored":"refused";};
 return{root,client,local,write,info,port,attempt,publish,holds,digestOf};
}
const names=(count:number)=>Array.from({length:count},(_,index)=>"o"+String(index).padStart(3,"0"));
const withTree=(m:Machine,list:readonly string[],text="v1")=>{for(const name of list)m.write(name,text);};

test("a reader whose owners are all stored in the fleet takes every one of them and infers nothing",async()=>{
 const server=await cache();
 try{
  const list=names(24),pool=machine(server,"pool",content("v1")),reader=machine(server,"reader",content("v1"));withTree(pool,list);withTree(reader,list);
  for(const name of list)expect(await pool.publish(name)).toBe("stored");
  const ledger=createTakeLedgerV1(list.length),inferred:string[]=[];
  const outcomes=await takePassV1({names:list,attempt:reader.attempt,mark:(name,outcome)=>ledger.mark(name,outcome)});
  for(const name of list)if(outcomes.get(name)==="miss"||outcomes.get(name)==="stale")inferred.push(name);
  expect(inferred).toEqual([]);expect([...outcomes.values()].every(outcome=>outcome==="fetched")).toBe(true);
  expect(ledger.counts()).toMatchObject({total:24,fetched:24,done:24,missing:0});
 }finally{await server.stop?.();}
},60000);

test("with nine tenths of the owners stored the reader takes those and infers exactly the missing tenth",async()=>{
 const server=await cache();
 try{
  const list=names(40),pool=machine(server,"pool",content("v1")),reader=machine(server,"reader",content("v1"));withTree(pool,list);withTree(reader,list);
  const stored=list.filter((_,index)=>index%10!==3);for(const name of stored)expect(await pool.publish(name)).toBe("stored");
  const ledger=createTakeLedgerV1(list.length),outcomes=await takePassV1({names:list,attempt:reader.attempt,mark:(name,outcome)=>ledger.mark(name,outcome)});
  const missing=list.filter(name=>outcomes.get(name)==="miss"||outcomes.get(name)==="stale");
  expect(missing).toEqual(list.filter((_,index)=>index%10===3));expect(missing.length).toBe(4);
  expect(ledger.counts()).toMatchObject({fetched:36,done:36,missing:4});
 }finally{await server.stop?.();}
},60000);

test("an owner another machine stores while the reader polls is taken within the poll bound, before the reader would start it",async()=>{
 const server=await cache();
 try{
  const list=names(12),pool=machine(server,"pool",content("v1")),reader=machine(server,"reader",content("v1"));withTree(pool,list);withTree(reader,list);
  for(const name of list.slice(0,6))await pool.publish(name);
  const first=await takePassV1({names:list,attempt:reader.attempt});const missing=list.filter(name=>first.get(name)==="miss");expect(missing).toEqual(list.slice(6));
  const fast={...TAKE_POLICY_V1,pollFirstMilliseconds:20,pollMaximumMilliseconds:80},arrivedAt=new Map<string,number>(),started=performance.now();let stopped=false;
  const wanted=[list[8]!,list[10]!];setTimeout(()=>{void Promise.all(wanted.map(name=>pool.publish(name)));},150);
  const poll=takePollV1({queue:()=>missing.filter(name=>!arrivedAt.has(name)),attempt:reader.attempt,arrived:name=>{arrivedAt.set(name,performance.now()-started);},stop:()=>stopped||arrivedAt.size===2,policy:fast});
  setTimeout(()=>{stopped=true;},3000);await poll;
  expect([...arrivedAt.keys()].sort()).toEqual(wanted);for(const at of arrivedAt.values())expect(at).toBeLessThan(150+80+600);
  const started2=missing.filter(name=>!arrivedAt.has(name));expect(started2.length).toBe(4);
 }finally{await server.stop?.();}
},60000);

test("a stored record that no longer holds in the tree of a puller is refreshed by that puller and replaces the old one; a record that holds stays stored",async()=>{
 const server=await cache();
 try{
  const first=machine(server,"first",content("v1")),puller=machine(server,"puller",content("v2"),{now:()=>Date.now()+600000}),reader=machine(server,"reader",content("v2"));
  withTree(first,["alpha","beta"],"v1");withTree(puller,["alpha"],"v2");withTree(puller,["beta"],"v1");withTree(reader,["alpha","beta"],"v2");reader.write("beta","v1");
  expect(await first.publish("alpha")).toBe("stored");expect(await first.publish("beta")).toBe("stored");
  expect(await reader.attempt("alpha")).toBe("stale");
  expect(await puller.port.claim(puller.info("beta"))).toMatchObject({state:"stored"});
  const refresh=await puller.port.claim(puller.info("alpha"));expect(refresh.state).toBe("granted");
  const second=machine(server,"second",content("v2"),{now:()=>Date.now()+600000});withTree(second,["alpha"],"v2");
  expect(await second.port.claim(second.info("alpha"))).toMatchObject({state:"held"});
  if(refresh.state==="granted"){expect(await refresh.publish()).toBe(true);}
  expect(await reader.attempt("alpha")).toBe("fetched");expect(await reader.attempt("beta")).toBe("fetched");
  expect(await puller.port.claim(puller.info("alpha"))).toMatchObject({state:"stored"});
  expect(puller.port.stats().refreshed).toBe(1);
 }finally{await server.stop?.();}
},60000);

test("a reader that finds a stored record stale and infers the owner itself replaces the record for everybody",async()=>{
 const server=await cache();
 try{
  const first=machine(server,"first",content("v1")),reader=machine(server,"reader",content("v2")),other=machine(server,"other",content("v2"));
  withTree(first,["alpha"],"v1");withTree(reader,["alpha"],"v2");withTree(other,["alpha"],"v2");
  expect(await first.publish("alpha")).toBe("stored");
  const gate=await reader.port.gate(reader.info("alpha"));expect(gate.seeded).toBe(true);expect(reader.holds("alpha")).toBe(false);
  reader.local.delete("alpha");gate.publish();await reader.port.flush();
  expect(await other.attempt("alpha")).toBe("fetched");
  expect(reader.port.stats().reinferred).toBe(1);
 }finally{await server.stop?.();}
},60000);

test("a first answer of the cache that is a fault is asked again and the owner is taken, not inferred",async()=>{
 const server=await cache();
 try{
  const first=machine(server,"first",content("v1"));withTree(first,["alpha"],"v1");expect(await first.publish("alpha")).toBe("stored");
  let left=1;const flaky=(async(url:string|URL|Request,init?:RequestInit)=>{if(left>0&&(init?.method??"GET")==="GET"&&new URL(String(url)).pathname.includes("/ac/")){left--;throw Object.assign(new Error("timed out"),{name:"TimeoutError"});}return fetch(url,init);}) as typeof fetch;
  const reader=machine(server,"reader",content("v1"),{fetch:flaky});withTree(reader,["alpha"],"v1");
  const gate=await reader.port.gate(reader.info("alpha"));
  expect(gate.seeded).toBe(true);expect(reader.port.stats().retried).toBe(1);expect(reader.port.stats().computed).toBe(0);
 }finally{await server.stop?.();}
},60000);

test("the manifests of two waves never share a retention slot, so a newer small wave does not collect an older big one",()=>{
 const one=ownerPoolStateV1("manifest","a".repeat(64)),two=ownerPoolStateV1("manifest","b".repeat(64)),pointer=ownerPoolStateV1("pointer");
 expect(graphEntrySlotV1(one)).not.toBe(graphEntrySlotV1(two));expect(graphEntrySlotV1(pointer)).not.toBe(graphEntrySlotV1(one));
 const verdicts=classifySlotsV1([{key:"a",slot:graphEntrySlotV1(one),storedAt:0,accessed:0,size:1},{key:"b",slot:graphEntrySlotV1(two),storedAt:20*60000,accessed:0,size:1}],{now:40*60000,graceMilliseconds:600000});
 expect(verdicts.get("a")?.state).toBe("current");expect(verdicts.get("b")?.state).toBe("current");
 const sameSlot=classifySlotsV1([{key:"a",slot:"s",storedAt:0,accessed:0,size:1},{key:"b",slot:"s",storedAt:20*60000,accessed:0,size:1}],{now:40*60000,graceMilliseconds:600000});
 expect(sameSlot.get("a")?.state).toBe("expired");
});

test("a too heavy owner is published as wanted and also inferred here, last, so it is never waited on the serving machine alone",async()=>{
 const server=await cache(),client=graphFleetClientV1({endpoint:server.url,instance:"main",token:"w",upload:true,stateDirectory:temp("state-heavy")});
 try{
  const stale=[...names(6),"@semio-tech/huge-rs","@semio-tech/giant-rs"],heavy=new Set(["@semio-tech/huge-rs","@semio-tech/giant-rs"]),handed:string[][]=[],lines:string[]=[];let sleeps=0;
  const result=await fleetReadV1({stale,admittedMegabytes:fleetAdmittedMegabytesV1({hostMegabytes:16384}),grantMegabytes:fleetGrantMegabytesV1({hostMegabytes:16384}),client,requester:"worker:1",platform:"darwin-arm64",graphKey:"g".repeat(64),language:"en",
   estimate:name=>({megabytes:heavy.has(name)?30000:300,source:"recorded"}),say:line=>lines.push(line),sleep:async()=>{sleeps++;await new Promise(done=>setTimeout(done,2));},
   infer:async run=>{handed.push([...run.names]);for(const name of run.names)if(!run.skip(name))run.done(name);}});
  expect(handed.length).toBe(1);expect(handed[0]!.slice(0,6).sort()).toEqual(names(6));expect(new Set(handed[0]!.slice(6))).toEqual(heavy);
  expect(result.outcome).toBe("served");expect(result.missing).toEqual([]);expect(result.solo).toBeUndefined();expect(sleeps).toBeLessThan(2000);
 }finally{await server.stop?.();}
},60000);

test("the provider ports of tool scripts are among the ids a taker registered, so the rows that name them are localized and hold; without them the token stays",async()=>{
 const inference=await import("../../../🟨️.mjs") as {libraryBootstrap:Promise<unknown>};await inference.libraryBootstrap;
 const analyses=await import("../../../🧬️source-analyses/🟨️.mjs") as {prepareSourceAnalysesV1():Record<string,{id:string}>},inputs=await import("../../../🧭️script-inputs/🟨️.mjs") as {scriptProviderIdsV1():readonly string[]};
 const ids=[...Object.values(analyses.prepareSourceAnalysesV1()).map(provider=>provider.id),...inputs.scriptProviderIdsV1()],ports=ids.find(id=>id.startsWith("build:script-provider-ports:v1:"));
 expect(ports).toBeDefined();
 const host=hostOfRootV1("/work/a"),local={digest:"d",authority:[],observations:[{kind:"analysis",path:"/work/a/x.ts",fact:{provider:ports,revision:"r",source:{size:1,mtimeMs:0,ctimeMs:0,ino:0,sha256:"s"}}}],image:"{}"},neutral=neutralRecordV1(local,host).record;
 expect(JSON.stringify(neutral)).toContain("\u27e6analysis-owner\u27e7");
 expect((localRecordV1(neutral,host,{providers:ids}).record.observations[0]!.fact as {provider:string}).provider).toBe(ports!);
 expect((localRecordV1(neutral,host,{providers:ids.filter(id=>id!==ports)}).record.observations[0]!.fact as {provider:string}).provider).toContain("\u27e6");
});

test("a code file changed or restamped anywhere in the library keeps a stored record fresh on the puller: freshness is the content of what the record read, never a modification time",async()=>{
 const server=await cache();
 try{
  const first=machine(server,"first",content("v1")),puller=machine(server,"puller",content("v1"),{now:()=>Date.now()+600000});
  withTree(first,["alpha"],"v1");withTree(puller,["alpha"],"v1");
  expect(await first.publish("alpha")).toBe("stored");
  expect(await puller.port.claim(puller.info("alpha"))).toMatchObject({state:"stored"});
  const code=join(puller.root,"🧰️framework","🔨️modules","tool","🟦️.ts");mkdirSync(join(code,".."),{recursive:true});writeFileSync(code,"export {}");const later=new Date(Date.now()+300000);utimesSync(code,later,later);rmSync(join(puller.root,".cache","f","owner-pool-fresh"),{recursive:true,force:true});
  const fresh=machine(server,"fresh",content("v1"),{now:()=>Date.now()+600000});withTree(fresh,["alpha"],"v1");
  expect(await fresh.port.claim(fresh.info("alpha"))).toMatchObject({state:"stored"});
  expect(await puller.port.claim(puller.info("alpha"))).toMatchObject({state:"stored"});
  expect(puller.port.stats().refreshed).toBe(0);
 }finally{await server.stop?.();}
},60000);
