import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {stretchedBudgetMillisecondsV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {Database} from "bun:sqlite";
import {appendFileSync,realpathSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,truncateSync,utimesSync,writeFileSync,copyFileSync,cpSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {blobPathV1,appendEventsV1,formatHlcV1,readEventSpansV1,readHeadV1,writeBlobV1,type FleetEventV1} from "../../📒️journal/🟦️.ts";
import {codeDigestV1,createAttesterV1,fleetReportV1,judgeFleetV1,readAttestationV1,skewSecondsV1,type FleetAttestationV1} from "../../🩺️attest/🟦️.ts";
import {listCodeSnapshotsV1,readHealthV1,restartJitterMillisecondsV1,sealCodeV1} from "../../🛡️resilience/🟦️.ts";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";
import {closeReplicaV1,compactV1,followOnceV1,foldStoreV1,homedOriginsV1,joinV1,openReplicaV1,OriginTakenV1,publishOnceV1,relayOnceV1,ReplicaBusyV1,ClockAheadV1,runV1,seedV1,snapshotV1,statusV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";
import {writeAttestationV1,attestPathV1} from "../../🩺️attest/🟦️.ts";
import {writeHintV1} from "../../🎣️tiers/🟦️.ts";
import outboundTail from "./🧫️fixtures/outbound-tail.json" with {type:"json"};
import fixture from "../../🩺️attest/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import {readProcessStartV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`resilience-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=20000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[] as string[]};
const headsOf=(heads:Readonly<Record<string,Readonly<{segment:number;bytes:number}>|undefined>>):Record<string,Readonly<{segment:number;bytes:number}>>=>Object.fromEntries(Object.entries(heads).flatMap(([origin,head])=>head===undefined?[]:[[origin,head]]));
const code=(label:string):string=>{const root=temp(label);put(root,"x/🟦️.ts","export const a=1;\n");return root;};
type Runner={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];controller:AbortController;done?:Promise<void>};
function runner(name:string,store:string,extra:Partial<ReplicaOptionsV1>={}):Runner{
 return {name,events:[],controller:new AbortController(),options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast,codeRoot:code(name),gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),...extra}};
}
const start=(who:Runner):void=>{who.done=runV1(who.options,who.controller.signal,event=>who.events.push(event));};
const stop=async(who:Runner):Promise<void>=>{who.controller.abort();await who.done;};
async function seeded(name:string,extra:Partial<ReplicaOptionsV1>={}){
 const store=temp("store"),who=runner(name,store,extra);
 put(who.options.worktree,"a.txt","alpha\n");
 await seedV1(who.options);
 return {store,who};
}
let tick=1_700_000_000;
const stamp=(file:string):void=>{utimesSync(file,tick,tick);tick++;};

test("new code that does not start is refused, the daemon keeps replicating on the old code, shows the refusal and restarts once the code on disk is fixed",async()=>{
 const checks:string[]=[];
 let verdict:{ok:true}|{ok:false;reason:string}={ok:false,reason:"module x/🟦️.ts does not load: Unexpected ;"};
 const {store,who}=await seeded("a1",{selfCheck:async digest=>{checks.push(digest);return verdict;}});
 start(who);
 let finished=false;void who.done!.then(()=>{finished=true;});
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 const running=(await readAttestationV1(store,"a1"))!.code,root=who.options.codeRoot!;
 put(root,"x/🟦️.ts","export const a=;\n");
 await until(()=>who.events.some(event=>event.event==="restart-refused"));
 const broken=await codeDigestV1(root);
 expect(who.events.filter(event=>event.event==="restart-refused")).toHaveLength(1);
 expect(who.events.find(event=>event.event==="restart-refused")).toMatchObject({reason:broken,message:"module x/🟦️.ts does not load: Unexpected ;"});
 await until(async()=>(await readAttestationV1(store,"a1"))?.pending?.code===broken);
 const attestation=(await readAttestationV1(store,"a1"))!;
 expect(attestation.code).toBe(running);expect(attestation.pending).toEqual({code:broken,ok:false,reason:"module x/🟦️.ts does not load: Unexpected ;"});
 put(who.options.worktree,"during.txt","written while the new code is refused\n");
 await until(()=>readFileSync(join(store,"v1","origins","a1","segments","000000000000.jsonl"),"utf8").includes("during.txt"));
 await Bun.sleep(600);
 expect(finished).toBe(false);expect(checks).toEqual([broken]);
 expect(who.events.filter(event=>event.event==="restarting")).toHaveLength(0);
 verdict={ok:true};
 put(root,"x/🟦️.ts","export const a=3;\n");
 await until(()=>finished);
 expect(checks).toHaveLength(2);expect(checks[1]).not.toBe(broken);
 expect(who.events.filter(event=>event.event==="restarting")).toHaveLength(1);
 expect(who.events.filter(event=>event.event==="restart-refused")).toHaveLength(1);
});

