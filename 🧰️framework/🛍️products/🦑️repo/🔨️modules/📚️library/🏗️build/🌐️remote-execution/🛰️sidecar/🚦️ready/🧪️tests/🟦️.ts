import{afterAll,expect,spyOn,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{findWorkspaceRoot}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const workspaceRootV1=findWorkspaceRoot(import.meta.dir);
import{READY_CONTAINMENT_POLICY_V1 as CONTAIN,readyContainmentStatusV1}from"../🌲️process-tree/🧹️containment/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import heartbeatFixture from"./💓️heartbeat/🔣️.json"with{type:"json"};
import{READY_TREE_POLICY_V1 as TREE}from"../🌲️process-tree/🟦️.ts";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,READY_STATUS_SCHEMA_V1,createReadyLeaseV1,createReadyPortsV1,createReadyStatusV1,readyWriteStoreFileV1,readyInputHashV1,readyReasonV1,readyRepoBinaryV1,emptyReadyStateV1,readyBackoffV1,readyDecideV1,readyDigestsV1,readyHumanActiveV1,readyHumanRecordV1,readyHumanVerdictV1,readyIsServerV1,readyJobsV1,readyRefusalV1,readyStoreFullV1,readyTreeMegabytesV1,runReadyStageV1,runReadyV1,type ReadyCodeV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1}from"../🟦️.ts";