test("the restart waits for the deterministic jitter, the minimum uptime and an empty queue, and an unproven digest never restarts",async()=>{
 let clock=1_000_000,busy=true,verdict=true;
 const root=code("jitter"),digests:string[]=[];
 const attester=createAttesterV1({storeRoot:temp("store"),originId:"g1-m07",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,stableMilliseconds:5000,jitterSpanMilliseconds:60_000,minimumUptimeMilliseconds:20_000,busyCapMilliseconds:30_000,codeRoot:root,git:async()=>git,reachable:async()=>false,busy:()=>busy,check:async digest=>{digests.push(digest);return verdict?{ok:true}:{ok:false,reason:"nope"};}});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-g1-m07"};
 const step=async(milliseconds:number)=>{clock+=milliseconds;return attester.tick(snapshot);};
 put(root,"x/🟦️.ts","export const a=2;\n");
 const next=await codeDigestV1(root),jitter=restartJitterMillisecondsV1("g1-m07",next,60_000);
 expect(jitter).toBeGreaterThan(0);
 expect(await step(1000)).toBe("idle");
 const noticed=clock;
 for(;clock<noticed+5000+jitter-1000;)expect(await step(1000)).toBe("idle");
 expect(digests).toEqual([next]);
 let restarted=false;
 for(let n=0;n<40&&!restarted;n++){restarted=await step(500)==="restart";if(restarted)break;}
 expect(restarted).toBe(false);
 busy=false;
 expect(await step(500)).toBe("restart");
 expect(clock-noticed).toBeGreaterThanOrEqual(5000+jitter);
 expect(attester.pending()).toBeNull();
 verdict=false;
 put(root,"x/🟦️.ts","export const a=4;\n");
 for(let n=0;n<40;n++)expect(await step(1000)).toBe("idle");
 expect(attester.pending()).toMatchObject({ok:false,reason:"nope"});
 expect(digests).toHaveLength(2);
});

test("a busy replica gives up deferring the proven restart after the cap",async()=>{
 let clock=1_000_000;
 const root=code("busycap"),attester=createAttesterV1({storeRoot:temp("store"),originId:"a",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,stableMilliseconds:1000,jitterSpanMilliseconds:0,minimumUptimeMilliseconds:0,busyCapMilliseconds:10_000,codeRoot:root,git:async()=>git,reachable:async()=>false,busy:()=>true});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:3,hlc:"0000001000000-000000-a"};
 put(root,"x/🟦️.ts","export const a=2;\n");
 const seen:string[]=[];
 for(let n=0;n<40;n++){clock+=1000;seen.push(await attester.tick(snapshot));}
 const first=seen.indexOf("restart");
 expect(first).toBeGreaterThanOrEqual(10);expect(first).toBeLessThan(20);
});

test("the daemon check runs the real self-check in a fresh process, refuses code whose check fails and seals the checked code before it restarts",async()=>{
 const repo=temp("repo"),module=join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree"),attest=join(import.meta.dirname,"..","..","🩺️attest","🟦️.ts");
 const script=(ok:boolean):string=>`import {codeDigestV1} from ${JSON.stringify(attest)};\nconst digest=await codeDigestV1(${JSON.stringify(module)});\nconsole.log(JSON.stringify({version:1,ok:${ok},digest,modules:1,schemas:0,texts:0,error:${ok?"null":JSON.stringify("module boom does not load")}}));\nprocess.exit(${ok?0:1});\n`;
 put(module,"a/🟦️.ts","export const a=1;\n");
 put(module,"🔁️replica/📜️script.ts",script(false));
 const store=temp("store"),who=runner("a1",store,{codeRoot:module,selfCheck:undefined,worktree:repo,storeRoot:store});
 put(repo,"a.txt","alpha\n");
 await seedV1(who.options);
 start(who);
 let finished=false;void who.done!.then(()=>{finished=true;});
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 put(module,"a/🟦️.ts","export const a=2;\n");
 await until(()=>who.events.some(event=>event.event==="restart-refused"));
 expect(who.events.find(event=>event.event==="restart-refused")!.message).toContain("module boom does not load");
 expect(finished).toBe(false);
 put(module,"🔁️replica/📜️script.ts",script(true));
 const fixed=await codeDigestV1(module);
 await until(()=>finished);
 expect(who.events.filter(event=>event.event==="restarting")).toHaveLength(1);
 const sealed=(await listCodeSnapshotsV1(repo)).map(item=>item.digest);
 expect(sealed).toContain(fixed);
 expect(sealed.length).toBeLessThanOrEqual(2);
});

test("a stalled store is reported within the deadline, local work and the loop continue, nothing new is issued and the daemon recovers by itself",async()=>{
 const {store,who}=await seeded("a1");
 let hung=false,gateCalls=0;
 const hang=new Promise<never>(()=>{});
 let releaseGate:()=>void=()=>{},releaseProbe:()=>void=()=>{};
 const gate=new Promise<void>(done=>{releaseGate=done;}),probeRelease=new Promise<boolean>(done=>{releaseProbe=()=>{done(true);};});
 who.options={...who.options,storeDeadlineMilliseconds:300,storeProbe:async()=>{if(hung)return probeRelease;return true;},storeGate:async()=>{gateCalls++;if(hung){await gate;}}};
 start(who);
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 await until(()=>who.events.some(event=>event.event==="published")||true);
 hung=true;
 put(who.options.worktree,"stalled.txt","written while the store hangs\n");
 const hungAt=Date.now();
 await until(()=>who.events.some(event=>event.event==="store-unreachable"),5000);
 expect(Date.now()-hungAt).toBeLessThan(4000);
 expect(who.events.find(event=>event.event==="store-unreachable")).toMatchObject({reason:"stalled"});
 let ticks=0;const timer=setInterval(()=>{ticks++;},10);
 const before=gateCalls;
 await Bun.sleep(700);
 clearInterval(timer);
 expect(ticks).toBeGreaterThan(Math.floor(20/(stretchedBudgetMillisecondsV1(1000)/1000)));
 expect(gateCalls-before).toBeLessThanOrEqual(1);
 await until(async()=>(await readHealthV1(who.options.worktree))?.store==="stalled");
 expect(readFileSync(join(store,"v1","origins","a1","segments","000000000000.jsonl"),"utf8")).not.toContain("stalled.txt");
 expect(existsSync(join(who.options.worktree,"stalled.txt"))).toBe(true);
 hung=false;releaseProbe();releaseGate();
 await until(()=>who.events.some(event=>event.event==="store-reachable"),15000);
 await until(()=>readFileSync(join(store,"v1","origins","a1","segments","000000000000.jsonl"),"utf8").includes("stalled.txt"),15000);
 expect((await readHealthV1(who.options.worktree))?.store).toBeDefined();
 await stop(who);
 void hang;
});

test("a store that stays stalled for the stuck limit makes the daemon end itself for the supervisor, a store that answers in time does not",async()=>{
 const {store,who}=await seeded("a1");
 let hung=false;
 who.options={...who.options,storeDeadlineMilliseconds:200,storeStuckRestartMilliseconds:2500,storeProbe:async()=>{if(hung)return new Promise<boolean>(()=>{});return true;}};
 let finished=false;
 start(who);void who.done!.then(()=>{finished=true;});
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 await Bun.sleep(3000);
 expect(finished).toBe(false);
 hung=true;
 const hungAt=Date.now();
 await until(()=>finished,20000);
 expect(Date.now()-hungAt).toBeGreaterThanOrEqual(2500);expect(Date.now()-hungAt).toBeLessThan(15000);
 expect(who.events.find(event=>event.event==="store-stuck")).toMatchObject({reason:"stalled",seconds:3});
 expect(who.events.some(event=>event.event==="restarting")).toBe(true);
});

test("a recurring failure stays visible in the attestation and the health file with its count while rounds back off instead of spinning",async()=>{
 const {store,who}=await seeded("a1");
 let calls=0;
 who.options={...who.options,storeGate:async()=>{calls++;throw Error("ESTALE: stale file handle");}};
 start(who);
 await until(()=>calls>=4);
 await until(async()=>((await readAttestationV1(store,"a1"))?.failures??[]).some(item=>item.kind==="round"&&item.count>=3));
 const attestation=(await readAttestationV1(store,"a1"))!;
 expect(attestation.failures![0]).toMatchObject({kind:"round"});
 expect(who.events.filter(event=>event.event==="error"&&(event.message??"").includes("ESTALE"))).toHaveLength(1);
 const seen=calls;
 await Bun.sleep(1500);
 expect(calls-seen).toBeLessThan(8);
 await until(async()=>((await readHealthV1(who.options.worktree))?.failures??[]).some(item=>item.kind==="round"));
 const report=await statusV1(who.options);
 expect(report.health?.failures.some(item=>item.kind==="round")).toBe(true);
 await stop(who);
});

test("the daemon still follows the peer journal while its own outbound journal repeatedly fails",async()=>{
 const vector=outboundTail,{store,who:source}=await seeded(vector.source);
 let replica:ReplicaV1|undefined;
 const receiver=runner(vector.receiver,store,{scanSeconds:3600,walkSeconds:3600,sweepMilliseconds:3_600_000,watch:false,opened:value=>{replica=value;}});
 const root=receiver.options.worktree,origin=(name:string)=>join(store,"v1","origins",name);
 const file=(path:string)=>join(root,path),segments=join(origin(vector.receiver),"segments");
 await joinV1(receiver.options);
 const peer=await openReplicaV1(source.options);
 try{
  start(receiver);
  await until(()=>receiver.events.some(event=>event.event==="started"));
  put(source.options.worktree,vector.baseline.path,vector.baseline.text);
  expect((await publishOnceV1(peer,undefined,[vector.baseline.path])).published).toBe(1);
  await until(()=>existsSync(file(vector.baseline.path)));
  expect(readFileSync(file(vector.baseline.path),"utf8")).toBe(vector.baseline.text);

  rmSync(segments,{recursive:true,force:true});
  writeFileSync(segments,vector.blockedJournal);
  put(root,vector.outbound.path,vector.outbound.text);
  await writeHintV1(root,[vector.outbound.path]);
  const failures=()=>replica?.failures.summary().find(row=>row.kind==="round")?.count??0;
  await until(()=>failures()>=1);
  put(source.options.worktree,vector.inbound.path,vector.inbound.text);
  expect((await publishOnceV1(peer,undefined,[vector.inbound.path])).published).toBe(1);
  await until(()=>failures()>=vector.minimumFailures);
  expect(receiver.events.some(event=>event.event==="error"&&event.message?.includes("segments"))).toBe(true);
  expect(readFileSync(segments,"utf8")).toBe(vector.blockedJournal);
  expect(await readHeadV1(store,vector.receiver)).toBeUndefined();
  expect(readFileSync(file(vector.outbound.path),"utf8")).toBe(vector.outbound.text);

  const journal=readFileSync(join(origin(vector.source),"segments","000000000000.jsonl"),"utf8").trim().split("\n");
  const sqlite=new Database(":memory:");
  try{
   const expected=sqlite.query(`select json_extract(value, '$.sha256') as sha256, json_extract(value, '$.bytes') as bytes
    from json_each(?) where json_extract(value, '$.path') = ? and json_extract(value, '$.operation') = 'put'
    order by json_extract(value, '$.hlc') desc limit 1`).get(`[${journal.join(",")}]`,vector.inbound.path);
   const inbound=existsSync(file(vector.inbound.path))?readFileSync(file(vector.inbound.path)):null;
   expect(expected).not.toBeNull();
   expect<unknown>(inbound&&{sha256:Bun.CryptoHasher.hash("sha256",inbound,"hex"),bytes:inbound.byteLength}).toEqual(expected);
   expect(inbound?.toString("utf8")).toBe(vector.inbound.text);
  }finally{sqlite.close();}
 }finally{try{await stop(receiver);}finally{await closeReplicaV1(peer);}}
});