/** 🚦️ Readiness that drives itself: a ready machine does nothing, a machine that is not ready runs its stages and records them, a failed stage waits for its back-off and is recorded, the group server never runs, and two runners on one machine never overlap. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`ready-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const fixtureFile=join(import.meta.dir,"🧫️fixtures","🔣️.json"),schemaFile=join(import.meta.dir,"..","🧬️schema","🔣️.json"),nativeSchemaFile=join(import.meta.dir,"../../../../../../🔩️native/🥾️bootstrap/🧬️schema/🔣️.json");

test("digests, back-off, jobs, tree size and the decision equal an independent implementation on the authored fixture",()=>{
 const fixture=JSON.parse(readFileSync(fixtureFile,"utf8")) as {digests:{platform:"linux"|"macos"|"windows";files:Record<string,string>}[];backoff:{attempts:number;unit:number}[];jobs:{cores:number;availableMegabytes:number|null}[];tree:{rows:string;root:number}[];decide:{name:string;input:Parameters<typeof readyDecideV1>[0]}[];human:{name:string;input:Parameters<typeof readyHumanVerdictV1>[0]}[]};
 const ours={
  digests:fixture.digests.map(row=>readyDigestsV1({platform:row.platform,hash:path=>row.files[path]})),
  backoff:fixture.backoff.map(row=>readyBackoffV1(row.attempts,row.unit)),
  jobs:fixture.jobs.map(row=>readyJobsV1(row)),
  tree:fixture.tree.map(row=>readyTreeMegabytesV1(row.rows,row.root)),
  decide:fixture.decide.map(row=>readyDecideV1(row.input)),
  human:fixture.human.map(row=>readyHumanVerdictV1(row.input)),
 };
 expect(ours.backoff.slice(0,7)).toEqual([30000,60000,120000,240000,480000,960000,1200000]);
 expect(ours.tree).toEqual([9,0]);
 expect(ours.human).toEqual([true,false,false,false,true,true,false,false]);
 expect(new Set(ours.digests.map(row=>row.digest)).size).toBe(4);
  expect(ours.digests[0]!.stages[0]!.digest).toBe(ours.digests[1]!.stages[0]!.digest);
 expect(ours.digests[0]!.stages[2]!.digest).not.toBe(ours.digests[1]!.stages[2]!.digest);
 expect(ours.digests[0]!.stages[3]!.digest,"a lock file edit touches only the stage that reads it").toBe(ours.digests[1]!.stages[3]!.digest);
 expect(ours.digests[0]!.stages[4]!.digest).toBe(ours.digests[1]!.stages[4]!.digest);
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🧫️fixtures","🐍️oracle.py"),fixtureFile,schemaFile,nativeSchemaFile],{encoding:"utf8"});
 expect(python.status).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual(JSON.parse(JSON.stringify(ours)));
});

test("selected Nx materials invalidate only workspace and a restarted observer reuses matching receipts",()=>{
 const fixture=JSON.parse(readFileSync(fixtureFile,"utf8")).workspaceInvalidation as {platforms:("linux"|"macos"|"windows")[];cases:{name:string;path:string|null;changed:string[]}[]};
 const result=fixture.platforms.flatMap(platform=>fixture.cases.map(row=>{
  const old=readyDigestsV1({platform,hash:()=>undefined}),current=readyDigestsV1({platform,hash:path=>path===row.path?"a".repeat(64):undefined});
  const changed=current.stages.filter((stage,index)=>stage.digest!==old.stages[index]!.digest).map(stage=>stage.name);
  expect(changed,row.name).toEqual(row.changed);
  const done=Object.fromEntries(old.stages.map(stage=>[stage.name,stage.digest]));
  const decision=readyDecideV1({server:false,...current,done,marker:true,repo:true,human:false,lease:"free",memoryOk:true,failure:null,now:1});
  expect(decision.action,row.name).toBe(row.changed.length?"run":"idle");
  expect(decision.stageIndex,row.name).toBe(STAGES.findIndex(stage=>stage.name===row.changed[0]));
  return {changed,decision};
 }));
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🧫️fixtures","🐍️oracle.py"),fixtureFile,schemaFile,nativeSchemaFile,"workspace"],{encoding:"utf8"});
 expect(python.status).toBe(0);
 expect(result).toEqual(JSON.parse(python.stdout).workspaceInvalidation);
});

test("the Ready workspace roster follows the unique patch selected by the Nx tool recipe",()=>{
 const workspace=STAGES.find(stage=>stage.name==="workspace")!;
 const recipe=workspace.inputs.find(path=>path.endsWith("/🚀️bootstrap/🛠️tools/package.json"))!;
 const manifest=JSON.parse(readFileSync(join(workspaceRootV1,recipe),"utf8")) as {semio:{toolPatches:Record<string,string>}};
 const patches=Object.entries(manifest.semio.toolPatches).filter(([name])=>name.startsWith("nx@")).map(([,path])=>path);
 expect(patches).toHaveLength(1);
 expect(workspace.inputs.filter(path=>/(?:^|\/)nx@[^/]+\.patch$/u.test(path))).toEqual(patches);
 expect(existsSync(join(workspaceRootV1,patches[0]!))).toBe(true);
});

test("Ready input hashing observes original bytes after stale path identity, miss and restart",async()=>{
 const root=temp("inputs"),file=join(root,"nx-material"),replacement=join(root,"next");
 const workspace=STAGES.find(stage=>stage.name==="workspace")!;
 const args={root,environment:{},origin:"g1-m97",group:"g1",stages:[{...workspace,inputs:["nx-material"]}],say:()=>undefined};
 const port=createReadyPortsV1(args),read=async()=>((await port.digests()).stages[0]!.digest);
 const absent=await read(),date=new Date(2026,0,1);
 writeFileSync(file,"alpha");utimesSync(file,date,date);
 const first=await read();expect(first).not.toBe(absent);
 expect(await read()).toBe(first);
 expect((await createReadyPortsV1(args).digests()).stages[0]!.digest).toBe(first);
 writeFileSync(replacement,"bravo");utimesSync(replacement,date,date);renameSync(replacement,file);
 const second=await read();expect(second).not.toBe(first);expect(second).not.toBe(absent);
 rmSync(file);expect(await read()).toBe(absent);
 mkdirSync(file);expect(await read()).toBe(absent);rmSync(file,{recursive:true});
 writeFileSync(file,"alpha");utimesSync(file,date,date);expect(await read()).toBe(first);
});

test("every refusal code has its text in English and German and the stage table is the order the bootstrap needs",()=>{
 const codes=schema.definitions.Refusal.properties.code.enum as ReadyCodeV1[];
 expect(Object.keys(schema.definitions.Texts.const.en).sort()).toEqual([...codes,"ready"].sort());
 expect(Object.keys(schema.definitions.Texts.const.de).sort()).toEqual([...codes,"ready"].sort());
 for(const code of codes){const refusal=readyRefusalV1(code,{stage:"tools",exit:7,available:100,needed:1536});expect(refusal.en).not.toBe(refusal.de);expect(refusal.en).not.toContain("{");expect(refusal.de).not.toContain("{");}
 expect(STAGES.map(stage=>stage.name)).toEqual(["native","repo-mcp","dependencies","tools","workspace"]);
 expect(STAGES.find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","all"]);
});

type Outcome=Partial<ReadyRunResultV1>;
function world(options:Readonly<{outcomes?:Outcome[];server?:boolean;human?:boolean;memory?:number|null;state?:ReadyLocalStateV1;marker?:boolean;repo?:boolean;stopAfterSleeps?:number;store?:string;leaseDirectory?:string;owner?:string;shared?:{state:ReadyLocalStateV1;marker:boolean;now:number;calls:string[];running:number;maximum:number}}>={}){
 const shared=options.shared??{state:options.state??emptyReadyStateV1(),marker:options.marker??false,now:1_000_000,calls:[],running:0,maximum:0};
 const controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],queue=[...options.outcomes??[]];
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)});
 const status=options.store===undefined?{update:(patch:Partial<ReadyStatusV1>)=>{updates.push(patch);}}:(()=>{const real=createReadyStatusV1({store:options.store,origin:"g1-m99",platform:"linux",now:()=>shared.now});return{update:(patch:Partial<ReadyStatusV1>)=>{updates.push(patch);real.update(patch);}};})();
 const ports:ReadyPortsV1={
  server:options.server??false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,2));if(sleeps.length>=(options.stopAfterSleeps??40)||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>options.repo??true,human:()=>options.human??false,
  lease:createReadyLeaseV1({directory:options.leaseDirectory??temp("lease"),owner:options.owner??"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:options.memory===undefined?8192:options.memory,ok:options.memory===undefined||options.memory===null||options.memory>=P.memoryFloorMegabytes}),
  run:async stage=>{
   shared.calls.push(stage.name);shared.running++;shared.maximum=Math.max(shared.maximum,shared.running);
   await new Promise<void>(done=>setTimeout(done,8));
   shared.running--;
   return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[],...queue.shift()};
  },
  status:{update:patch=>status.update(patch)},say:line=>{said.push(line);},
 };
 return{ports,controller,shared,sleeps,updates,said,digest,last:()=>Object.assign({},...updates) as Partial<ReadyStatusV1>};
}

test("a pending supervisor restart publishes the current wait after a workspace timeout",async()=>{
 const w=world({repo:true,stopAfterSleeps:1});
 const stages=Object.fromEntries(w.digest.stages.filter(row=>row.name!=="workspace").map(row=>[row.name,{digest:row.digest,doneMs:1}]));
 const failure={stage:"workspace",digest:w.digest.digest,attempts:1,exit:1,code:"stage-timeout" as const,atMs:10,nextAttemptMs:20,tail:[]};
 w.shared.state={...w.shared.state,stages,failure,attempts:1};
 await runReadyV1({...w.ports,restartPending:()=>true},w.controller.signal);
 expect(w.shared.calls).toEqual([]);
 expect(w.last()).toMatchObject({state:"waiting",stage:"workspace",refusal:{code:"sidecar-restarting"}});
 expect(w.shared.state.failure).toEqual(failure);
});

test("a ready machine does nothing: no stage, no marker write, status ready",async()=>{
 const probe=world({stopAfterSleeps:1}),done=Object.fromEntries(probe.digest.stages.map(stage=>[stage.name,{digest:stage.digest,doneMs:1}]));
 const w=world({stopAfterSleeps:1,marker:true,state:{...emptyReadyStateV1(),stages:done,readyMs:5}});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual([]);
 expect(w.last().state).toBe("ready");expect(w.last().ready).toBe(true);expect(w.last().digest).toBe(w.digest.digest);
});

test("a machine made ready by a person (marker and binary, no stage run here) adopts the marker and does nothing",async()=>{
 const w=world({stopAfterSleeps:1,marker:true,repo:true});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual([]);expect(w.shared.state.adopted).toBe(true);expect(w.last().ready).toBe(true);
});

test("a machine that is not ready runs every stage in order, records each, writes the marker last and ready.json validates against the schema",async()=>{
 const store=temp("store"),w=world({store,stopAfterSleeps:1,repo:false});
 let writes=0;const original=w.ports.marker.write;
 const ports:ReadyPortsV1={...w.ports,marker:{exists:w.ports.marker.exists,write:()=>{writes=w.shared.calls.length;original();}},repoPresent:()=>w.shared.calls.length>0};
 await runReadyV1(ports,w.controller.signal);
 expect(w.shared.calls).toEqual(STAGES.map(stage=>stage.name));
 expect(writes).toBe(STAGES.length);
 expect(w.shared.marker).toBe(true);
 expect(Object.keys(w.shared.state.stages)).toEqual(STAGES.map(stage=>stage.name));
 expect(w.shared.state.peakMegabytes).toBe(512);expect(w.shared.state.failure).toBeNull();
 const running=w.updates.filter(update=>update.state==="running").map(update=>update.stage);
 expect(running).toEqual(STAGES.map(stage=>stage.name));
 const file=JSON.parse(readFileSync(join(store,"v1","origins","g1-m99","ready.json"),"utf8")) as ReadyStatusV1;
 expect(validateJsonSchemaSubset(READY_STATUS_SCHEMA_V1,file,schema)).toEqual([]);
 expect(file.ready).toBe(true);expect(file.state).toBe("ready");expect(file.digest).toBe(w.digest.digest);expect(file.peakMegabytes).toBe(512);expect(file.readyMs).not.toBeNull();
});

test("a failed stage is recorded with both texts, waits for its back-off exactly, is retried, and a success clears the attempts",async()=>{
 const store=temp("store"),w=world({store,outcomes:[{},{},{},{exit:7,tail:["boom"]},{exit:7,tail:["boom again"]}],stopAfterSleeps:12});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls.slice(0,7)).toEqual(["native","repo-mcp","dependencies","tools","tools","tools","workspace"]);
 expect(w.sleeps.slice(0,2)).toEqual([30000,60000]);
 const failed=w.updates.filter(update=>update.state==="backoff");
 expect(failed.length).toBeGreaterThan(1);
 const first=failed[0]!;
 expect(first.refusal!.code).toBe("stage-failed");expect(first.refusal!.en).toContain("tools");expect(first.refusal!.en).toContain("7");expect(first.refusal!.de).toContain("Stufe tools");
 expect(first.attempts).toBe(1);expect(first.exit).toBe(7);expect(first.tail).toEqual(["boom"]);expect(first.nextAttemptMs!).toBeGreaterThan(1_000_000);
 expect(failed[failed.length-1]!.attempts).toBe(2);
 expect(w.shared.state.failure).toBeNull();expect(w.shared.state.attempts).toBe(0);
 expect(w.shared.marker).toBe(true);
});

test("a stage that ends with the retry exit of the repo command (75, run again) waits the first back-off every time instead of doubling it, and a real failure still doubles",async()=>{
 const w=world({outcomes:[{},{},{},{exit:75},{exit:75},{exit:75},{exit:7},{exit:7}],stopAfterSleeps:9});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.sleeps.slice(0,3)).toEqual([30000,30000,30000]);expect(w.sleeps[3]!).toBeGreaterThan(60000);
});

test("a failing machine never loops hot: with a frozen clock a failed stage runs once and waits",async()=>{
 const w=world({outcomes:Array.from({length:20},()=>({exit:1})),stopAfterSleeps:6});
 const frozen:ReadyPortsV1={...w.ports,sleep:async(milliseconds,signal)=>{w.sleeps.push(milliseconds);await new Promise<void>(done=>setTimeout(done,2));if(w.sleeps.length>=6||signal.aborted)w.controller.abort();}};
 await runReadyV1(frozen,w.controller.signal);
 expect(w.shared.calls).toEqual(["native"]);
 expect(w.sleeps.every(milliseconds=>milliseconds>0&&milliseconds<=P.backoffInitialMilliseconds)).toBe(true);
 expect(w.shared.state.failure!.code).toBe("stage-failed");
});

test("timeouts and spawn failures are recorded with their own code and the stage input changing retries at once",async()=>{
 const w=world({outcomes:[{exit:null,timedOut:true},{exit:null,spawnFailed:true}],stopAfterSleeps:4});
 await runReadyV1(w.ports,w.controller.signal);
 const codes=w.updates.filter(update=>update.state==="backoff").map(update=>update.refusal!.code);
 expect(codes[0]).toBe("stage-timeout");expect(codes).toContain("spawn-failed");
 const decided=readyDecideV1({server:false,digest:"e".repeat(64),stages:[{name:"native",digest:"1".repeat(64)}],done:{},marker:false,repo:false,human:false,lease:"free",memoryOk:true,failure:{digest:"d".repeat(64),nextAttemptMs:9_999_999},now:1});
 expect(decided.action).toBe("run");
});

test("the group server never runs a stage and writes nothing",async()=>{
 const store=temp("store"),w=world({server:true,store,stopAfterSleeps:1});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual([]);expect(w.updates).toEqual([]);expect(existsSync(join(store,"v1","origins","g1-m99","ready.json"))).toBe(false);
 expect(readyIsServerV1("g1-server","g1")).toBe(true);expect(readyIsServerV1("g1-m03","g1")).toBe(false);expect(readyIsServerV1("g1-server",undefined)).toBe(false);expect(readyIsServerV1("g2-server","g1")).toBe(false);
 expect(readyDecideV1({server:true,digest:"d".repeat(64),stages:[],done:{},marker:false,repo:false,human:false,lease:"free",memoryOk:true,failure:null,now:1}).action).toBe("server");
});

test("a person running the setup and too little free memory are observed or waited for, never fought",async()=>{
 const human=world({human:true,stopAfterSleeps:2});
 await runReadyV1(human.ports,human.controller.signal);
 expect(human.shared.calls).toEqual([]);expect(human.last().state).toBe("observing");expect(human.last().refusal!.code).toBe("human-running");expect(human.last().refusal!.de).toContain("Person");
 const low=world({memory:900,stopAfterSleeps:2});
 await runReadyV1(low.ports,low.controller.signal);
 expect(low.shared.calls).toEqual([]);expect(low.last().state).toBe("waiting");expect(low.last().refusal!.code).toBe("memory-low");expect(low.last().refusal!.en).toContain("900");
 const directory=temp("human");
 expect(readyHumanActiveV1(directory)).toBe(false);
 const file=join(directory,P.humanName),listed={commands:()=>["bash /home/w/native-bootstrap.sh /tmp/log"]},unlisted={commands:()=>["bash /home/w/other.sh"]};
 writeFileSync(file,"");
 expect(readyHumanActiveV1(directory,Date.now(),listed),"an empty marker counts while the older wrapper is in the process list").toBe(true);
 expect(readyHumanActiveV1(directory,Date.now(),unlisted),"an empty marker of a wrapper that is gone expires at once").toBe(false);
 expect(readyHumanActiveV1(directory,Date.now(),{commands:()=>{throw new Error("ps failed");}})).toBe(false);
 const old=new Date(Date.now()-P.humanFreshMilliseconds-5000);utimesSync(file,old,old);expect(readyHumanActiveV1(directory,Date.now(),listed)).toBe(false);
 const exited=spawnSync(process.execPath,["-e","0"]);expect(exited.status).toBe(0);
 const record=(pid:number,startedMs:number)=>JSON.stringify({version:1,pid,startedMs});
 writeFileSync(file,record(process.pid,Date.now()));
 expect(readyHumanActiveV1(directory,Date.now(),unlisted),"a live owner counts however the process list reads").toBe(true);
 writeFileSync(file,record(exited.pid,Date.now()));
 expect(readyHumanActiveV1(directory,Date.now(),listed),"the process that wrote the marker is gone: the fresh marker is ignored").toBe(false);
 writeFileSync(file,record(process.pid,Date.now()-P.humanMaximumMilliseconds-1000));
 expect(readyHumanActiveV1(directory,Date.now(),listed),"a live owner that started longer ago than the maximum no longer blocks, whatever the file times say").toBe(false);
 expect(readyHumanRecordV1("")).toBeNull();expect(readyHumanRecordV1("{\"version\":1,\"pid\":0,\"startedMs\":5}")).toBeNull();expect(readyHumanRecordV1("{\"version\":1,\"pid\":7,\"startedMs\":5}")).toEqual({pid:7,startedMs:5});
 expect(validateJsonSchemaSubset(schema.definitions.Human,JSON.parse(record(7,5)),schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Human,{version:1,pid:0,startedMs:5},schema)).not.toEqual([]);
});

test("two runners on one machine never overlap: one lease, every stage runs once, the loser observes",async()=>{
 const leaseDirectory=temp("lease2"),shared={state:emptyReadyStateV1(),marker:false,now:1_000_000,calls:[] as string[],running:0,maximum:0};
 const a=world({shared,leaseDirectory,owner:"a",stopAfterSleeps:6}),b=world({shared,leaseDirectory,owner:"b",stopAfterSleeps:6});
 await Promise.all([runReadyV1(a.ports,a.controller.signal),runReadyV1(b.ports,b.controller.signal)]);
 expect(shared.maximum).toBe(1);
 expect(shared.calls).toEqual(STAGES.map(stage=>stage.name));
 expect(shared.marker).toBe(true);
 const sawLoser=[a,b].some(runner=>runner.updates.some(update=>update.refusal?.code==="lease-held"));
 expect(sawLoser||a.updates.length+b.updates.length>0).toBe(true);
});

test("the lease: exclusive, renewed by its holder, taken over only when stale or its process is gone, released by its owner only",()=>{
 const directory=temp("lease3");let clock=1000;
 const make=(owner:string,alive=(_pid:number)=>true)=>createReadyLeaseV1({directory,owner,pid:owner==="a"?111:222,now:()=>clock,alive});
 const a=make("a"),b=make("b");
 expect(a.acquire()).toBe("own");expect(b.acquire()).toBe("other");expect(a.acquire()).toBe("own");expect(b.peek()).toBe("other");expect(a.peek()).toBe("own");
 b.release();expect(a.peek()).toBe("own");
 clock+=P.leaseStaleMilliseconds-1;expect(b.acquire()).toBe("other");
 expect(a.renew()).toBe(true);clock+=P.leaseStaleMilliseconds-1;expect(b.acquire()).toBe("other");
 clock+=2;expect(b.peek()).toBe("free");expect(b.acquire()).toBe("own");expect(a.renew()).toBe(false);expect(a.peek()).toBe("other");
 const dead=make("c",pid=>pid!==222);expect(dead.peek()).toBe("free");
 b.release();expect(b.peek()).toBe("free");
 expect(existsSync(join(directory,"takeover"))).toBe(false);
});

test("the status file is replaced atomically, validated, and a document that violates the schema is not written",()=>{
 const store=temp("status");let clock=5;
 const status=createReadyStatusV1({store,origin:"g1-m98",platform:"linux",now:()=>clock});
 status.update({state:"running",stage:"native"});
 const file=join(store,"v1","origins","g1-m98","ready.json");
 expect(JSON.parse(readFileSync(file,"utf8")).stage).toBe("native");
 clock=9;status.update({state:"ready",ready:true,stage:null,peakMegabytes:1.5 as unknown as number});
 expect(JSON.parse(readFileSync(file,"utf8")).state).toBe("running");
});

const posix=process.platform==="win32"?test.skip:test;
function fakeHome(script:string):{home:string;restore:()=>void}{
 const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});
 writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);
 return{home,restore:()=>undefined};
}
const stageOf=(name:string,timeoutMilliseconds?:number)=>({...STAGES.find(stage=>stage.name===name)!,...timeoutMilliseconds===undefined?{}:{timeoutMilliseconds}});

posix("a real stage runs the account's repo binary with its tools on PATH, the language, no cache override and a bounded job count, and keeps a log and a peak",async()=>{
 const {home,restore}=fakeHome('#!/bin/sh\necho "args=$*"\necho "locale=$LANG"\necho "lang=$SEMIO_LANGUAGE jobs=$CARGO_BUILD_JOBS make=$MAKEFLAGS root=$SEMIO_WORKSPACE_ROOT token=${SEMIO_REMOTE_CACHE_TOKEN:-none}"\necho "path=$PATH"\nexit 0\n');
 try{
  const root=temp("root");
  const result=await runReadyStageV1({home,root,environment:{PATH:"/usr/bin:/bin",SEMIO_REMOTE_CACHE_TOKEN:"secret",SEMIO_FLEET_GROUP:"g1"},stage:stageOf("tools"),signal:new AbortController().signal,jobs:3,onLine:()=>undefined});
  expect(result.exit).toBe(0);expect(result.timedOut).toBe(false);expect(result.spawnFailed).toBe(false);
  expect(result.tail[0]).toBe("args=tools setup all");
  expect(result.tail[1]).toBe(`locale=${process.platform==="darwin"?"en_US.UTF-8":"C.UTF-8"}`);
  expect(result.tail[2]).toBe(`lang=en jobs=3 make=-j3 root=${root} token=none`);
  expect(result.tail[3]!.startsWith(`path=${join(home,".local","bin")}`)).toBe(true);
  expect(readFileSync(join(root,...P.directory,"tools.log"),"utf8")).toContain("args=tools setup all");
  const german=await runReadyStageV1({home,root,environment:{PATH:"/usr/bin:/bin",SEMIO_LANGUAGE:"de"},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
  expect(german.tail[2]).toContain("lang=de");
 }finally{restore();}
});

posix("a real stage reports a failing exit and a missing binary, never throws",async()=>{
 const {home,restore}=fakeHome('#!/bin/sh\necho "no network"\nexit 9\n');
 try{
  const root=temp("root"),failed=await runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
  expect(failed.exit).toBe(9);expect(failed.tail).toEqual(["no network"]);
 }finally{restore();}
 const empty=temp("empty"),missing=await runReadyStageV1({home:empty,root:temp("root"),environment:{},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
 expect(missing.spawnFailed).toBe(true);expect(missing.exit).toBeNull();
});

posix("a stage outlives the runner that started it: aborting the signal detaches, a new runner adopts the same process and gets its result, and the cancel file ends the whole process group",async()=>{
 const pidFile=join(temp("pids"),"pid");
 const {home}=fakeHome(`#!/bin/sh\nif [ "$1" = run-many ]; then echo workspace-complete; exit 0; fi\n(sleep 30 & echo $! > "${pidFile}"; wait) &\nsleep 1\necho adopted-output\nwait\n`);
 const gone=(pid:number):boolean=>{try{process.kill(pid,0);return false;}catch{return true;}};
 const root=temp("root"),first=new AbortController(),lines:string[]=[];
 const run=(signal:AbortSignal,timeout?:number)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools",timeout),signal,jobs:1,onLine:line=>lines.push(line)});
 const running=run(first.signal);
 for(let waited=0;!existsSync(pidFile)&&waited<5000;waited+=25)await new Promise<void>(done=>setTimeout(done,25));
 const sleeper=Number(readFileSync(pidFile,"utf8").trim());
 first.abort();
 const detached=await running;
 expect(detached.detached).toBe(true);
 expect(gone(sleeper),"the stage keeps running after its runner left").toBe(false);
 const record=JSON.parse(readFileSync(join(root,...P.directory,P.inflightName),"utf8")) as {stage:string;pid:number};
 expect(record.stage).toBe("tools");
 const transitions:[string,boolean][]=[];
 const second=run(new AbortController().signal),other=runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,onStage:(stage,adopted)=>transitions.push([stage,adopted])});
 await new Promise<void>(done=>setTimeout(done,800));
 writeFileSync(join(root,...P.directory,P.cancelName),"");
 const cancelled=await second;
 expect(cancelled.cancelled).toBe(true);expect(cancelled.detached).toBe(false);expect(cancelled.exit).not.toBe(0);
 for(let waited=0;!gone(sleeper)&&waited<3000;waited+=50)await new Promise<void>(done=>setTimeout(done,50));
 const survivor=spawnSync("ps",["-p",String(sleeper),"-o","pid=,ppid=,pgid=,stat=,command="],{encoding:"utf8"});
 expect(gone(sleeper),"the cancel ended the whole process group: "+survivor.stdout.trim()).toBe(true);
 expect(existsSync(join(root,...P.directory,P.inflightName))).toBe(false);
 const after=await other;
 expect(after.exit).toBe(0);
 expect(transitions).toEqual([["tools",true],["workspace",false]]);
});

posix("a silent stage that spends processor time is alive and is not stopped as stalled, a silent stage that spends none is, and the kill is recorded with its killer in both languages",async()=>{
 const busy=fakeHome("#!/bin/bash\nwhile [ $SECONDS -lt 17 ]; do :; done\nexit 0\n"),idle=fakeHome("#!/bin/sh\nexec sleep 40\n"),rootBusy=temp("root"),rootIdle=temp("root");
 const moves:string[]=[];
 const run=(home:string,root:string)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onActivity:()=>moves.push(root),onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:11000}});
 const [alive,silent]=await Promise.all([run(busy.home,rootBusy),run(idle.home,rootIdle)]);
 expect(moves).toContain(rootBusy);
 expect(alive.stalled,"a busy compile is alive without any output").toBe(false);expect(alive.exit).toBe(0);
 expect(silent.stalled,"no output and no processor time is a stall").toBe(true);
 const record=readFileSync(join(rootIdle,".cache","f","ready",CONTAIN.killRecordName),"utf8");
 expect(record).toContain("received SIGTERM from the ready stage runner of this worker");expect(record).toContain("owner ready:");expect(record).toContain("showed no output and no processor time for 11 second(s)");expect(record).toContain(" / Prozess ");expect(record).toContain("weder Ausgabe noch Prozessorzeit");
 expect(existsSync(join(rootBusy,".cache","f","ready",CONTAIN.killRecordName))).toBe(false);
},120000);

posix("a stage that runs too long is stopped as timed out, with its group",async()=>{
 const pidFile=join(temp("pids"),"pid"),{home}=fakeHome(`#!/bin/sh\n(sleep 30 & echo $! > ${pidFile}; wait) &\nwait\n`);
 const gone=(pid:number):boolean=>{try{process.kill(pid,0);return false;}catch{return true;}};
 const timed=await runReadyStageV1({home,root:temp("root"),environment:{},stage:stageOf("tools",600),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
 expect(timed.timedOut).toBe(true);expect(timed.exit).not.toBe(0);
 const sleeper=Number(readFileSync(pidFile,"utf8").trim());
 for(let waited=0;!gone(sleeper)&&waited<3000;waited+=50)await new Promise<void>(done=>setTimeout(done,50));
 expect(gone(sleeper)).toBe(true);
});

test("the output of a stage and its timings are mirrored into the store: at most 200 lines and 64 KiB per stage, rotated by stage, timings capped, both validated",async()=>{
 const store=temp("mirror"),root=temp("mirror-root");
 const ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-m97",group:"g1",say:()=>undefined});
 const origin=join(store,"v1","origins","g1-m97");
 ports.mirror!("tools",Array.from({length:200},(_all,index)=>`line ${index} ${"x".repeat(600)}`));
 const text=readFileSync(join(origin,`${P.mirrorPrefix}tools.txt`),"utf8");
 expect(text.length).toBeLessThanOrEqual(P.mirrorBytes);expect(text).toContain("x".repeat(100));expect(text).toContain("Plattenfakten");expect(text.slice(0,text.indexOf("disk facts"))).toMatch(/x\n$/u);expect(text.trimEnd().split("\n").at(-1)).toContain(root);
 ports.mirror!("native",["a","b"]);
 expect(readFileSync(join(origin,`${P.mirrorPrefix}native.txt`),"utf8")).toContain("a\nb");
 for(let index=0;index<P.timingsKept+5;index++)ports.timing!({stage:"tools",startedMs:1000*index,endedMs:1000*index+5000,seconds:5,exit:index%2,outcome:index%2?"stage-failed":"ok",peakMegabytes:null,attempts:index%2,digest:"abc"});
 const timings=JSON.parse(readFileSync(join(origin,P.timingsName),"utf8")) as {runs:{startedMs:number}[]};
 expect(timings.runs.length).toBe(P.timingsKept);expect(timings.runs.at(-1)!.startedMs).toBe(1000*(P.timingsKept+4));
 expect(validateJsonSchemaSubset(schema.definitions.Timings,timings,schema)).toEqual([]);
});

posix("a real stage's output is mirrored while it runs and once more at its end, 200 lines at most",async()=>{
 const {home}=fakeHome('#!/bin/sh\ni=0\nwhile [ $i -lt 300 ]; do echo "line $i"; i=$((i+1)); done\nsleep 1\nexit 0\n');
 const seen:string[][]=[];
 const result=await runReadyStageV1({home,root:temp("root"),environment:{},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,mirror:(stage,lines)=>{expect(stage).toBe("tools");seen.push([...lines]);}});
 expect(result.exit).toBe(0);expect(result.startedMs).toBeGreaterThan(0);
 expect(seen.length).toBeGreaterThan(0);expect(seen.at(-1)!.length).toBe(P.mirrorLines);expect(seen.at(-1)!.at(-1)).toBe("line 299");
});

test("a native stage that ends 0 without leaving the repo binary is a failure with a back-off, not a loop; the binary is found in the launcher directory too",async()=>{
 const w=world({repo:false,stopAfterSleeps:1});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls).toEqual(["native"]);
 expect(w.shared.state.failure!.stage).toBe("native");expect(w.shared.state.failure!.code).toBe("stage-failed");
 expect(w.shared.marker).toBe(false);
 const home=temp("repohome"),local=temp("localappdata");
 expect(readyRepoBinaryV1(home,{},"linux")).toBeUndefined();
 mkdirSync(join(home,".local","share","semio","launcher","bin"),{recursive:true});writeFileSync(join(home,".local","share","semio","launcher","bin","repo"),"");
 expect(readyRepoBinaryV1(home,{},"linux")).toBe(join(home,".local","share","semio","launcher","bin","repo"));
 mkdirSync(join(local,"semio","launcher","bin"),{recursive:true});writeFileSync(join(local,"semio","launcher","bin","repo.exe"),"");
 expect(readyRepoBinaryV1(temp("emptyhome"),{LOCALAPPDATA:local},"win32")).toBe(join(local,"semio","launcher","bin","repo.exe"));
});

test("provisioning hashes track tool pins independently of descriptions and package scripts",()=>{
 const table=(pin:string,summary:string)=>JSON.stringify({groups:{all:["bun"]},tools:[{id:"bun",summary:{en:summary},pin:{format:"value",value:pin},locate:[],setup:{kind:"bun"}}],steps:[{id:"h",kind:"cargo-helpers",configuration:"prototype",platforms:["linux"],timeoutSeconds:1200,summary:{en:summary}}],messages:{texts:{a:summary}}});
 const path="x/🧰️tool-setup/🔣️.json";
 expect(readyInputHashV1(path,table("1.4.2","one"))).toBe(readyInputHashV1(path,table("1.4.2","another text")));
 expect(readyInputHashV1(path,table("1.4.2","one"))).not.toBe(readyInputHashV1(path,table("1.4.3","one")));
 const pkg=(extra:object)=>JSON.stringify({packageManager:"bun@1.4.2",engines:{node:"24"},workspaces:["a"],...extra});
 expect(readyInputHashV1("package.json",pkg({scripts:{a:"b"},dependencies:{x:"1"}}))).toBe(readyInputHashV1("package.json",pkg({})));
 expect(readyInputHashV1("package.json",pkg({packageManager:"bun@1.4.3"}))).not.toBe(readyInputHashV1("package.json",pkg({})));
 expect(readyInputHashV1("bun.lock","a")).not.toBe(readyInputHashV1("bun.lock","b"));
});

test("readiness invalidates only hosts that consume a changed provisioning input",()=>{
 const fixture=JSON.parse(readFileSync(fixtureFile,"utf8")).platformInputs as {systems:NodeJS.Platform[];changes:[string,string][]};
 const bootstrap="x/🥾️bootstrap/🧬️schema/🔣️.json",table="x/🧰️tool-setup/🔣️.json";
 const native=(change:string)=>{
  const assets=fixture.systems.map(platform=>JSON.stringify({tool:"node",version:platform==="linux"&&change==="linux-asset"?"2":"1",platform,arch:"arm64"}));
  return JSON.stringify({definitions:{WindowsNodeInstallation:{const:{environment:change==="node-installation"?"NEW":"OLD"}},Pins:{const:{assets:"$rows"}}}}).replace('"$rows"',"[\n"+assets.join(",\n")+"\n]");
 };
 const catalog=(change:string)=>JSON.stringify({groups:{all:["node"]},tools:[{id:"node",pin:change==="common-pin"?"2":"1",locate:[...(change==="node-locator"?[{kind:"bootstrap-node"}]:[]),{kind:"path",names:["node"],...(change==="node-locator"?{windowsProgramDirectories:["nodejs"]}:{})}],setup:{kind:change==="node-locator"?"bootstrap-node":"bootstrap"}}],steps:[]});
 const actual=fixture.changes.map(([change])=>fixture.systems.filter(platform=>[bootstrap,table].some((path,i)=>{
  const input=i===0?native:catalog;
  return readyInputHashV1(path,input(""),platform)!==readyInputHashV1(path,input(change),platform);
 })));
 const oracle=spawnSync("python3",["-c","import json,sys; f=json.loads(sys.argv[1]); print(json.dumps([[p for p in f['systems'] if target in (p,'all')] for _,target in f['changes']]))",JSON.stringify(fixture)],{encoding:"utf8"});
 expect(oracle.status).toBe(0);expect(actual).toEqual(JSON.parse(oracle.stdout));
 const compact=JSON.stringify(JSON.parse(native("")));
 expect(readyInputHashV1(bootstrap,native(""),"darwin")).not.toBe(readyInputHashV1(bootstrap,compact,"darwin"));
 expect(readyInputHashV1(bootstrap,native(""),"win32")).toBe(readyInputHashV1(bootstrap,compact,"win32"));
});

test("the Rustup launcher source is an input only of Windows native readiness",()=>{
 const path=JSON.parse(readFileSync(nativeSchemaFile,"utf8")).definitions.RustupInstallation.const.windowsProcess;
 for(const platform of["windows","macos","linux"] as const){
  const digest=(revision:string)=>readyDigestsV1({platform,hash:file=>file===path?revision:"shared"}).stages.map(stage=>stage.digest);
  const first=digest("first"),second=digest("second");
  expect(first.map((value,index)=>value!==second[index])).toEqual(STAGES.map(stage=>platform==="windows"&&stage.name==="native"));
 }
});

test("a different adopted stage is reported until the requested stage actually starts",async()=>{
 const fixture=JSON.parse(readFileSync(fixtureFile,"utf8")).adoptedTransition as {requested:string;physical:string;indices:number[]};
 const probe=world({stopAfterSleeps:1}),at=probe.digest.stages.findIndex(row=>row.name===fixture.requested);
 const prior=Object.fromEntries(probe.digest.stages.slice(0,at).map(row=>[row.name,{digest:row.digest,doneMs:1}]));
 const w=world({stopAfterSleeps:1,state:{...emptyReadyStateV1(),stages:prior,failure:{digest:"d".repeat(64),stage:"native",attempts:1,exit:1,code:"stage-failed",nextAttemptMs:0,atMs:1,tail:["earlier run ended"]}}});
 const observed:{stage:string|null|undefined;index:number|undefined;reason:string|undefined}[]=[];
 const starting:unknown[]=[];
 const capture=()=>{const status=w.last();observed.push({stage:status.stage,index:status.stageIndex,reason:status.reason?.code});};
 await runReadyV1({...w.ports,inflight:()=>fixture.physical,run:async(stage,options)=>{
  starting.push({tail:w.last().tail,exit:w.last().exit});
  capture();options.onStage?.(stage.name,false);capture();
  w.controller.abort();return{...await w.ports.run(stage,options),detached:true};
 }},w.controller.signal);
 expect(starting).toEqual([{tail:[],exit:null}]);
 const expected=[fixture.physical,fixture.requested];
 const oracle=spawnSync("python3",["-c","import json,sys; names=json.loads(sys.argv[1]); print(json.dumps([names.index(s)+1 for s in sys.argv[2:]]))",JSON.stringify(STAGES.map(row=>row.name)),...expected],{encoding:"utf8"});
 expect(oracle.status).toBe(0);expect(JSON.parse(oracle.stdout)).toEqual(fixture.indices);
 expect(observed.map(row=>row.stage)).toEqual(expected);
 expect(observed.map(row=>row.index)).toEqual(fixture.indices);
 expect(observed.map(row=>row.reason)).toEqual(["adopted","first-run"]);
});

test("ready.json says why a stage was started: first run, inputs changed, previous failed, adopted",async()=>{
 const first=world({stopAfterSleeps:1,outcomes:[{exit:1}]});
 await runReadyV1(first.ports,first.controller.signal);
 const reasons=first.updates.filter(update=>update.state==="running").map(update=>update.reason?.code);
 expect(reasons[0]).toBe("first-run");
 const again=world({stopAfterSleeps:3,state:first.shared.state});
 again.shared.now=first.shared.now+10*60_000;
 await runReadyV1(again.ports,again.controller.signal);
 expect(again.updates.find(update=>update.state==="running")!.reason!.code).toBe("previous-failed");
 const changed=world({stopAfterSleeps:1,state:{...emptyReadyStateV1(),stages:{native:{digest:"f".repeat(64),doneMs:1}}}});
 await runReadyV1(changed.ports,changed.controller.signal);
 const row=changed.updates.find(update=>update.state==="running")!.reason!;
 expect(row.code).toBe("inputs-changed");expect(row.en).toContain("native");expect(row.de).toContain("native");
 const adopted=world({stopAfterSleeps:1});
 await runReadyV1({...adopted.ports,inflight:()=>"native"},adopted.controller.signal);
 expect(adopted.updates.find(update=>update.state==="running")!.reason!.code).toBe("adopted");
 expect(readyReasonV1("binary-missing","tools").de).toContain("tools");
});

posix("a stage past its soft cap goes on while a counter of its log moves and is stopped as soon as the counters stand still, with every process below it",async()=>{
 const moving=fakeHome('#!/bin/sh\nfor i in 1 2 3 4 5 6 7 8; do echo "store-ownership:frontier:$((i*64))"; sleep 0.4; done\nexit 0\n');
 try{
  const root=temp("root"),result=await runReadyStageV1({home:moving.home,root,environment:{},stage:stageOf("tools",800),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,hardCapMultiplier:10,stallMilliseconds:2500}});
  expect(result.timedOut).toBe(false);expect(result.exit).toBe(0);
 }finally{moving.restore();}
 const pidFile=join(temp("pids"),"pid");
 const still=fakeHome(`#!/bin/sh\necho "store-ownership:frontier:64"\n(sleep 60 & echo $! > ${pidFile}; wait) &\nwait\n`);
 try{
  const root=temp("root"),began=Date.now(),result=await runReadyStageV1({home:still.home,root,environment:{},stage:stageOf("tools",800),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,hardCapMultiplier:10,stallMilliseconds:2500}});
  expect(result.timedOut).toBe(true);expect(Date.now()-began).toBeGreaterThan(3500);expect(Date.now()-began).toBeLessThan(20000);
  const sleeper=Number(readFileSync(pidFile,"utf8").trim());
  for(let waited=0;waited<3000;waited+=50){try{process.kill(sleeper,0);}catch{break;}await new Promise<void>(done=>setTimeout(done,50));}
  expect((()=>{try{process.kill(sleeper,0);return false;}catch{return true;}})(),"the sleeper below the stage is gone").toBe(true);
 }finally{still.restore();}
},60000);

test("the lease names its holder and is free at once when its pid belongs to another process now",()=>{
 const directory=temp("lease-holder");let clock=1000;
 const make=(owner:string,startOf:(pid:number)=>number|null)=>createReadyLeaseV1({directory,owner,pid:owner==="a"?111:222,startMs:owner==="a"?5000:6000,now:()=>clock,alive:()=>true,startOf});
 const a=make("a",()=>5000),b=make("b",pid=>pid===111?5000:6000);
 expect(a.acquire()).toBe("own");expect(b.peek()).toBe("other");expect(b.holder()).toEqual({pid:111,owner:"a",ageMilliseconds:0});
 const reused=make("c",()=>9_000_000);expect(reused.peek()).toBe("free");expect(reused.holder()).toBeUndefined();expect(reused.acquire()).toBe("own");
 expect(readyRefusalV1("lease-held",{holder:111}).en).toContain("process 111");expect(readyRefusalV1("lease-held",{holder:111}).de).toContain("Prozess 111");
});

test("a stalled stage is stopped and started again after its first pause of 30 s: no failure, no attempt, the reason in English and German in status, log line and the next start",async()=>{
 const store=temp("store"),w=world({store,outcomes:[{},{},{},{exit:null,stalled:true,tail:["Reading the project graph slice (4313 s): verification 3575 of 3575."]},{}],stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls.slice(0,6)).toEqual(["native","repo-mcp","dependencies","tools","tools","workspace"]);
 expect(w.sleeps).toEqual([30000,P.idlePollMilliseconds,P.idlePollMilliseconds,P.idlePollMilliseconds,P.idlePollMilliseconds,P.idlePollMilliseconds]);
 expect(w.updates.some(update=>update.state==="backoff")).toBe(false);
 expect(w.shared.state.failure).toBeNull();expect(w.shared.state.attempts).toBe(0);
 const stalled=w.updates.find(update=>update.refusal?.code==="stage-stalled")!;
 expect(stalled.refusal!.en).toContain("tools");expect(stalled.refusal!.en).toContain("10 minutes");expect(stalled.refusal!.de).toContain("Stufe tools");expect(stalled.refusal!.de).toContain("10 Minuten");expect(stalled.attempts).toBe(0);
 const line=w.said.find(text=>text.includes("showed nothing new"))!;expect(line).toContain("Stufe tools zeigte");
 const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");
 expect(restarted[1]!.reason!.code).toBe("stalled");expect(restarted[1]!.reason!.en).toContain("started again at once");expect(restarted[1]!.reason!.de).toContain("sofort neu gestartet");
 expect(w.shared.marker).toBe(true);
});

posix("a stage that only counts seconds is stopped with every process below it after the stall limit and reports stalled, one whose counter moves is not",async()=>{
 const moving=fakeHome('#!/bin/sh\nfor i in 1 2 3 4 5 6 7 8 9 10 11 12; do echo "Reading the project graph slice ($i s): verification $i of 50"; sleep 0.4; done\nexit 0\n');
 try{
  const root=temp("root"),result=await runReadyStageV1({home:moving.home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:3000}});
  expect(result.stalled).toBe(false);expect(result.exit).toBe(0);
 }finally{moving.restore();}
 const pidFile=join(temp("pids"),"pid");
 const counting=fakeHome(`#!/bin/sh\n(sleep 60 & echo $! > ${pidFile}; wait) &\ni=0\nwhile true; do i=$((i+1)); echo "Reading the project graph slice ($((i*2)) s): verification 3575 of 3575."; echo "[repo] still working after $((i*30)) s; last phase started: door.script"; sleep 0.3; done\n`);
 try{
  const root=temp("root"),began=Date.now(),result=await runReadyStageV1({home:counting.home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:3000}});
  expect(result.stalled).toBe(true);expect(result.timedOut).toBe(false);expect(Date.now()-began).toBeGreaterThan(3000);expect(Date.now()-began).toBeLessThan(25000);
  const sleeper=Number(readFileSync(pidFile,"utf8").trim());
  for(let waited=0;waited<3000;waited+=50){try{process.kill(sleeper,0);}catch{break;}await new Promise<void>(done=>setTimeout(done,50));}
  expect((()=>{try{process.kill(sleeper,0);return false;}catch{return true;}})(),"the sleeper below the stage is gone").toBe(true);
 }finally{counting.restore();}
},90000);

posix("a long exact dependency binding refusal keeps all three paths in the failed stage mirror",async()=>{
 const fixturePath=join(import.meta.dir,"🧫️fixtures","🔗️binding-log.json");
 const fixture=JSON.parse(readFileSync(fixturePath,"utf8"));
 const {prefix,binding,ordinary,ordinaryLength}=fixture;
 const raw=prefix+JSON.stringify(binding),noise=ordinary+"x".repeat(ordinaryLength);
 expect(raw.length).toBeGreaterThan(P.mirrorLineCharacters);
 const home=fakeHome("#!/bin/sh\nprintf '%s\\n' '"+raw+"' '"+noise+"'\nexit 1\n");
 try{
  const result=await runReadyStageV1({home:home.home,root:temp("binding-log"),environment:{},stage:stageOf("dependencies",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
  expect(result.exit).toBe(1);
  const rows=result.failure??[],diagnostic=rows.find(line=>line.startsWith(prefix))??"";
  const independent=spawnSync("python3",["-I","-c","import json,sys; fixture=json.load(open(sys.argv[1],encoding='utf-8')); print(json.dumps(json.loads(sys.stdin.read().removeprefix(fixture['prefix'])) == fixture['binding']))",fixturePath],{input:diagnostic,encoding:"utf8"});
  expect(independent.status,independent.stderr).toBe(0);
  expect(JSON.parse(independent.stdout)).toBe(true);
  expect(rows.find(line=>line.startsWith(ordinary))?.length).toBe(P.mirrorLineCharacters);
 }finally{home.restore();}
});

posix("a failed stage mirror keeps the first error lines whole and the last lines when a flood of other lines follows",async()=>{
 const diagnostic='[DATA] Store dependency installer failed {"version":1,"purpose":"current-dependency-installer-failure","status":1,"stderrBytes":188,"stdoutSha256":"'+"b".repeat(64)+'","stderrSha256":"'+"a".repeat(64)+'","stderrTail":"Resolving dependencies\\nResolved, downloaded and extracted 9\\nerror: EPERM: Operation not permitted, rename"}';
 expect(diagnostic.length).toBeGreaterThan(P.mirrorLineCharacters);
 const home=fakeHome("#!/bin/sh\nprintf '%s\\n' 'Internal error: first cause EPERM at start' '"+diagnostic+"'\ni=0\nwhile [ $i -lt 400 ]; do i=$((i+1)); echo \"ordinary line $i\"; done\necho 'last words'\nexit 1\n");
 try{
  const result=await runReadyStageV1({home:home.home,root:temp("first-errors"),environment:{},stage:stageOf("dependencies",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
  expect(result.exit).toBe(1);
  const rows=result.failure??[];
  expect(rows).toContain("Internal error: first cause EPERM at start");
  expect(rows).toContain(diagnostic);
  expect(rows.some(line=>/earlier lines left out/u.test(line))).toBe(true);
  expect(rows).toContain("ordinary line 400");expect(rows).toContain("last words");
  expect(rows.length).toBeLessThanOrEqual(P.mirrorLines+2);
  const independent=spawnSync("python3",["-I","-c","import sys; rows=sys.stdin.read().split('\\n'); print(sum(1 for r in rows if r.startswith('[DATA] Store dependency installer failed ') and len(r)>400))"],{input:rows.join("\n"),encoding:"utf8"});
  expect(independent.stdout.trim()).toBe("1");
 }finally{home.restore();}
});

test("a stage that failed because the disk or the quota is full is a store-full refusal naming the path in English and German, a plain failure stays stage-failed",async()=>{
 const full="Internal error: No space left (EDQUOT) in /home/deck/.cache/semio/leases: the disk or the quota of the user holding it is full; free space there. / Kein Platz mehr (EDQUOT) in /home/deck/.cache/semio/leases: der Datenträger oder die Quote des Benutzers ist voll; dort Platz freigeben.";
 expect(readyStoreFullV1(["a",full,"repository:command failed with exit code 1"])).toEqual({errno:"EDQUOT",path:"/home/deck/.cache/semio/leases"});
 expect(readyStoreFullV1(["Internal error: unable to open database file"])).toBeNull();
 expect(readyStoreFullV1(["Internal error: No space left (ENOSPC) in C:\\Users\\x\\leases: the disk"])).toEqual({errno:"ENOSPC",path:"C:\\Users\\x\\leases"});
 const w=world({store:temp("store"),outcomes:[{},{},{},{exit:1,tail:['[script] {"activity":{"resource":"dependencies:selected-plan:7"}}',full,"repository:command failed with exit code 1"]}],stopAfterSleeps:1});
 await runReadyV1(w.ports,w.controller.signal);
 const refused=w.updates.find(update=>update.state==="backoff")!;
 expect(refused.refusal!.code).toBe("store-full");expect(w.shared.state.failure!.code).toBe("store-full");
 expect(refused.refusal!.en).toContain("/home/deck/.cache/semio/leases");expect(refused.refusal!.en).toContain("EDQUOT");expect(refused.refusal!.en).toContain("tools");
 expect(refused.refusal!.de).toContain("/home/deck/.cache/semio/leases");expect(refused.refusal!.de).toContain("Stufe tools");expect(refused.refusal!.de).toContain("Quote");
 const plain=world({store:temp("store"),outcomes:[{},{},{},{exit:1,tail:["Internal error: unable to open database file"]}],stopAfterSleeps:1});
 await runReadyV1(plain.ports,plain.controller.signal);
 expect(plain.updates.find(update=>update.state==="backoff")!.refusal!.code).toBe("stage-failed");
});

test("a stage stopped because it waited on a holder while its code changed starts again at once: no failure, no attempt, no back-off, its own refusal and reason in both languages",async()=>{
 const w=world({store:temp("store"),outcomes:[{},{},{},{exit:null,stalled:true,codeChanged:"🧰️framework/🔨️modules/🏃️process/x.ts",tail:["Process 48352 (x show projects) holds the dependency store since 2026-11-14T16:34:21.027Z; waiting for it (700 s of at most 21600 s)"]},{}],stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls.slice(0,6)).toEqual(["native","repo-mcp","dependencies","tools","tools","workspace"]);
 expect(w.updates.some(update=>update.state==="backoff")).toBe(false);expect(w.shared.state.failure).toBeNull();expect(w.shared.state.attempts).toBe(0);
 const stopped=w.updates.find(update=>update.refusal?.code==="stage-code-changed")!;
 expect(stopped.refusal!.en).toContain("10 minutes");expect(stopped.refusal!.en).toContain("x.ts");expect(stopped.refusal!.de).toContain("10 Minuten");expect(stopped.refusal!.de).toContain("neuen Code");
 expect(w.said.find(text=>text.includes("while its code changed"))).toContain("Stufe tools wartete");
 const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");
 expect(restarted[1]!.reason!.code).toBe("code-changed");expect(restarted[1]!.reason!.en).toContain("new code");expect(restarted[1]!.reason!.de).toContain("neuen Code");
});

posix("a stage that waits on a named holder is stopped with its tree only when a declared semantics source moved after it started; an edit of other code lets it wait",async()=>{
 const holder='#!/bin/sh\ni=0\nwhile [ $i -lt 40 ]; do i=$((i+1)); echo "Process 48352 (x show projects) holds the dependency store since 2026-11-14T16:34:21.027Z; waiting for it ($i s of at most 21600 s)"; echo "[script] {\\"activity\\":{\\"resource\\":\\"dependencies:maintenance-parent-observation\\",\\"mode\\":\\"shared\\",\\"elapsedMs\\":1.$i}}"; sleep 0.3; done\nexit 0\n';
 const policy={...TREE,holderWaitRestartMilliseconds:1500,codeCheckMilliseconds:300,semanticsSources:[{file:"code/semantics.json",pointer:"/revision"}],stallStopMilliseconds:600000},declare=(directory:string,revision:number)=>{mkdirSync(join(directory,"code"),{recursive:true});writeFileSync(join(directory,"code","semantics.json"),JSON.stringify({revision}));};
 const quiet=fakeHome(holder),quietRoot=temp("root");declare(quietRoot,1);writeFileSync(join(quietRoot,"code","a.ts"),"x");
 const unchanged=await runReadyStageV1({home:quiet.home,root:quietRoot,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 expect(unchanged.stalled).toBe(false);expect(unchanged.codeChanged).toBeUndefined();expect(unchanged.exit).toBe(0);
 const busy=fakeHome(holder),root=temp("root");declare(root,1);
 const began=Date.now(),pending=runReadyStageV1({home:busy.home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 setTimeout(()=>{writeFileSync(join(root,"code","b.ts"),"y");declare(root,2);},700);
 const changed=await pending;
 expect(changed.stalled).toBe(true);expect(changed.codeChanged).toBe("code/semantics.json");expect(changed.timedOut).toBe(false);expect(Date.now()-began).toBeGreaterThan(1500);expect(Date.now()-began).toBeLessThan(25000);
},90000);

posix("a stage whose only moving values are elapsedMs floats is stopped as stalled after the stall limit, one whose resource counter grows is not",async()=>{
 const flat=fakeHome('#!/bin/sh\ni=0\nwhile true; do i=$((i+1)); echo "[script] {\\"activity\\":{\\"resource\\":\\"dependencies:maintenance-parent-observation\\",\\"mode\\":\\"shared\\",\\"elapsedMs\\":1.$i}}"; sleep 0.3; done\n');
 const began=Date.now(),stuck=await runReadyStageV1({home:flat.home,root:temp("root"),environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:3000}});
 expect(stuck.stalled).toBe(true);expect(Date.now()-began).toBeGreaterThan(3000);expect(Date.now()-began).toBeLessThan(25000);
 const growing=fakeHome('#!/bin/sh\ni=48000\nwhile [ $i -lt 48040 ]; do i=$((i+1)); echo "[script] {\\"activity\\":{\\"resource\\":\\"javascript-selected-payload:$i\\",\\"mode\\":\\"shared\\",\\"elapsedMs\\":1.5}}"; sleep 0.2; done\nexit 0\n');
 const moving=await runReadyStageV1({home:growing.home,root:temp("root"),environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:3000}});
 expect(moving.stalled).toBe(false);expect(moving.exit).toBe(0);
},90000);

test("the status file is still written when the rename over the old file is refused (Windows with an open reader): the new document replaces it directly and no temporary file is left",()=>{
 const store=temp("status-rename");let clock=5,refused=0;
 const status=createReadyStatusV1({store,origin:"g1-m97",platform:"windows",now:()=>clock,rename:()=>{refused++;throw Object.assign(Error("EPERM: operation not permitted, rename"),{code:"EPERM"});}});
 status.update({state:"running",stage:"tools"});
 const directory=join(store,"v1","origins","g1-m97");
 expect(JSON.parse(readFileSync(join(directory,"ready.json"),"utf8")).stage).toBe("tools");
 clock=9;status.update({state:"running",stage:"workspace"});
 expect(JSON.parse(readFileSync(join(directory,"ready.json"),"utf8")).stage).toBe("workspace");
 expect(refused,"the refused rename was really attempted, twice").toBe(2);expect(readdirSync(directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
});

test("what a stage run reaped and what survived its stop is in ready.json in English and German until the next run, and the file still validates",async()=>{
 const store=temp("store"),containment={reaped:[{pid:91672,name:"bun.exe",kind:"registry" as const}],survivors:[{pid:610,startMs:1,name:"stubborn.exe"}]};
 const w=world({store,outcomes:[{},{},{},{exit:null,stalled:true,containment},{}],stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 const stalled=w.updates.find(update=>update.refusal?.code==="stage-stalled")!;
 expect(stalled.containment).toEqual(readyContainmentStatusV1({stage:"tools",...containment}));
 expect(stalled.containment!.en).toContain("stubborn.exe (610)");expect(stalled.containment!.en).toContain("bun.exe (91672)");expect(stalled.containment!.de).toContain("Stufe tools");expect(stalled.containment!.de).toContain("stubborn.exe (610)");
 const running=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");expect(running[1]!.containment).toBeNull();
 const directory=join(store,"v1","origins","g1-m99"),status=createReadyStatusV1({store,origin:"g1-m99",platform:"linux",now:()=>5});
 status.update({state:"waiting",stage:"tools",containment:stalled.containment});
 expect(JSON.parse(readFileSync(join(directory,"ready.json"),"utf8")).containment.survivors).toEqual([{pid:610,name:"stubborn.exe"}]);
 status.update({state:"waiting",containment:{...stalled.containment!,survivors:[{pid:0,name:"x"}]}});
 expect(JSON.parse(readFileSync(join(directory,"ready.json"),"utf8")).containment.survivors,"a document that violates the schema is not written").toEqual([{pid:610,name:"stubborn.exe"}]);
});

const BUN_LAUNCH=`const {spawn}=require("node:child_process");const c=spawn(process.execPath,[__dirname+"/grandchild.js",...process.argv.slice(2)],{detached:true,stdio:"ignore"});c.unref();`;
const BUN_GRANDCHILD=`const fs=require("node:fs");const [folder,command,directory,session]=process.argv.slice(2);const startedAtMs=Math.round(performance.timeOrigin);const entry={version:1,pid:process.pid,startedAtMs,updatedAtMs:Date.now(),command,directory,...session?{session}:{},...process.env.SEMIO_LEASE_OWNER?{owner:process.env.SEMIO_LEASE_OWNER}:{},holds:[{resource:"native-fixed-image:x",mode:"shared",sinceMs:Date.now(),task:1}],waits:[]};fs.mkdirSync(folder,{recursive:true});fs.writeFileSync(folder+"/"+process.pid+"-"+startedAtMs+".json",JSON.stringify(entry));setInterval(()=>{if(!fs.existsSync(__filename))process.exit(0);},250);setTimeout(()=>process.exit(0),125000);`;
const toolsDirectory=(label:string):string=>{const directory=temp(label);writeFileSync(join(directory,"launch.js"),BUN_LAUNCH);writeFileSync(join(directory,"grandchild.js"),BUN_GRANDCHILD);return directory;};
const registryEntryV1=(folder:string):{pid:number;startedAtMs:number}|undefined=>{try{return JSON.parse(readFileSync(join(folder,readdirSync(folder)[0]!),"utf8")) as {pid:number;startedAtMs:number};}catch{return undefined;}};
const gone=async(pid:number,milliseconds:number=4000):Promise<boolean>=>{for(let waited=0;waited<milliseconds;waited+=50){try{process.kill(pid,0);}catch{return true;}await new Promise<void>(done=>setTimeout(done,50));}return false;};

posix("a detached test grandchild exits when its private fixture directory closes",async()=>{
 const tools=toolsDirectory("orphan-lifetime"),registry=temp("orphan-lifetime-registry"),started=Date.now();
 const run=spawnSync(process.execPath,[join(tools,"launch.js"),registry,"test-child",tools],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 let child=registryEntryV1(registry);
 while(!child&&Date.now()-started<5000){
  await Bun.sleep(20);
  child=registryEntryV1(registry);
 }
 expect(child).toBeDefined();
 const pid=child!.pid;
 try{
  rmSync(tools,{recursive:true,force:true});
  expect(await gone(pid,4000)).toBe(true);
 }finally{try{process.kill(pid,"SIGKILL");}catch{}}
},15000);

posix("a stage that starts a detached grandchild whose parent exits is stopped without it, and the next stage start reaps it by pid and start time and says so in both languages, while a process that started in a person's working window is left alone",async()=>{
 const tools=toolsDirectory("tools"),registry=temp("registry"),root=temp("root"),command="fed0aa7c7c00cedaa8abc3ae7c9a032a8655b5ef9d0d37095feddf0e08b75442 show projects",lease=`${root}/.cache/b/l/m/data`;
 const files={orphan:join(registry,"aaaa"),agent:join(registry,"bbbb"),person:join(registry,"cccc")};
 const script=`#!/bin/sh\nsleep 0.7\n"${process.execPath}" ${tools}/launch.js ${files.orphan} "${command}" ${lease}\n"${process.execPath}" ${tools}/launch.js ${files.agent} "${command}" ${lease} claude-code:s1\nsleep 1.5\n"${process.execPath}" ${tools}/launch.js ${files.person} "${command}" ${lease}\nwhile true; do echo "Reading the project graph slice (1 s): verification 3575 of 3575."; sleep 0.3; done\n`;
 const fake=fakeHome(script);
 try{
  const began=Date.now(),first=await runReadyStageV1({home:fake.home,root,environment:{},stage:stageOf("dependencies",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,stallStopMilliseconds:5000}});
  const ended=Date.now();
  expect(first.stalled).toBe(true);expect(Date.now()-began).toBeLessThan(40000);
  const found=Object.values(files).map(folder=>registryEntryV1(folder)?.pid);expect(found.every(pid=>pid!==undefined),JSON.stringify(found)).toBe(true);const pids=found as number[];
  const [orphan,agent,person]=pids as [number,number,number];
  for(const pid of pids)expect(()=>process.kill(pid,0),`pid ${pid} survived the stop of its stage: it is a detached grandchild of an exited parent`).not.toThrow();
  const personEntry=registryEntryV1(files.person)!;
  const said:string[]=[],second=await runReadyStageV1({home:fake.home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:line=>said.push(line),tree:{...TREE,stallStopMilliseconds:2500},orphans:{registryRoot:registry,windows:()=>[{stage:"dependencies",startedMs:first.startedMs,endedMs:ended}],human:()=>({startedMs:personEntry.startedAtMs+1000})}});
  expect(await gone(orphan),"the orphan is reaped before the next stage").toBe(true);
  expect(await gone(agent),"an entry with a session address is judged like any other").toBe(true);expect(()=>process.kill(person,0),"a process that started in the working window of a person is never reaped").not.toThrow();
  expect(second.containment!.reaped.map(row=>[row.pid,row.kind]).sort()).toEqual([[orphan,"registry"],[agent,"registry"]].sort());
  const line=said.find(text=>text.includes(`Stopped orphan ${orphan}`))!;expect(line).toContain(" / Verwaisten Prozess ");
  expect(readFileSync(join(root,".cache","f","ready","tools.log"),"utf8"),"the reap is in the log of the stage").toContain(`Stopped orphan ${orphan}`);
 }finally{
  for(const folder of Object.values(files)){
   const child=registryEntryV1(folder);
   if(child)try{process.kill(child.pid,"SIGKILL");}catch{}
  }
  fake.restore();
 }
},120000);

posix("a stage that waits on a holder that is an orphan of a stopped try reaps it at once instead of waiting",async()=>{
 const tools=toolsDirectory("holder"),registry=temp("registry"),root=temp("root"),command="fed0aa7c7c00cedaa8abc3ae7c9a032a8655b5ef9d0d37095feddf0e08b75442 show projects",lease=`${root}/.cache/b/l/m/data`,file=join(registry,"aaaa");
 const script=`#!/bin/sh\ni=0\nwhile [ $i -lt 40 ]; do i=$((i+1)); [ $i -eq 3 ] && "${process.execPath}" ${tools}/launch.js ${file} "${command}" ${lease}; echo "Process 91672 (x show projects) holds the dependency store since 2026-11-14T16:34:21.027Z; waiting for it ($i s of at most 21600 s)"; sleep 0.4; done\nexit 0\n`;
 const fake=fakeHome(script);let orphan:number|undefined;
 try{
  const began=Date.now(),said:string[]=[],result=await runReadyStageV1({home:fake.home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:line=>said.push(line),tree:{...TREE,stallStopMilliseconds:600000},containment:{...CONTAIN,holderReapMilliseconds:500},orphans:{registryRoot:registry,windows:()=>{try{return[{stage:"tools",startedMs:(JSON.parse(readFileSync(join(root,".cache","f","ready","running.json"),"utf8")) as {startedMs:number}).startedMs,endedMs:began+120000}];}catch{return[];}},human:()=>null}});
  orphan=registryEntryV1(file)?.pid;expect(orphan).toBeDefined();
  expect(result.exit).toBe(0);expect(await gone(orphan!,1000),"reaped while the stage was waiting").toBe(true);
  expect(result.containment!.reaped).toEqual([{pid:orphan!,name:expect.any(String),kind:"holder"}]);
  const line=said.find(text=>text.includes("was waiting for process"))!;expect(line).toContain(`process ${orphan}`);expect(line).toContain(" / Stufe tools wartete auf Prozess ");
 }finally{fake.restore();if(orphan!==undefined)try{process.kill(orphan,"SIGKILL");}catch{}}
},120000);

test("a store file is written whole even where the rename over the old file is refused, and no temporary file is left (the mirrors of g1-m05 stayed at 17:24 for hours)",()=>{
 const directory=temp("store-file"),file=join(directory,"ready-log-tools.txt");
 readyWriteStoreFileV1(file,"one\n");expect(readFileSync(file,"utf8")).toBe("one\n");
 let refused=0;readyWriteStoreFileV1(file,"two\n",()=>{refused++;throw Object.assign(Error("EPERM"),{code:"EPERM"});});
 expect(refused).toBe(1);expect(readFileSync(file,"utf8")).toBe("two\n");expect(readdirSync(directory)).toEqual(["ready-log-tools.txt"]);
});

posix("a stage past its soft cap whose log keeps showing new lines is not stopped as timed out although none of them is a declared progress counter; a standing line is (g1-m04 and g1-m05 hit their cap inside counters that the cap did not know)",async()=>{
 const growing=fakeHome('#!/bin/sh\ni=0\nwhile [ $i -lt 24 ]; do i=$((i+1)); echo "[script] {\\"activity\\":{\\"resource\\":\\"dependencies:workspace-edges:$((i*64))\\",\\"mode\\":\\"exclusive\\",\\"elapsedMs\\":0}}"; sleep 0.3; done\nexit 0\n');
 const policy={...TREE,stallMilliseconds:3500,stallStopMilliseconds:600000};
 const moving=await runReadyStageV1({home:growing.home,root:temp("root"),environment:{},stage:stageOf("tools",2500),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 expect(moving.timedOut).toBe(false);expect(moving.exit).toBe(0);
 const standing=fakeHome('#!/bin/sh\nwhile true; do echo "[script] {\\"activity\\":{\\"resource\\":\\"native-artifact-input-stage\\",\\"mode\\":\\"exclusive\\",\\"elapsedMs\\":0}}"; sleep 0.3; done\n');
 const began=Date.now(),stuck=await runReadyStageV1({home:standing.home,root:temp("root"),environment:{},stage:stageOf("tools",2500),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 expect(stuck.timedOut).toBe(true);expect(Date.now()-began).toBeLessThan(25000);
},90000);

test("a stage that is stopped as silent again and again restarts after 30, 60, 120, 240 and 480 seconds, then is a failure with its reason and a back-off, and never loops without a pause",async()=>{
 const w=world({outcomes:Array.from({length:6},()=>({exit:null,stalled:true})),stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.sleeps.slice(0,5)).toEqual([30000,60000,120000,240000,480000]);
 expect(w.shared.calls.length).toBe(6);
 expect(w.shared.state.failure?.code).toBe("stage-stall-exhausted");
 const refusals=w.updates.map(update=>update.refusal).filter(row=>row!==undefined&&row!==null);
 expect(refusals.some(row=>row!.code==="stage-stalled"&&row!.en.includes("30 seconds")&&row!.de.includes("30 Sekunden")),"the first stop names its pause in both languages").toBe(true);
 expect(refusals.at(-1)!.code).toBe("stage-stall-exhausted");expect(refusals.at(-1)!.en).toContain("after 5 restarts");expect(refusals.at(-1)!.de).toContain("nach 5 Neustarts");
 expect(w.last().state).toBe("backoff");
});

test("a stop in a row is forgotten by a stage that ends: a stalled stage that then succeeds starts its next series at the first pause",async()=>{
 const w=world({outcomes:[{exit:null,stalled:true},{exit:null,stalled:true},{exit:0}],stopAfterSleeps:5,repo:true});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.sleeps.slice(0,2)).toEqual([30000,60000]);expect(w.shared.state.failure).toBeNull();
});

test("a stage stopped because it had run long while its code changed starts again at once: no failure, no attempt, no back-off, its own refusal and reason in both languages",async()=>{
 const w=world({store:temp("store"),outcomes:[{},{},{},{exit:null,stalled:true,codeChanged:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts",codeStale:true,tail:["Reading the project graph slice (13930 s): taken 1657 of 4455."]},{}],stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls.slice(0,6)).toEqual(["native","repo-mcp","dependencies","tools","tools","workspace"]);
 expect(w.updates.some(update=>update.state==="backoff")).toBe(false);expect(w.shared.state.failure).toBeNull();expect(w.shared.state.attempts).toBe(0);
 const stopped=w.updates.find(update=>update.refusal?.code==="stage-code-stale")!;
 expect(stopped.refusal!.en).toContain("30 minutes");expect(stopped.refusal!.en).toContain("load/");expect(stopped.refusal!.de).toContain("30 Minuten");expect(stopped.refusal!.de).toContain("neuen Code");
 expect(w.said.find(text=>text.includes("had run for at least"))).toContain("Stufe tools lief mindestens");
 const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");
 expect(restarted[1]!.reason!.code).toBe("code-stale");expect(restarted[1]!.reason!.en).toContain("30 minutes");expect(restarted[1]!.reason!.de).toContain("30 Minuten");expect(restarted[1]!.reason!.de).toContain("neuen Code");
});

test("a stopped current attempt replaces the historical failure tail while it waits to restart",async()=>{
 const {historical:previous,latest:current,provenance,expectedExit}=await import("./🧷️failure-log/🔣️.json",{with:{type:"json"}}).then(row=>row.default);
 const probe=world(),stages=Object.fromEntries(probe.digest.stages.filter(stage=>stage.name!=="workspace").map(stage=>[stage.name,{digest:stage.digest,doneMs:1}]));
 const state={...emptyReadyStateV1(),stages,attempts:1,failure:{digest:probe.digest.digest,stage:"workspace",attempts:1,exit:1,signal:null,code:"stage-failed" as const,nextAttemptMs:0,atMs:0,tail:[previous]}};
 const mirrors=new Map<string,readonly string[]>(),failure=[provenance,current];
 const w=world({state,outcomes:[{exit:null,tail:[current],failure,stalled:true}],stopAfterSleeps:1});
 await runReadyV1({...w.ports,mirror:(stage,lines)=>{mirrors.set(stage,lines);}},w.controller.signal);
 const waiting=w.updates.find(update=>update.refusal?.code==="stage-stalled");
 expect(waiting?.tail).toEqual([current]);expect(waiting?.exit).toBe(expectedExit);
 expect(mirrors.get("workspace-stalled")).toEqual(failure);
 expect(w.shared.state.failure?.tail).toEqual([previous]);
});

test("a quiet stage refreshes its running receipt only on owned movement and respects cancellation",async()=>{
 const store=temp("heartbeat-store"),w=world(),{stage}=heartbeatFixture;
 const origin="g1-m99",file=join(store,"v1","origins",origin,"ready.json"),started=w.shared.now,publications:number[]=[];
 const status=createReadyStatusV1({store,origin,platform:"linux",now:()=>w.shared.now,rename(from,to){renameSync(from,to);publications.push(w.shared.now-started);}});
 const receipt=()=>JSON.parse(readFileSync(file,"utf8")) as ReadyStatusV1;
 let active:ReadyStatusV1|undefined,silentAge:number|undefined;
 await runReadyV1({...w.ports,status,run:async(_stage,options)=>{
  for(const at of stage.movementsMs){w.shared.now=started+at;options.onActivity?.();}
  active=receipt();
  w.shared.now=started+stage.endedMs;
  silentAge=w.shared.now-receipt().updatedMs;
  w.controller.abort();
  options.onActivity?.();
  return{exit:null,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:true,startedMs:started,peakMegabytes:null,tail:[]};
 }},w.controller.signal);
 expect(publications).toEqual(stage.expectedPublicationsMs);
 expect(active?.updatedMs).toBe(started+stage.expectedPublicationsMs.at(-1)!);
 expect([active?.state,active?.ready,active?.tail]).toEqual(["running",false,[]]);
 expect(silentAge).toBeGreaterThan(TREE.foregroundFreshMilliseconds);
 status.update({state:"ready",ready:true,stage:null});
 const verified=receipt().updatedMs;
 w.shared.now+=TREE.foregroundFreshMilliseconds+1;
 status.heartbeat?.();
 expect(receipt().updatedMs).toBe(verified);
});

test("a long loop backoff renews only its stated wait up to the original retry deadline",async()=>{
 const store=temp("backoff-store"),w=world(),{loopBackoff,heartbeatMs}=heartbeatFixture;
 const receipts:ReadyStatusV1[]=[];
 let failures=0,backoffStarted=0;
 const status=createReadyStatusV1({store,origin:"g1-m99",platform:"linux",now:()=>w.shared.now,rename(from,to){
  renameSync(from,to);
  if(failures!==loopBackoff.attempt)return;
  receipts.push(JSON.parse(readFileSync(to,"utf8")) as ReadyStatusV1);
 }});
 await runReadyV1({...w.ports,status,
  digests:async()=>{
   failures++;
   if(failures===loopBackoff.attempt)backoffStarted=w.shared.now;
   if(failures>loopBackoff.attempt)w.controller.abort();
   throw new Error("fixture refused");
  },
  sleep:async(milliseconds)=>{w.shared.now+=milliseconds;},
 },w.controller.signal);
 expect(w.shared.now-backoffStarted).toBe(loopBackoff.pauseMs);
 expect(receipts.every(doc=>doc.state==="backoff"&&!doc.ready&&doc.refusal?.code==="loop-failed")).toBe(true);
 expect([...new Set(receipts.map(doc=>doc.nextAttemptMs))]).toEqual([backoffStarted+loopBackoff.pauseMs]);
 const offsets=receipts.map(doc=>doc.updatedMs-backoffStarted);
 expect(offsets).toHaveLength(loopBackoff.expectedPublications);
 expect(offsets.at(-1)).toBe(loopBackoff.expectedLastMs);
 expect(offsets.every((at,index)=>at===index*heartbeatMs)).toBe(true);
});

test("disk recovery renews only its scoped wait and mirrors actual progress",async()=>{
 const fixture=heartbeatFixture.diskRecovery;
 const oracle=spawnSync("python3",["-I","-c",`import json,sys
fixture=json.loads(sys.argv[1])
written=[0]
for event in fixture["events"]:
 period=fixture["heartbeatMs"] if event["action"]=="beat" else fixture["intervalMs"]
 if event["at"]-written[-1]>=period:written.append(event["at"])
print(json.dumps(written))`,JSON.stringify(fixture)],{encoding:"utf8"});
 expect(oracle.status).toBe(0);
 expect(JSON.parse(oracle.stdout)).toEqual(fixture.expectedPublicationsMs);
 for(const cancel of [false,true]){
  const probe=world(),stages=Object.fromEntries(probe.digest.stages.filter(row=>row.name!==fixture.stage).map(row=>[row.name,{digest:row.digest,doneMs:1}]));
  const w=world({state:{...emptyReadyStateV1(),stages}}),store=temp("disk-heartbeat"),origin="g1-m99",file=join(store,"v1","origins",origin,"ready.json"),started=w.shared.now,receipts:ReadyStatusV1[]=[];
  const read=()=>JSON.parse(readFileSync(file,"utf8")) as ReadyStatusV1;
  const status=createReadyStatusV1({store,origin,platform:"linux",now:()=>w.shared.now,rename(from,to){renameSync(from,to);const doc=read();if(doc.refusal?.code==="disk-collecting")receipts.push(doc);}});
  const interval=globalThis.setInterval,clear=globalThis.clearInterval;
  const token={unref:()=>{}} as ReturnType<typeof setInterval>;
  let pulse:(()=>void)|undefined,report:((line:string)=>void)|undefined,cleared=false,recovered=false,ran=false;
  const timer=spyOn(globalThis,"setInterval").mockImplementation(((handler:()=>void,period:number)=>{
   if(period!==P.statusHeartbeatMilliseconds)return interval(handler,period);
   pulse=handler;return token;
  })as typeof setInterval);
  const cancelTimer=spyOn(globalThis,"clearInterval").mockImplementation(((handle:ReturnType<typeof setInterval>)=>{
   if(handle===token){cleared=true;return;}
   clear(handle);
  })as typeof clearInterval);
  try{
   await runReadyV1({...w.ports,status,disk:()=>recovered?null:{path:"C:\\semio\\.cache",resource:"space",needed:fixture.needed,free:fixture.free},recover:async request=>{
    report=request.report;expect(request.signal).toBe(w.controller.signal);
    for(const event of fixture.events){w.shared.now=started+event.at;if(event.action==="report")request.report(event.line!);else pulse?.();}
    if(cancel){w.controller.abort();request.report("late after cancellation");}
    recovered=true;return{fits:true,consumers:[]};
   },run:async(_stage,options)=>{
    ran=true;expect(cleared).toBe(true);
    w.shared.now=started+fixture.closedAt;
    pulse?.();report?.("late after recovery");
    expect(read().tail).toEqual([]);
    w.controller.abort();
    return{exit:null,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:true,startedMs:w.shared.now,peakMegabytes:null,tail:[]};
   }},w.controller.signal);
  }finally{timer.mockRestore();cancelTimer.mockRestore();}
  expect(receipts.map(row=>row.updatedMs-started)).toEqual(fixture.expectedPublicationsMs);
  expect(receipts.every(row=>row.state==="waiting"&&!row.ready&&row.stage===fixture.stage)).toBe(true);
  expect(receipts.at(-1)?.tail).toEqual([fixture.events.find(row=>row.action==="report")!.line!]);
  expect(cleared).toBe(true);expect(ran).toBe(!cancel);
  expect(w.said).not.toContain("late after cancellation");
  expect(w.said).not.toContain("late after recovery");
 }
});

test("late movement from a failed stage cannot renew its replacement's running receipt",async()=>{
 const w=world(),store=temp("replacement-store"),origin="g1-m99",file=join(store,"v1","origins",origin,"ready.json");
 const status=createReadyStatusV1({store,origin,platform:"linux",now:()=>w.shared.now});
 let oldActivity:(()=>void)|undefined,observed:ReadyStatusV1|undefined,replacementStarted=0;
 await runReadyV1({...w.ports,status,sleep:async(milliseconds)=>{w.shared.now+=milliseconds;},run:async(_stage,options)=>{
  const result={timedOut:false,spawnFailed:false,cancelled:false,stalled:false,startedMs:w.shared.now,peakMegabytes:null,tail:[]};
  if(oldActivity===undefined){oldActivity=options.onActivity;return{...result,exit:7,detached:false};}
  options.onStage?.("tools",true);
  replacementStarted=w.shared.now;
  w.shared.now+=heartbeatFixture.heartbeatMs;
  oldActivity();
  observed=JSON.parse(readFileSync(file,"utf8")) as ReadyStatusV1;
  w.controller.abort();
  return{...result,exit:null,detached:true};
 }},w.controller.signal);
 expect([observed?.state,observed?.stage,observed?.updatedMs]).toEqual(["running","tools",replacementStarted]);
});