test("a state checkpoint that a power loss tore is reported, rebuilt from the store and never republishes the tree",async()=>{
 const store=temp("store"),mk=(name:string)=>({name,options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,logFiles:[] as string[]} as ReplicaOptionsV1}),a=mk("a"),b=mk("b");
 put(a.options.worktree,"one.txt","one\n");put(a.options.worktree,"two.txt","two\n");
 await seedV1(a.options);await joinV1(b.options);
 for(const torn of ["state.ckp"]){
  const events:ReplicaEventV1[]=[],replica=await openReplicaV1(a.options,event=>events.push(event));
  put(a.options.worktree,`edit-${torn}.txt`,torn);
  await publishOnceV1(replica);
  await closeReplicaV1(replica);
  writeFileSync(join(a.options.worktree,".cache","f",torn),'{"version":1,"gener');
  const reopened=await openReplicaV1(a.options,event=>events.push(event));
  expect(events.some(event=>event.event==="state-rebuilt"),torn).toBe(true);
  const before=(await readHeadV1(store,"a"))!.events;
  for(let round=0;round<3;round++){await followOnceV1(reopened);await publishOnceV1(reopened);}
  expect((await readHeadV1(store,"a"))!.events,torn).toBe(before);
  expect(readFileSync(join(a.options.worktree,"one.txt"),"utf8")).toBe("one\n");
  await closeReplicaV1(reopened);
 }
 const other=await openReplicaV1(b.options);
 for(let round=0;round<3;round++)await followOnceV1(other);
 expect(readFileSync(join(b.options.worktree,"two.txt"),"utf8")).toBe("two\n");
 await closeReplicaV1(other);
});

test("a segment that is shorter than its head is sealed short and the append continues in a new segment that every cursor still reaches",async()=>{
 const store=temp("store"),origin="a",event=(n:number):FleetEventV1=>({version:1,hlc:formatHlcV1({ms:1_700_000_000_000+n,counter:0},origin),operation:"delete",path:`p${n}.txt`,base:null});
 await appendEventsV1(store,origin,[event(1),event(2),event(3)]);
 const segment=join(store,"v1","origins",origin,"segments","000000000000.jsonl"),head=(await readHeadV1(store,origin))!;
 const seen=await readEventSpansV1(store,origin,undefined);
 expect(seen.events).toHaveLength(3);
 const full=readFileSync(segment);
 const cut=full.indexOf(10,full.indexOf(10)+1)+1+5;
 truncateSync(segment,cut);
 const repairs:unknown[]=[];
 const repaired=await appendEventsV1(store,origin,[event(4)],repair=>repairs.push(repair));
 expect(repairs).toEqual([{segment:0,bytes:cut-5,found:2}]);
 expect(repaired.segment).toBe(1);expect(head.segment).toBe(0);
 expect(statSizeOf(segment)).toBe(cut-5);
 const after=await readEventSpansV1(store,origin,{segment:0,bytes:head.bytes});
 expect(after.events.map(item=>item.path)).toEqual(["p4.txt"]);
 const fresh=await readEventSpansV1(store,origin,undefined);
 expect(fresh.events.map(item=>item.path)).toEqual(["p1.txt","p2.txt","p4.txt"]);
 rmSync(join(store,"v1","origins",origin,"segments","000000000001.jsonl"));
 const missing=await appendEventsV1(store,origin,[event(5)],repair=>repairs.push(repair));
 expect(missing.segment).toBe(2);
 expect(repairs.at(-1)).toEqual({segment:1,bytes:0,found:0});
});
const statSizeOf=(file:string):number=>readFileSync(file).length;

test("a snapshot is refused while a fold needs a rejoin and compaction then deletes nothing",async()=>{
 const {store,who}=await seeded("a1");
 const origin=join(store,"v1","origins","a1","segments");
 put(who.options.worktree,"b.txt","beta\n");
 const replica=await openReplicaV1(who.options);await publishOnceV1(replica);await closeReplicaV1(replica);
 rmSync(join(origin,"000000000000.jsonl"));
 const events:ReplicaEventV1[]=[];
 expect(await snapshotV1(store,"a1",undefined,event=>events.push(event))).toBeUndefined();
 expect(events.some(event=>event.event==="refused"&&event.stage==="snapshot"&&event.reason==="rejoin-required")).toBe(true);
 const compacted=await compactV1(store,"a1",0);
 expect(compacted.snapshot).toBeUndefined();expect(compacted.segments).toBe(0);
 expect(existsSync(join(store,"v1","snapshots","a1"))).toBe(false);
});

test("an event of an unsupported version stalls its origin visibly: it is counted, the cursor stays and nothing after it is applied",async()=>{
 const {store,who}=await seeded("a1");
 const other=join(store,"v1","origins","z9");
 mkdirSync(join(other,"segments"),{recursive:true});
 const bad=`{"version":2,"hlc":"${formatHlcV1({ms:Date.now(),counter:0},"z9")}","operation":"put","path":"new.txt","sha256":"${"b".repeat(64)}","bytes":1,"executable":false,"base":null,"extra":true}\n`;
 writeFileSync(join(other,"segments","000000000000.jsonl"),bad);
 writeFileSync(join(other,"head.json"),JSON.stringify({version:1,segment:0,bytes:Buffer.byteLength(bad),events:1,hlc:formatHlcV1({ms:Date.now(),counter:0},"z9")}));
 const events:ReplicaEventV1[]=[],replica=await openReplicaV1(who.options,event=>events.push(event));
 for(let round=0;round<3;round++)await followOnceV1(replica);
 expect(events.filter(event=>event.event==="error"&&(event.message??"").includes("unsupported event version 2"))).toHaveLength(1);
 expect(replica.failures.summary().find(item=>item.kind==="unsupported")?.count).toBe(3);
 expect(replica.cursors.z9).toBeUndefined();
 await closeReplicaV1(replica);
});

test("a corrupt blob in the store is replaced when the same content is written again and when the relay copies it",async()=>{
 const store=temp("store"),source=join(temp("src"),"f.txt");
 writeFileSync(source,"the real content\n");
 const sha=(await import("node:crypto")).createHash("sha256").update("the real content\n").digest("hex");
 expect(await writeBlobV1(store,"a",sha,source)).toBe(true);
 expect(await writeBlobV1(store,"a",sha,source)).toBe(false);
 writeFileSync(blobPathV1(store,sha),"bit rot");
 const second=temp("other"),events:ReplicaEventV1[]=[];
 expect(readFileSync(blobPathV1(store,sha),"utf8")).toBe("bit rot");
 const {writeBlobV1:fresh}=await import(`../../📒️journal/🟦️.ts?again=${Date.now()}`) as typeof import("../../📒️journal/🟦️.ts");
 expect(await fresh(store,"a",sha,source)).toBe(true);
 expect(readFileSync(blobPathV1(store,sha),"utf8")).toBe("the real content\n");
 void second;void events;
});

test("two machines never share one origin id: the second one is refused at open, at join and at a publish, and a legacy home gets the key of its owner",async()=>{
 const store=temp("store"),make=(worktree:string):ReplicaOptionsV1=>({worktree,storeRoot:store,originId:"a",groupId:"g1",pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,logFiles:[]});
 const one=make(temp("one")),two=make(temp("two"));
 put(one.worktree,"a.txt","alpha\n");
 await seedV1(one);
 const home=JSON.parse(readFileSync(join(store,"v1","origins","a","home.json"),"utf8")) as {key:string};
 expect(home.key).toMatch(/^[0-9a-f]{32}$/u);
 await expect(openReplicaV1(two)).rejects.toBeInstanceOf(OriginTakenV1);
 await expect(joinV1(two)).rejects.toBeInstanceOf(OriginTakenV1);
 const again=await openReplicaV1(one);await closeReplicaV1(again);
 writeFileSync(join(store,"v1","origins","a","home.json"),JSON.stringify({version:1,group:"g1"}));
 const owner=await openReplicaV1(one);
 put(one.worktree,"b.txt","beta\n");
 await publishOnceV1(owner);await closeReplicaV1(owner);
 expect((JSON.parse(readFileSync(join(store,"v1","origins","a","home.json"),"utf8")) as {key:string}).key).toBe(home.key);
 await expect(openReplicaV1(make(temp("three")))).rejects.toBeInstanceOf(OriginTakenV1);
 const torn=join(one.worktree,".cache","f","machine.json");
 writeFileSync(torn,'{"version":1,"orig');
 const adopted=await openReplicaV1(one);await closeReplicaV1(adopted);
 expect((JSON.parse(readFileSync(torn,"utf8")) as {key:string}).key).toBe(home.key);
});

test("a lock of an earlier boot, of a silent holder, of a dead holder and a steal race are all settled to exactly one daemon",async()=>{
 const worktree=temp("lock"),store=temp("store"),options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"a",groupId:"g1",logFiles:[]},lock=join(worktree,".cache","f","daemon.lock");
 mkdirSync(join(worktree,".cache","f"),{recursive:true});
 const old=new Date(Date.now()-120_000),child=Bun.spawn(["sleep","30"]);
 try{
  const alive=child.pid,begun=(()=>{const read=readProcessStartV1(alive);return typeof read==="number"?read:1;})();
  writeFileSync(lock,JSON.stringify({version:1,pid:alive,startMs:begun,originId:"a"}));utimesSync(lock,old,old);
  await expect(openReplicaV1(options)).rejects.toBeInstanceOf(ReplicaBusyV1);
  writeFileSync(lock,JSON.stringify({version:1,pid:alive,startMs:begun,originId:"a",hb:true,bootMs:Date.now()}));utimesSync(lock,old,old);
  const silent=await openReplicaV1(options);await closeReplicaV1(silent);
  writeFileSync(lock,JSON.stringify({version:1,pid:alive,startMs:begun,originId:"a",bootMs:Date.now()-10*86_400_000}));utimesSync(lock,old,old);
  const rebooted=await openReplicaV1(options);await closeReplicaV1(rebooted);
  writeFileSync(lock,JSON.stringify({version:1,pid:alive,startMs:begun,originId:"a",hb:true,bootMs:Date.now()}));
  await expect(openReplicaV1(options)).rejects.toBeInstanceOf(ReplicaBusyV1);
 }finally{child.kill();}
 writeFileSync(lock,JSON.stringify({version:1,pid:2147483646,startedMs:1,originId:"a"}));
 const results=await Promise.allSettled(Array.from({length:8},()=>openReplicaV1(options)));
 const opened=results.filter(item=>item.status==="fulfilled") as PromiseFulfilledResult<ReplicaV1>[];
 expect(opened).toHaveLength(1);
 for(const item of results)if(item.status==="rejected")expect(item.reason).toBeInstanceOf(ReplicaBusyV1);
 await closeReplicaV1(opened[0]!.value);
 expect(existsSync(lock)).toBe(false);
});

test("events from the future in the own journal refuse new stamps until the wall clock catches up instead of freezing the origin",async()=>{
 const store=temp("store"),wall=Date.now();
 let now=wall;
 const options:ReplicaOptionsV1={worktree:temp("wt"),storeRoot:store,originId:"a",groupId:"g1",persistMilliseconds:0,logFiles:[],now:()=>now};
 put(options.worktree,"a.txt","alpha\n");
 await seedV1(options);
 const events:ReplicaEventV1[]=[];
 await appendEventsV1(store,"a",[{version:1,hlc:formatHlcV1({ms:wall+3_600_000,counter:0},"a"),operation:"delete",path:"future.txt",base:null}]);
 const replica=await openReplicaV1(options,event=>events.push(event));
 expect(events.find(event=>event.event==="clock-refused")).toMatchObject({reason:"own-clock-ahead"});
 put(options.worktree,"b.txt","beta\n");
 await expect(publishOnceV1(replica)).rejects.toBeInstanceOf(ClockAheadV1);
 expect((await readHeadV1(store,"a"))!.events).toBe(2);
 now=wall+3_600_001;
 const result=await publishOnceV1(replica);
 expect(result.published).toBe(1);
 await closeReplicaV1(replica);
});

test("preserved overwritten files and oversized logs are bounded by the daemon itself",async()=>{
 const {who}=await seeded("a1");
 const old=join(who.options.worktree,".cache","f","lost",`${String(Date.now()-40*86_400_000).padStart(13,"0")}-000000-a`,"gone.txt"),fresh=join(who.options.worktree,".cache","f","lost",`${String(Date.now()).padStart(13,"0")}-000000-a`,"kept.txt");
 put(dirname(dirname(dirname(old))),relative(dirname(dirname(dirname(old))),old),"x");put(dirname(dirname(dirname(fresh))),relative(dirname(dirname(dirname(fresh))),fresh),"y");
 const log=join(temp("logs"),"service.log");
 writeFileSync(log,"");truncateSync(log,70*1024*1024);
 who.options={...who.options,logFiles:[log]};
 start(who);
 await until(()=>!existsSync(old)&&existsSync(`${log}.1`));
 expect(existsSync(fresh)).toBe(true);
 expect(readFileSync(log).length).toBeLessThan(1024);
 await stop(who);
});

const verdictCase=fixture.verdicts[0]!;
const attestations=():Record<string,FleetAttestationV1>=>JSON.parse(JSON.stringify(verdictCase.attestations)) as Record<string,FleetAttestationV1>;

test("the fleet verdict votes the code digest and does not trust a writer whose clock runs ahead",()=>{
 const same=judgeFleetV1({now:verdictCase.now,heads:headsOf(verdictCase.heads),attestations:attestations(),retirements:{}});
 expect(same.ok).toBe(true);
 const mixed=attestations();
 mixed.b={...mixed.b!,code:"e".repeat(64)};
 const report=judgeFleetV1({now:verdictCase.now,heads:headsOf(verdictCase.heads),attestations:mixed,retirements:{}});
 expect(report.ok).toBe(false);
 expect(report.differing).toEqual([{originId:"b",field:"code",value:"e".repeat(64),expected:"d".repeat(64)}]);
 expect(report.rows.find(row=>row.originId==="b")).toMatchObject({verdict:"diverged",differs:["code"]});
 const ahead=attestations();
 ahead.c={...ahead.c!,wallMs:verdictCase.now+(skewSecondsV1+1)*1000};
 const skewed=judgeFleetV1({now:verdictCase.now,heads:headsOf(verdictCase.heads),attestations:ahead,retirements:{}});
 expect(skewed.rows.find(row=>row.originId==="c")).toMatchObject({verdict:"stale",fresh:false,ageSeconds:0});
 const slight=attestations();
 slight.c={...slight.c!,wallMs:verdictCase.now+10_000};
 expect(judgeFleetV1({now:verdictCase.now,heads:headsOf(verdictCase.heads),attestations:slight,retirements:{}}).ok).toBe(true);
});

test("retire refuses an origin whose attestation exists but cannot be read, unless forced",async()=>{
 const {store,who}=await seeded("a1");
 const other=join(store,"v1","origins","z9");
 mkdirSync(other,{recursive:true});writeFileSync(join(other,"attest.json"),'{"version":1,"orig');
 const lines:string[]=[],context={root:who.options.worktree,environment:{} as Record<string,string|undefined>,signal:new AbortController().signal,write:()=>{},notice:(line:string)=>{lines.push(line);}};
 const args=["retire","z9","--worktree",who.options.worktree,"--store",store,"--lang","en"];
 expect((await runReplicaCommandV1([...args],context)).exitCode).toBe(1);
 expect(lines.join("\n")).toContain("cannot be read");
 expect((await runReplicaCommandV1([...args,"--force"],context)).exitCode).toBe(0);
 void attestPathV1;void writeAttestationV1;void fleetReportV1;void foldStoreV1;void homedOriginsV1;void relayOnceV1;void sealCodeV1;void appendFileSync;void symlinkSync;void copyFileSync;void cpSync;void stamp;void tick;
});

const entryTree=(label:string,worktreeCommand:string):{repo:string;module:string;script:string}=>{
 const repo=temp(label),module=join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree"),script=join(module,"🔁️replica","📜️script.ts");
 mkdirSync(dirname(script),{recursive:true});
 copyFileSync(join(import.meta.dirname,"..","📜️script.ts"),script);
 mkdirSync(join(module,"🔁️replica","🛟️supervisor"),{recursive:true});copyFileSync(join(import.meta.dirname,"..","🛟️supervisor","🟦️.ts"),join(module,"🔁️replica","🛟️supervisor","🟦️.ts"));
 put(module,"🔁️replica/⌨️command/🟦️.ts",worktreeCommand);
 put(module,"🛡️resilience/🟦️.ts","import {existsSync} from \"node:fs\";\nimport {join} from \"node:path\";\nexport async function selfCheckV1(root:string){return {ok:!existsSync(join(root,\"broken.flag\")),error:\"broken.flag present\"};}\nexport async function startRejectionV1(root:string){return existsSync(join(root,\"revert.flag\"))?\"ran before and was superseded (revert)\":null;}\n");
 put(module,"🩺️attest/🟦️.ts","export async function codeDigestV1(){return \"d\".repeat(64);}\n");
 return {repo,module,script};
};
const commandSource=(label:string):string=>`export async function runReplicaCommandV1(args:string[],context:{environment:Record<string,string|undefined>;write:(line:string)=>void}){context.write(JSON.stringify({ran:${JSON.stringify(label)},args,watch:context.environment.SEMIO_FLEET_CODE_WATCH??null}));return {exitCode:0};}\n`;
const runEntry=(script:string,args:string[]):{code:number;out:string;err:string}=>{const result=Bun.spawnSync([process.execPath,script,...args],{stdout:"pipe",stderr:"pipe"});return {code:result.exitCode??-1,out:result.stdout.toString(),err:result.stderr.toString()};};
const seal=async(tree:{repo:string;module:string},label:string,now:number):Promise<string> =>{
 put(tree.module,"🔁️replica/⌨️command/🟦️.ts",commandSource(label));
 const digest=await codeDigestV1(tree.module);
 expect((await sealCodeV1({root:tree.repo,moduleRoot:tree.module,digest,digestOf:codeDigestV1,now:()=>now})).created).toBe(true);
 return digest;
};

test("the stable entry runs the worktree code when it starts and falls back to the newest sealed snapshot that starts when it does not, but never for a self-check",async()=>{
 const tree=entryTree("entry",commandSource("worktree"));
 const old=await seal(tree,"snapshot-old",1000),fresh=await seal(tree,"snapshot-new",2000);
 expect(old).not.toBe(fresh);
 put(tree.module,"🔁️replica/⌨️command/🟦️.ts",commandSource("worktree"));
 const healthy=runEntry(tree.script,["status"]);
 expect(healthy.code).toBe(0);expect(JSON.parse(healthy.out)).toEqual({ran:"worktree",args:["status"],watch:null});expect(healthy.err).toBe("");
 put(tree.module,"🔁️replica/⌨️command/🟦️.ts","export const broken=;\n");
 const syntax=runEntry(tree.script,["run"]);
 expect(syntax.code).toBe(0);expect(JSON.parse(syntax.out)).toEqual({ran:"snapshot-new",args:["run"],watch:realpathSync(tree.module)});
 expect(syntax.err).toContain("does not start");expect(syntax.err).toContain("startet nicht");expect(syntax.err).toContain("Running the sealed code");
 put(tree.module,"🔁️replica/⌨️command/🟦️.ts",commandSource("worktree"));writeFileSync(join(tree.module,"broken.flag"),"");
 const failedCheck=runEntry(tree.script,["run"]);
 expect(JSON.parse(failedCheck.out).ran).toBe("snapshot-new");
 const strict=runEntry(tree.script,["self-check"]);
 expect(strict.code).toBe(1);expect(strict.out).toBe("");expect(strict.err).toContain("broken.flag present");
 rmSync(join(tree.module,"broken.flag"));
 put(tree.repo,`.cache/f/code/${fresh}/files/${relative(tree.repo,tree.module).split("\\").join("/")}/🔁️replica/⌨️command/🟦️.ts`,"export const gone=;\n");
 put(tree.module,"🔁️replica/⌨️command/🟦️.ts","export const broken=;\n");
 const older=runEntry(tree.script,["run"]);
 expect(JSON.parse(older.out).ran).toBe("snapshot-old");
 rmSync(join(tree.repo,".cache","f","code"),{recursive:true,force:true});
 const none=runEntry(tree.script,["run"]);
 expect(none.code).toBe(1);expect(none.err).toContain("No replica code starts");expect(none.err).toContain("Kein Replica-Code startet");
});

test("the real self-check command passes on the real module and its report is a closed document",()=>{
 const result=Bun.spawnSync([process.execPath,join(import.meta.dirname,"..","📜️script.ts"),"self-check","--lang","en"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const report=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {ok:boolean;digest:string;modules:number;schemas:number;texts:number};
 expect(report.ok).toBe(true);expect(report.modules).toBeGreaterThanOrEqual(12);
 expect(report.digest).toMatch(/^[0-9a-f]{64}$/u);
 expect(report.schemas).toBeGreaterThanOrEqual(7);expect(report.texts).toBeGreaterThanOrEqual(2);
});

test("a failed write of the last chunk of a blob copy is never swallowed, so a truncated blob is never stored",async()=>{
 const dir=temp("copy"),source=join(dir,"big.bin");
 writeFileSync(source,Buffer.alloc(3*1024*1024+17,7));
 const {copyHashedV1}=await import("../../📒️journal/🟦️.ts");
 const target=join(dir,"copy.bin"),healthy=await copyHashedV1(source,target,{});
 expect(healthy.stable).toBe(true);expect(readFileSync(target).length).toBe(3*1024*1024+17);
 rmSync(target);
 const {installKillPointsV1}=await import("../../🛡️resilience/🧪️tests/💥️killpoints.ts");
 const kill=await installKillPointsV1();
 const {copyHashedV1:mocked}=await import(`../../📒️journal/🟦️.ts?kill=${Date.now()}`) as typeof import("../../📒️journal/🟦️.ts");
 kill.reset();await mocked(source,target,{});rmSync(target);
 const writes=kill.last().map((name,index)=>name==="handle.write"?index:-1).filter(index=>index>=0);
 expect(writes.length).toBeGreaterThanOrEqual(4);
 kill.reset();kill.arm(writes.at(-1)!,"before",true);
 await expect(mocked(source,target,{})).rejects.toThrow("kill point");
 await kill.disarm();
 expect(existsSync(target)).toBe(false);
});

test("code that ran before and was superseded, or was refused, is never adopted automatically and the start prefers the sealed snapshot over it",async()=>{
 const root=temp("history");
 const {recordRefusedV1,recordRunV1,suspectCodeV1,startRejectionV1,readRunHistoryV1}=await import("../../🛡️resilience/🟦️.ts");
 const [one,two,three]=["1","2","3"].map(n=>n.repeat(64));
 await recordRunV1(root,one,1000);await recordRunV1(root,two,2000);await recordRunV1(root,two,9000);
 const history=await readRunHistoryV1(root);
 expect(history.runs).toEqual([{digest:one,firstMs:1000},{digest:two,firstMs:2000}]);
 expect(suspectCodeV1(history,two,one)).toContain("revert");
 expect(suspectCodeV1(history,two,three)).toBeUndefined();
 expect(suspectCodeV1(history,two,two)).toBeUndefined();
 expect(suspectCodeV1(history,one,two)).toBeUndefined();
 await recordRefusedV1(root,three);
 expect(suspectCodeV1(await readRunHistoryV1(root),two,three)).toBe("refused before");
 expect(await startRejectionV1(root,one)).toContain("revert");
 expect(await startRejectionV1(root,two)).toBeNull();
});

test("the attester does not check or restart into a digest that is suspect and reports it once",async()=>{
 let clock=1_000_000;
 const root=code("revert"),suspicious:string[]=[];
 const attester=createAttesterV1({storeRoot:temp("store"),originId:"a",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,stableMilliseconds:1000,jitterSpanMilliseconds:0,minimumUptimeMilliseconds:0,codeRoot:root,git:async()=>git,reachable:async()=>false,suspect:()=>"ran before and was superseded (revert)",suspicious:(digest,reason)=>{suspicious.push(`${digest.slice(0,4)} ${reason}`);},check:async()=>{throw Error("must not be checked");}});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a",activity:0};
 put(root,"x/🟦️.ts","export const a=2;\n");
 for(let n=0;n<20;n++){clock+=1000;expect(await attester.tick(snapshot)).toBe("idle");}
 expect(suspicious).toHaveLength(1);
 expect(attester.pending()).toMatchObject({ok:false,reason:expect.stringContaining("revert")});
});

test("the self-check dry-opens the current local state read-only and refuses code that cannot read it",async()=>{
 const {store,who}=await seeded("a1");
 const replica=await openReplicaV1(who.options);await closeReplicaV1(replica);
 const {dryOpenStateV1}=await import("../🟦️.ts");
 expect(await dryOpenStateV1(who.options.worktree)).toBeNull();
 const before=readdirSync(join(who.options.worktree,".cache","f")).sort();
 writeFileSync(join(who.options.worktree,".cache","f","state.ckp"),"garbage that no code can read");
 expect(await dryOpenStateV1(who.options.worktree)).not.toBeNull();
 expect(readdirSync(join(who.options.worktree,".cache","f")).sort()).toEqual(before);
 void store;
});
