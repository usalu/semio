import {Database} from "bun:sqlite";
import observationFixture from "./🔭️git-observation/🔣️.json" with {type:"json"};
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {listOriginsV1} from "../../📒️journal/🟦️.ts";
import type {LatencyReportV1} from "../../🎣️tiers/🟦️.ts";
import {platformV1} from "../../🗺️topology/🟦️.ts";
import {parseGitViewV1} from "../../🌿️git/🟦️.ts";
import {closeReplicaV1,CompactTableV1,compactV1,homedOriginsV1,joinV1,openReplicaV1,publishOnceV1,ReplicaBookV1,relayOnceV1,runV1,seedV1,setSyncEnabledV1,type ReplicaEventV1,type ReplicaV1,type ReplicaOptionsV1} from "../../🔁️replica/🟦️.ts";
import {runReplicaCommandV1} from "../../🔁️replica/⌨️command/🟦️.ts";
import {attestPathV1,bucketMaskV1,bucketOfV1,codeDigestV1,compareCheckpointsV1,gitPathV1,recomputeBucketsV1,viewBucketsV1,type GapWindowV1,type FleetGuardsV1,createAttesterV1,fleetReportV1,judgeFleetV1,readAttestationV1,readAttestationsV1,readRetirementV1,readViewLinesV1,recomputeManifestV1,viewDiffV1,viewPathV1,writeAttestationV1,type AttesterGitV1,type AttesterViewV1,type FleetAttestationV1,type FleetReportV1,type FleetTermV1,type RetirementV1} from "../🟦️.ts";

setDefaultTimeout(60_000);
const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});
const scratch:string[]=[],python=testPythonV1(),temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`attest-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const headsOf=(heads:Readonly<Record<string,Readonly<{segment:number;bytes:number}>|undefined>>):Record<string,Readonly<{segment:number;bytes:number}>>=>Object.fromEntries(Object.entries(heads).flatMap(([origin,head])=>head===undefined?[]:[[origin,head]]));
const idle:LatencyReportV1["tiers"]["hot"]={passes:0,lastMs:null,dutyPercent:null},silent:LatencyReportV1={publishMs:{p50:null,p95:null},applyMs:{p50:null,p95:null},samples:{publish:0,apply:0},tiers:{hot:idle,sweep:idle,file:idle,walk:idle,view:idle}};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,selfCheck:async()=>({ok:true as const}),logFiles:[] as string[],viewMilliseconds:300,viewDebounceMilliseconds:50};
type Runner={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];controller:AbortController;done?:Promise<void>};
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=20000):Promise<void>=>{
 const end=Date.now()+milliseconds;
 while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}
};
const code=(label:string):string=>{const root=temp(label);mkdirSync(join(root,"x"));writeFileSync(join(root,"x","🟦️.ts"),"export const a=1;\n");return root;};
function runner(name:string,store:string,extra:Partial<ReplicaOptionsV1>={}):Runner{
 const worktree=temp(name);
 return {name,events:[],controller:new AbortController(),options:{worktree,storeRoot:store,originId:name,groupId:"g1",...fast,codeRoot:code(name),gitState:async()=>git,...extra}};
}
function start(who:Runner):void{who.done=runV1(who.options,who.controller.signal,event=>who.events.push(event));}
async function stop(who:Runner):Promise<void>{who.controller.abort();await who.done;}
async function fleet(names:readonly string[],extra:(name:string)=>Partial<ReplicaOptionsV1>=()=>({})){
 const store=temp("store"),runners=names.map(name=>runner(name,store,extra(name)));
 writeFileSync(join(runners[0]!.options.worktree,".gitignore"),"ignored\n");
 writeFileSync(join(runners[0]!.options.worktree,"a.txt"),"alpha\n");
 await seedV1(runners[0]!.options);
 for(const other of runners.slice(1))await joinV1(other.options);
 for(const who of runners)start(who);
 return {store,runners};
}
const agreed=async(store:string,count:number):Promise<boolean>=>{const report=await fleetReportV1(store);return report.ok&&report.rows.length===count;};

test("the fixture and a report satisfy the closed schema",()=>{
 const check=validator("Fixture");
 expect(check(fixture),JSON.stringify(check.errors?.[0])).toBe(true);
 const report=judgeFleetV1({now:fixture.verdicts[0]!.now,heads:headsOf(fixture.verdicts[0]!.heads),attestations:fixture.verdicts[0]!.attestations as unknown as Record<string,FleetAttestationV1>,retirements:{}});
 expect(validator("Report")(report)).toBe(true);
});

for(const row of fixture.digests)test(`manifest digest vector: ${row.name}`,()=>{
 expect(recomputeManifestV1(row.entries as FleetTermV1[]).manifest).toBe(row.manifest);
});

for(const row of fixture.code)test(`code digest vector: ${row.name}`,async()=>{
 const root=temp("code");
 for(const file of row.files){const target=join(root,...file.path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,file.text);}
 expect(await codeDigestV1(root)).toBe(row.code);
});

for(const row of fixture.verdicts)test(`fleet verdict vector: ${row.name}`,()=>{
 expect(judgeFleetV1({now:row.now,heads:headsOf(row.heads),attestations:row.attestations as unknown as Record<string,FleetAttestationV1>,retirements:row.retirements as Record<string,RetirementV1>,...("registered" in row?{registered:row.registered as string[]}:{})})).toEqual(row.expected as FleetReportV1);
});

for(const row of fixture.agreements)test(`fleet agreement vector: ${row.name}`,()=>{
 const report=judgeFleetV1({now:row.now,heads:headsOf(row.heads),attestations:row.attestations as unknown as Record<string,FleetAttestationV1>,retirements:row.retirements as Record<string,RetirementV1>,windows:row.windows as unknown as Record<string,GapWindowV1>});
 expect(report).toEqual(row.expected as unknown as FleetReportV1);
 expect(validator("Report")(report)).toBe(true);
});

for(const row of fixture.gitComparisons)test(`Git evidence vector: ${row.name}`,()=>{
 for(const [left,right] of [[row.left,row.right],[row.right,row.left]] as const){
  const result=compareCheckpointsV1({git:left},{git:right},row.left.wallMs,{});
  expect(result.known.has("git")).toBe(row.known);
  expect<unknown>(result.items.map(({field,same})=>({field,same}))).toEqual(row.compared);
 }
});

for(const row of fixture.bucketing.paths)test(`bucket of a path: ${row.path}`,()=>{expect(bucketOfV1(row.path)).toBe(row.bucket);});
for(const row of fixture.bucketing.gitPaths)test(`git path: ${row.path}`,()=>{expect(gitPathV1(row.path)).toBe(row.git);});
for(const row of fixture.bucketing.manifests)test(`manifest buckets vector: ${row.name}`,()=>{expect(recomputeBucketsV1(row.entries as FleetTermV1[])).toBe(row.buckets);});
for(const row of fixture.bucketing.views)test(`view buckets vector: ${row.name}`,()=>{
 expect(viewBucketsV1(row.lines)).toBe(row.buckets);
 expect(bucketMaskV1(row.pending)).toBe(row.mask);
});

for(const row of fixture.views)test(`view vector: ${row.name}`,()=>{
 const view=parseGitViewV1(row.porcelain);
 expect({staged:view.staged,modified:view.modified,deleted:view.deleted,untracked:view.untracked,lines:view.lines,digest:view.digest}).toEqual({staged:row.staged,modified:row.modified,deleted:row.deleted,untracked:row.untracked,lines:row.lines,digest:row.digest});
 expect(createHash("sha256").update(row.lines.join("\n")).digest("hex")).toBe(row.digest);
});

const viewOf=(lines:readonly string[]):AttesterViewV1=>({staged:0,modified:0,deleted:0,untracked:lines.length,digest:createHash("sha256").update(lines.join("\n")).digest("hex"),lines});

test("the Python oracle reproduces every digest and verdict of the fixture",()=>{
 const result=Bun.spawnSync([python!,"-I",join(import.meta.dirname,"🐍️.py")],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stdout.toString()+result.stderr.toString()).toBe(0);
 expect(result.stdout.toString()).toContain("0 mismatches");
});

test("the incremental manifest digest equals the recomputation after random writes, deletes, copies and clears",()=>{
 const book=new ReplicaBookV1(),table=new CompactTableV1(book,true),other=new CompactTableV1(book,true);
 let seed=7;
 const random=(limit:number):number=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%limit;};
 const sha=(n:number):string=>n.toString(16).padStart(64,"0");
 const terms=():FleetTermV1[]=>[...table.entries()].map(([path,entry])=>({path,operation:entry.operation,value:entry.operation==="put"?entry.sha256!:entry.operation==="link"?entry.target!:""}));
 for(let step=0;step<600;step++){
  const path=random(5)===0?`.cache/f/index/${random(10)}`:`dir/${random(40)}.txt`,kind=random(5),hlc=`${String(1_700_000_000_000+step).padStart(13,"0")}-000000-a`;
  if(kind===0)table.delete(path);
  else if(kind===1)table.set(path,{hlc,operation:"link",target:`t${random(9)}`,mtimeMs:0});
  else if(kind===2)table.set(path,{hlc,operation:"delete",mtimeMs:0});
  else if(kind===3){other.set(path,{hlc,operation:"put",sha256:sha(random(9)),bytes:1,executable:random(2)===0,mtimeMs:0});table.copyRow(other,book.intern(path));}
  else table.set(path,{hlc,operation:"put",sha256:sha(random(9)),bytes:1,executable:random(2)===0,mtimeMs:0});
  const expected=recomputeManifestV1(terms());
  expect(table.manifestHex()).toBe(expected.manifest);
  expect(table.bucketsHex()).toBe(recomputeBucketsV1(terms()));
  expect(table.shared).toBe(expected.files);
 }
 const adopted=new CompactTableV1(book,true);
 adopted.adopt(table.columns(table.flags.length),table.fold,table.shared);
 expect(adopted.bucketsHex()).toBe(table.bucketsHex());
 expect(adopted.manifestHex()).toBe(table.manifestHex());
 table.clear();
 expect(table.manifestHex()).toBe("0".repeat(64));
 expect(table.bucketsHex()).toBe("0".repeat(1024));
 expect(table.shared).toBe(0);
});

test("a reopened replica loads the same manifest digest from state and log",async()=>{
 const store=temp("store"),who=runner("a1",store);
 writeFileSync(join(who.options.worktree,".gitignore"),"ignored\n");
 for(let n=0;n<20;n++)writeFileSync(join(who.options.worktree,`f${n}.txt`),`v${n}\n`);
 await seedV1(who.options);
 const first=await openReplicaV1(who.options);
 const terms=[...first.files.entries()].map(([path,entry])=>({path,operation:entry.operation,value:entry.operation==="put"?entry.sha256!:entry.operation==="link"?entry.target!:""}));
 const held=first.files.manifestHex();
 expect(held).toBe(recomputeManifestV1(terms).manifest);
 expect(first.files.bucketsHex()).toBe(recomputeBucketsV1(terms));
 expect(held).not.toBe("0".repeat(64));
 await closeReplicaV1(first);
 const second=await openReplicaV1(who.options);
 expect(second.files.manifestHex()).toBe(held);
 expect(second.files.bucketsHex()).toBe(recomputeBucketsV1(terms));
 await closeReplicaV1(second);
});

test("three replicas on one store attest and the fleet command exits 0 with a schema-valid report",async()=>{
 const {store,runners}=await fleet(["a1","a2","a3"]);
 try{
  await until(()=>agreed(store,3));
  const report=await fleetReportV1(store);
  expect(new Set(report.rows.map(row=>row.platform))).toEqual(new Set([platformV1()]));
  const attestation=(await readAttestationV1(store,"a2"))!;
  expect(attestation.head).toEqual({ref:"refs/heads/main",commit:git.commit});
  expect(attestation.indexTree).toBe(git.indexTree);
  expect(attestation.files).toBeGreaterThanOrEqual(2);
  const lines:string[]=[],notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet","--worktree",runners[0]!.options.worktree,"--store",store,"--json"],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:line=>lines.push(line),notice:line=>notices.push(line)});
  expect(result.exitCode).toBe(0);
  expect(validator("Report")(JSON.parse(lines[0]!))).toBe(true);
  const human:string[]=[];
  await runReplicaCommandV1(["fleet","--worktree",runners[0]!.options.worktree,"--store",store,"--lang","de"],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>human.push(line)});
  expect(human.join("\n")).toContain("a1");
  expect(human.join("\n")).toContain("Flotte");
  const wait:string[]=[];
  const waited=await runReplicaCommandV1(["fleet","--worktree",runners[0]!.options.worktree,"--store",store,"--wait","5","--lang","en"],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>wait.push(line)});
  expect(waited.exitCode).toBe(0);
 }finally{for(const who of runners)await stop(who);}
});

test("a replica with another commit is named by origin and field and the command exits 1",async()=>{
 const {store,runners}=await fleet(["a1","a2","a3"],name=>name==="a3"?{gitState:async()=>({...git,commit:"b".repeat(40)})}:{});
 try{
  await until(async()=>{const report=await fleetReportV1(store);return report.rows.length===3&&report.rows.every(row=>row.settled)&&report.differing.length>0;});
  const report=await fleetReportV1(store);
  expect(report.ok).toBe(false);
  expect(report.differing).toEqual([{originId:"a3",field:"head.commit",value:"b".repeat(40),expected:git.commit}]);
  const notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet","--worktree",runners[0]!.options.worktree,"--store",store,"--lang","en"],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  expect(result.exitCode).toBe(1);
  expect(notices.join("\n")).toContain("a3 differs in head.commit");
  const started=Date.now(),waited=await runReplicaCommandV1(["fleet","--worktree",runners[0]!.options.worktree,"--store",store,"--wait","2","--lang","en"],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  expect(waited.exitCode).toBe(1);
  expect(Date.now()-started).toBeGreaterThanOrEqual(1500);
  expect(notices.join("\n")).toContain("Waiting for agreement");
 }finally{for(const who of runners)await stop(who);}
});

test("a replica whose file state differs is named by the manifest field",async()=>{
 const {store,runners}=await fleet(["a1","a2"]);
 try{
  await until(()=>agreed(store,2));
  await setSyncEnabledV1(runners[1]!.options.worktree,false);
  await until(async()=>(await readAttestationV1(store,"a2"))?.sync===false);
  writeFileSync(join(runners[0]!.options.worktree,"b.txt"),"beta\n");
  await until(async()=>(await readAttestationV1(store,"a1"))!.files===3);
  await setSyncEnabledV1(runners[1]!.options.worktree,true);
  await until(async()=>(await readAttestationV1(store,"a2"))!.files===3&&await agreed(store,2));
  const one=(await readAttestationV1(store,"a1"))!,two=(await readAttestationV1(store,"a2"))!;
  expect(one.manifest).toBe(two.manifest);
  const forged=judgeFleetV1({now:Date.now(),heads:{},attestations:{a1:one,a2:{...two,checkpoints:{...two.checkpoints,manifest:{...two.checkpoints!.manifest!,buckets:`ffff${two.checkpoints!.manifest!.buckets.slice(4)}`,cursors:one.checkpoints!.manifest!.cursors}}}},retirements:{}});
  expect(forged.differing.map(item=>item.field)).toEqual(["manifest"]);
 }finally{for(const who of runners)await stop(who);}
});

test("an attestation older than 60 s is stale and an origin behind the heads is still agreed with while the event it lacks is younger than 60 s",async()=>{
 const {store,runners}=await fleet(["a1","a2"]);
 try{await until(()=>agreed(store,2));}finally{for(const who of runners)await stop(who);}
 const later=await fleetReportV1(store,Date.now()+61_000);
 expect(later.ok).toBe(false);
 expect(later.rows.map(row=>[row.originId,row.fresh,row.verdict])).toEqual([["a1",false,"stale"],["a2",false,"stale"]]);
 const replica=await openReplicaV1({...runners[0]!.options,attestMilliseconds:undefined});
 writeFileSync(join(runners[0]!.options.worktree,"c.txt"),"gamma\n");
 await publishOnceV1(replica);
 await closeReplicaV1(replica);
 const behind=await fleetReportV1(store);
 expect(behind.rows.find(row=>row.originId==="a2")).toMatchObject({settled:true,verdict:"ok",behindOrigins:1,basis:"checkpoint"});
});

test("an origin with sync off still attests and is excluded from the agreement",async()=>{
 const store=temp("store"),who=runner("a1",store,{});
 writeFileSync(join(who.options.worktree,"a.txt"),"alpha\n");
 await seedV1(who.options);
 await setSyncEnabledV1(who.options.worktree,false);
 start(who);
 try{
  await until(async()=>(await readAttestationV1(store,"a1"))?.sync===false);
  const report=await fleetReportV1(store);
  expect(report.rows[0]!.verdict).toBe("off");
  expect(report.ok).toBe(false);
 }finally{await stop(who);}
});

test("the server receives fresh attestations while a replication turn is waiting",async()=>{
 const store=temp("heartbeat-store"),who=runner("a1",store,{gitView:async()=>null});
 let entered=false;
 const gate=Promise.withResolvers<void>();
 try{
  await seedV1(who.options);
  who.options={...who.options,storeGate:async()=>{entered=true;await gate.promise;}};
  start(who);
  await until(()=>entered);
  await until(async()=>(await readAttestationV1(store,who.name))!==undefined,3000);
  const file=attestPathV1(store,who.name),first=JSON.parse(readFileSync(file,"utf8")) as FleetAttestationV1;
  await until(async()=>((await readAttestationV1(store,who.name))?.wallMs??0)>first.wallMs,3000);
  const latest=JSON.parse(readFileSync(file,"utf8")) as FleetAttestationV1;
  expect(validator("Attestation")(latest)).toBe(true);
  expect(latest).toMatchObject({originId:who.name,manifest:first.manifest,cursors:first.cursors});
  expect(latest.wallMs).toBeGreaterThan(first.wallMs);
 }finally{gate.resolve();await stop(who);}
});

test("the readiness sidecar starts while a later replication turn is waiting",async()=>{
 const store=temp("sidecar-store"),gate=Promise.withResolvers<void>();
 let rounds=0;
 const who=runner("a1",store,{gitView:async()=>null,storeGate:async()=>{if(++rounds>1)await gate.promise;},sidecars:{
  enabled:true,startAfterSeconds:2,graceSeconds:1,restart:{initialMilliseconds:60,maximumMilliseconds:240,stableMilliseconds:60_000},
  children:[{name:"readiness",script:"child.ts",arguments:["stay"],roles:["member"],requires:"none",enabled:true}],
 }});
 const script=join(import.meta.dir,"../../🔁️replica/🧷️sidecars/🧪️tests/🧬️child.ts");
 writeFileSync(join(who.options.worktree,"child.ts"),readFileSync(script));
 try{
  await seedV1(who.options);
  start(who);
  await until(()=>rounds>1);
  const log=join(who.options.worktree,".cache","f","sidecars","readiness.log");
  await until(()=>existsSync(log)&&readFileSync(log,"utf8").includes("child stay started"),5000);
  expect(rounds).toBe(2);
 }finally{gate.resolve();await stop(who);}
});

test("nothing is written while the store is unreachable and the attestation is written at most once per heartbeat when nothing changed",async()=>{
 const store=temp("store");
 let reachable=false,clock=1_000_000,asked=0;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("probe"),git:async()=>{asked++;return git;},reachable:async()=>reachable});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 expect(await attester.tick(snapshot)).toBe("idle");
 expect(existsSync(attestPathV1(store,"a1"))).toBe(false);
 reachable=true;
 expect(await attester.tick(snapshot)).toBe("written");
 clock+=1000;
 expect(await attester.tick(snapshot)).toBe("idle");
 clock+=1000;
 expect(await attester.tick({...snapshot,files:2})).toBe("written");
 clock+=1000;
 expect(await attester.tick({...snapshot,files:3})).toBe("written");
 clock+=4999;
 expect(await attester.tick({...snapshot,files:3})).toBe("idle");
 clock+=1;
 expect(await attester.tick({...snapshot,files:3})).toBe("written");
 expect(asked).toBeLessThanOrEqual(3);
 expect((await readAttestationV1(store,"a1"))!.files).toBe(3);
});

test("a self-check that only timed out is repeated after a pause and the machine restarts once it passes, while a check that blames the code is never repeated",async()=>{
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 const run=async(label:string,first:string,retry:number)=>{
  const root=code(label);
  let clock=1_000_000,calls=0;
  const attester=createAttesterV1({storeRoot:temp("store"),originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,retryMilliseconds:retry,jitterSpanMilliseconds:0,minimumUptimeMilliseconds:0,codeRoot:root,git:async()=>git,reachable:async()=>true,check:async()=>{calls++;return calls===1?{ok:false as const,reason:first}:{ok:true as const};}});
  await attester.start();
  writeFileSync(join(root,"x","🟦️.ts"),"export const a=2;\n");
  const answers:string[]=[];
  for(let step=0;step<200;step++){clock+=5000;answers.push(await attester.tick(snapshot));await Bun.sleep(1);if(answers.includes("restart"))break;}
  return {calls,answers,pending:attester.pending()};
 };
 const slow=await run("slow-check","self-check timed out after 180000 ms",60_000);
 expect(slow.answers).toContain("restart");
 expect(slow.calls).toBe(2);
 expect(slow.pending).toBeNull();
 const blamed=await run("blamed-check","self-check exited with 1: SyntaxError",60_000);
 expect(blamed.answers).not.toContain("restart");
 expect(blamed.calls).toBe(1);
 expect(blamed.pending).toMatchObject({ok:false,reason:"self-check exited with 1: SyntaxError"});
});

test("the daemon restarts exactly once after the code changed and stayed the same, and not while it still changes",async()=>{
 const store=temp("store"),root=code("code"),who=runner("a1",store,{codeRoot:root,restartStableMilliseconds:600});
 writeFileSync(join(who.options.worktree,"a.txt"),"alpha\n");
 await seedV1(who.options);
 start(who);
 let finished=false;
 void who.done!.then(()=>{finished=true;});
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 const first=(await readAttestationV1(store,"a1"))!.code;
 for(let n=0;n<12;n++){writeFileSync(join(root,"x","🟦️.ts"),`export const a=${n+2};\n`);await Bun.sleep(100);}
 expect(finished).toBe(false);
 expect(who.events.filter(event=>event.event==="restarting")).toHaveLength(0);
 await until(()=>finished);
 expect(who.events.filter(event=>event.event==="restarting")).toHaveLength(1);
 expect(who.events.at(-1)!.event).toBe("stopped");
 expect(who.events.findIndex(event=>event.event==="restarting")).toBeLessThan(who.events.findIndex(event=>event.event==="stopped"));
 expect(await codeDigestV1(root)).not.toBe(first);
 const again=runner("a1",store,{worktree:who.options.worktree,codeRoot:root});
 start(again);
 await Bun.sleep(600);
 expect(again.events.filter(event=>event.event==="restarting")).toHaveLength(0);
 await stop(again);
});

test("the relay copies attest.json with the head and a compaction keeps it",async()=>{
 const one=temp("s1"),two=temp("s2"),who=runner("a1",one,{});
 writeFileSync(join(who.options.worktree,"a.txt"),"alpha\n");
 await seedV1(who.options);
 start(who);
 try{
  await until(async()=>(await readAttestationV1(one,"a1"))!==undefined);
  await relayOnceV1(two,one,await homedOriginsV1(one,"g1"));
  const copied=await readAttestationV1(two,"a1");
  expect(copied).toBeDefined();
  const text=readFileSync(attestPathV1(two,"a1"),"utf8");
  expect(JSON.parse(text)).toMatchObject({originId:"a1",wallMs:copied!.wallMs,head:copied!.head,indexTree:copied!.indexTree});
  const before=copied!.wallMs;
  await until(async()=>(await readAttestationV1(one,"a1"))!.wallMs>before);
  await relayOnceV1(two,one,await homedOriginsV1(one,"g1"));
  expect((await readAttestationV1(two,"a1"))!.wallMs).toBeGreaterThan(before);
 }finally{await stop(who);}
 await compactV1(one,"a1",0);
 expect(existsSync(attestPathV1(one,"a1"))).toBe(true);
});

test("AppleDouble names next to origin directories are not origins",async()=>{
 const store=temp("store"),origins=join(store,"v1","origins");
 mkdirSync(join(origins,"a1"),{recursive:true});mkdirSync(join(origins,"b1"));
 writeFileSync(join(origins,"._a1"),"x");mkdirSync(join(origins,"._b1"));writeFileSync(join(origins,".DS_Store"),"x");
 expect(await listOriginsV1(store)).toEqual(["a1","b1"]);
 expect(readdirSync(origins).sort()).toEqual([".DS_Store","._a1","._b1","a1","b1"]);
 await writeAttestationV1(store,{version:1,originId:"a1",groupId:"g1",platform:"linux",wallMs:1,hlc:"0000000000001-000000-a1",sync:true,code:"d".repeat(64),cursors:{},files:0,manifest:"0".repeat(64),head:null,indexTree:null,view:null,guards:null,latency:silent,queue:{incoming:0}});
 expect(Object.keys(await readAttestationsV1(store))).toEqual(["a1"]);
});

test("a replica that cannot read blobs is named by a blobs warning while the failure is recent, and the verdict stays ok",async()=>{
 const store=temp("store"),now=1_700_000_000_000,common={version:1 as const,groupId:"g1",platform:"linux",wallMs:now,hlc:"1700000000000-000000-a1",sync:true,code:"d".repeat(64),cursors:{},files:1,manifest:"1".repeat(64),head:null,indexTree:null,view:null,queue:{incoming:0},guards:null,latency:silent};
 await writeAttestationV1(store,{...common,originId:"a1",failures:[{kind:"blob-wanted",count:3,lastMs:now-5000}]});
 await writeAttestationV1(store,{...common,originId:"a2",failures:[{kind:"blob-wanted",count:1,lastMs:now-600_000}]});
 await writeAttestationV1(store,{...common,originId:"a3"});
 const report=await fleetReportV1(store,now);
 expect(report.warnings.filter(item=>item.kind==="blobs")).toEqual([{originId:"a1",kind:"blobs",detail:"3"}]);
 expect(validator("Report")(report)).toBe(true);
});

test("the attestation of code that predates the guards is present with unknown guards, compared on everything else and warned about",async()=>{
 const store=temp("store"),now=1_700_000_000_000,common={version:1 as const,groupId:"g1",platform:"linux",wallMs:now,hlc:"1700000000000-000000-a1",sync:true,code:"d".repeat(64),cursors:{},files:1,manifest:"1".repeat(64),head:null,indexTree:null,view:null,queue:{incoming:0}};
 await writeAttestationV1(store,{...common,originId:"a1",guards:null,latency:silent});
 const older=(name:string,extra:object={})=>{
  const directory=join(store,"v1","origins",name);mkdirSync(directory,{recursive:true});
  writeFileSync(join(directory,"attest.json"),JSON.stringify({...common,originId:name,latency:silent,...extra}));
 };
 older("a2");older("a3",{manifest:"2".repeat(64)});
 const legacy=(await readAttestationV1(store,"a2"))!;
 expect(legacy).toMatchObject({originId:"a2",guards:null,legacy:true});
 expect((await readAttestationV1(store,"a1"))!.legacy).toBeUndefined();
 const bare=join(store,"v1","origins","a4");mkdirSync(bare,{recursive:true});
 const {latency:_latency,...withoutLatency}={...common,originId:"a4",latency:silent};
 writeFileSync(join(bare,"attest.json"),JSON.stringify(withoutLatency));
 expect(await readAttestationV1(store,"a4")).toMatchObject({legacy:true,guards:null,latency:silent});
 const report=await fleetReportV1(store,now);
 expect(Object.fromEntries(report.rows.map(row=>[row.originId,row.verdict]))).toEqual({a1:"ok",a2:"ok",a3:"diverged",a4:"ok"});
 expect(report.rows.some(row=>row.verdict==="missing")).toBe(false);
 expect(report.rows.find(row=>row.originId==="a3")!.differs).toContain("manifest");
 expect(report.rows.find(row=>row.originId==="a2")!.verdict).toBe("ok");
 expect(report.warnings.filter(item=>item.kind==="guards-unknown").map(item=>item.originId).sort()).toEqual(["a2","a3","a4"]);
 expect(validator("Report")(report)).toBe(true);
});

test("a replica whose manifest silently drifted from the settled fleet repairs itself, reports it once and agrees again",async()=>{
 let victim:ReplicaV1|undefined;
 const {store,runners}=await fleet(["a1","a2","a3"],name=>name==="a2"?{driftMilliseconds:300,opened:replica=>{victim=replica;}}:{driftMilliseconds:300});
 try{
  await until(()=>agreed(store,3));
  const a2=runners[1]!,id=victim!.book.find("a.txt")!,real=victim!.files.get("a.txt")!;
  victim!.files.write(id,{...real,hlc:"0000000000001-000000-g1-seed",sha256:"e".repeat(64)});
  victim!.manifest.set("ghost.txt",{hlc:"0000000000003-000000-g1-seed",operation:"put",sha256:"d".repeat(64),bytes:1,executable:false});
  victim!.files.set("phantom.txt",{hlc:"0000000000002-000000-g1-seed",operation:"put",sha256:"f".repeat(64),bytes:1,executable:false});
  await until(async()=>(await fleetReportV1(store)).rows.some(row=>row.originId==="a2"&&row.verdict==="diverged"),20000).catch(()=>{});
  await until(()=>a2.events.some(event=>event.event==="drift"),30000);
  await until(()=>agreed(store,3),30000);
  expect(victim!.files.get("a.txt")).toMatchObject({hlc:real.hlc,sha256:real.sha256});
  expect(victim!.files.get("phantom.txt")).toBeUndefined();
  expect(victim!.manifest.get("ghost.txt")).toBeUndefined();
  const drifts=a2.events.filter(event=>event.event==="drift");
  expect(drifts[0]).toMatchObject({reason:"manifest"});
  expect(a2.events.some(event=>event.event==="repaired"&&event.reason==="drift")).toBe(true);
  expect(runners.filter(who=>who!==a2).flatMap(who=>who.events).some(event=>event.event==="drift")).toBe(false);
 }finally{for(const who of runners)await stop(who);}
});

async function command(who:Runner,args:readonly string[],language:string="en"){
 const lines:string[]=[],notices:string[]=[];
 const result=await runReplicaCommandV1([...args,"--worktree",who.options.worktree,"--store",who.options.storeRoot,"--lang",language],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:line=>lines.push(line),notice:line=>notices.push(line)});
 return {exitCode:result.exitCode,lines,text:notices.join("\n")};
}

test("retire makes a silent origin stop blocking the fleet, refuses the unsafe cases and undo re-admits it",async()=>{
 const {store,runners}=await fleet(["a1","a2"]),[a1]=runners as [Runner,Runner];
 const silent=runner("s1",store,{});
 await joinV1(silent.options);
 writeFileSync(join(silent.options.worktree,"s1.txt"),"silent\n");
 await seedV1(silent.options);
 try{
  await until(async()=>{const report=await fleetReportV1(store);return report.rows.length===3&&report.rows.filter(row=>row.verdict==="ok").length===2&&report.rows.some(row=>row.originId==="s1"&&row.verdict==="missing");});
  const before=await command(a1,["fleet","--json"]);
  expect(before.exitCode).toBe(1);
  expect((JSON.parse(before.lines[0]!) as FleetReportV1).rows.find(row=>row.originId==="s1")!.verdict).toBe("missing");
  expect((await command(a1,["retire","a1"])).text).toContain("cannot retire itself");
  expect((await command(a1,["retire","a1"])).exitCode).toBe(1);
  expect((await command(a1,["retire","zz9"])).text).toContain("is not in the store");
  expect((await command(a1,["retire","a2"])).text).toContain("looks alive");
  expect(await readRetirementV1(store,"a2")).toBeUndefined();
  await expect(command(a1,["retire"])).rejects.toThrow("needs the origin");
  await expect(command(a1,["retire","s1","--undo","--force"])).rejects.toThrow("--undo cannot be combined");
  expect((await command(a1,["retire","s1","--undo"])).text).toContain("nothing to undo");
  const retired=await command(a1,["retire","s1","--reason","left the fleet"]);
  expect(retired.exitCode).toBe(0);
  const record=(await readRetirementV1(store,"s1"))!;
  expect([record.by,record.reason,record.retired,record.version]).toEqual(["a1","left the fleet",true,1]);
  expect(validator("Retirement")(JSON.parse(readFileSync(join(store,"v1","origins","s1","retired.json"),"utf8")))).toBe(true);
  const after=await command(a1,["fleet","--json"]);
  expect(after.exitCode).toBe(0);
  const parsed=JSON.parse(after.lines[0]!) as FleetReportV1;
  expect(validator("Report")(parsed)).toBe(true);
  expect(parsed.rows.map(row=>[row.originId,row.verdict])).toEqual([["a1","ok"],["a2","ok"],["s1","retired"]]);
  expect((await command(a1,["fleet"],"de")).text).toContain("stillgelegt");
  expect((await command(a1,["retire","s1"])).text).toContain("already retired");
  expect((await readRetirementV1(store,"s1"))!.hlc).toBe(record.hlc);
  const status=JSON.parse((await command(a1,["status","--json"])).lines[0]!) as {retired:unknown;origins:{origin:string;retired:boolean}[]};
  expect(status.retired).toBeNull();
  expect(status.origins.find(entry=>entry.origin==="s1")!.retired).toBe(true);
  expect((await command(a1,["status"],"de")).text).toContain("stillgelegt");
  const undone=await command(a1,["retire","s1","--undo"]);
  expect(undone.exitCode).toBe(0);
  expect((await readRetirementV1(store,"s1"))!.retired).toBe(false);
  expect((await readRetirementV1(store,"s1"))!.hlc > record.hlc).toBe(true);
  expect((await command(a1,["fleet","--json"])).exitCode).toBe(1);
 }finally{for(const who of runners)await stop(who);}
});

test("a live origin is retired only with --force, then its daemon parks itself and a retire --undo from another machine lets it work again",async()=>{
 const {store,runners}=await fleet(["a1","a2"]),[a1,a2] = runners as [Runner,Runner];
 try{
  await until(()=>agreed(store,2));
  const done=await command(a1,["retire","a2","--force","--reason","moved away"]);
  expect(done.exitCode).toBe(0);
  await until(()=>a2.events.some(event=>event.event==="retired"));
  writeFileSync(join(a2.options.worktree,"parked.txt"),"never\n");
  writeFileSync(join(a1.options.worktree,"live.txt"),"live\n");
  await until(async()=>(await readAttestationV1(store,"a1"))!.files>=3);
  await Bun.sleep(800);
  expect(a2.events.filter(event=>event.event==="retired")).toHaveLength(1);
  expect(a2.events.some(event=>event.event==="published")).toBe(false);
  expect(existsSync(join(a1.options.worktree,"parked.txt"))).toBe(false);
  expect(existsSync(join(a2.options.worktree,"live.txt"))).toBe(false);
  await until(async()=>(await fleetReportV1(store)).ok);
  const report=await fleetReportV1(store);
  expect(report.rows.map(row=>[row.originId,row.verdict])).toEqual([["a1","ok"],["a2","retired"]]);
  expect((await attestedSync(store,"a2"))).toBe(false);
  const status=JSON.parse((await command(a2,["status","--json"])).lines[0]!) as {retired:{by:string;reason:string}|null};
  expect([status.retired?.by,status.retired?.reason]).toEqual(["a1","moved away"]);
  expect((await command(a2,["status"],"en")).text).toContain("was retired by a1");
  expect((await command(a1,["retire","a2","--undo"])).exitCode).toBe(0);
  await until(()=>existsSync(join(a1.options.worktree,"parked.txt"))&&existsSync(join(a2.options.worktree,"live.txt")));
  await until(()=>agreed(store,2));
  expect(a2.events.filter(event=>event.event==="retired")).toHaveLength(1);
 }finally{for(const who of runners)await stop(who);}
});

const attestedSync=async(store:string,origin:string):Promise<boolean|undefined>=>(await readAttestationV1(store,origin))?.sync;

test("the view is asked at start and afterwards only after a change, at most every 120 s, never on a timer while idle and never concurrently, and view.txt is written only when the digest changed",async()=>{
 const store=temp("store");
 let clock=1_000_000,asked=0,lines=["?? a"],gate:Promise<void>|undefined;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("view"),git:async()=>git,reachable:async()=>true,view:async()=>{asked++;await gate;return viewOf(lines);},viewMilliseconds:120_000,viewDebounceMilliseconds:5000});
 await attester.start();
 let snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1",activity:0};
 const tick=async(milliseconds:number)=>{clock+=milliseconds;return attester.tick(snapshot);};
 expect(await attester.tick(snapshot)).toBe("written");
 await tick(1000);
 const stamp=()=>statSync(viewPathV1(store,"a1")).mtimeMs;
 expect(asked).toBe(1);
 expect((await readAttestationV1(store,"a1"))!.view).toEqual({staged:0,modified:0,deleted:0,untracked:1,digest:viewOf(["?? a"]).digest});
 expect(await readViewLinesV1(store,"a1")).toEqual(["?? a"]);
 const first=stamp();
 for(let n=0;n<400;n++)await tick(1000);
 expect(asked).toBe(1);
 snapshot={...snapshot,activity:1};
 await tick(1000);await tick(3000);
 expect(asked).toBe(1);
 await tick(2000);
 expect(asked).toBe(2);
 expect(stamp()).toBe(first);
 lines=["?? a","?? b"];
 snapshot={...snapshot,files:2};
 for(let n=0;n<10;n++)await tick(1000);
 expect(asked).toBe(2);
 for(let n=0;n<100;n++){snapshot={...snapshot,activity:snapshot.activity+1};await tick(1000);}
 expect(asked).toBe(2);
 for(let n=0;n<20;n++){snapshot={...snapshot,activity:snapshot.activity+1};await tick(1000);}
 expect(asked).toBe(3);
 await Bun.sleep(10);
 await tick(1000);
 expect(await readViewLinesV1(store,"a1")).toEqual(["?? a","?? b"]);
 expect((await readAttestationV1(store,"a1"))!.view!.digest).toBe(viewOf(lines).digest);
 expect(await readViewLinesV1(store,"a1","0".repeat(64))).toBeUndefined();
 let release!:()=>void;
 gate=new Promise<void>(done=>{release=done;});
 lines=["?? a","?? b","?? c"];
 snapshot={...snapshot,activity:snapshot.activity+1};
 for(let n=0;n<130;n++)await tick(1000);
 expect(asked).toBe(4);
 for(let n=0;n<40;n++){snapshot={...snapshot,activity:snapshot.activity+1};await tick(1000);}
 expect(asked).toBe(4);
 release();
 await Bun.sleep(10);
 expect(await tick(1000)).toBe("written");
 expect(await readViewLinesV1(store,"a1")).toEqual(lines);
});

test("changes named by the replica update the view incrementally, mark it stale until it reflects them and fall back to a complete pass for an overflow or a verification",async()=>{
 const store=temp("store");
 let clock=1_000_000,full=0,gate:Promise<void>|undefined,delta:{paths:readonly string[];restage:{before:string|null;after:string|null}|null}[]=[],lines=["?? a"],gitNow=git,drained:{paths:string[];overflow:boolean}={paths:[],overflow:false};
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("delta"),git:async()=>gitNow,reachable:async()=>true,view:async(_,request)=>{if(request===undefined)full++;else delta.push({paths:request.paths,restage:request.restage});await gate;if(request===undefined)return viewOf(lines);lines=[...lines.filter(line=>!request.paths.includes(line.slice(3))),...request.paths.map(path=>`?? ${path}`)];return viewOf(lines);},changes:()=>{const taken=drained;drained={paths:[],overflow:false};return taken;},viewMilliseconds:120_000,viewDebounceMilliseconds:5000,deltaDebounceMilliseconds:1000,deltaLimit:3});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1",activity:0};
 const tick=async(milliseconds:number)=>{clock+=milliseconds;return attester.tick(snapshot);};
 const stale=async():Promise<boolean|undefined>=>(await readAttestationV1(store,"a1"))!.view!.stale;
 await attester.tick(snapshot);await Bun.sleep(5);
 expect(await tick(1000)).toBe("written");
 expect(full).toBe(1);expect(await stale()).toBeUndefined();
 drained={paths:["b"],overflow:false};
 await tick(1000);
 expect(await tick(5000)).toBe("written");
 expect(await stale()).toBe(true);
 await tick(1000);await Bun.sleep(5);
 expect(delta).toEqual([{paths:["b"],restage:null}]);
 expect(full).toBe(1);
 await tick(1000);
 expect(await tick(5000)).toBe("written");
 expect(await stale()).toBeUndefined();
 expect((await readAttestationV1(store,"a1"))!.view!.digest).toBe(viewOf(["?? a","?? b"]).digest);
 expect(await readViewLinesV1(store,"a1")).toEqual(["?? a","?? b"]);
 let release!:()=>void;
 gate=new Promise<void>(done=>{release=done;});
 drained={paths:["c"],overflow:false};
 await tick(1000);await tick(1500);await tick(1200);
 expect(delta).toHaveLength(2);
 expect(await stale()).toBe(true);
 drained={paths:["d"],overflow:false};
 await tick(1000);
 release();await Bun.sleep(5);
 gate=undefined;
 await tick(1000);
 expect(await stale()).toBe(true);
 await tick(20_000);await tick(1500);await Bun.sleep(5);await tick(1000);await tick(6000);
 expect(delta.map(item=>item.paths)).toEqual([["b"],["c"],["d"]]);
 expect(await stale()).toBeUndefined();
 drained={paths:["e","f","g","h"],overflow:false};
 await tick(1000);await tick(6000);await Bun.sleep(5);await tick(6000);
 expect(full).toBe(2);
 expect(delta).toHaveLength(3);
 gitNow={...git,indexTree:"d".repeat(40)};
 await tick(5000);await tick(1500);await tick(1500);await Bun.sleep(5);
 expect(delta.at(-1)).toEqual({paths:[],restage:{before:"c".repeat(40),after:"d".repeat(40)}});
 expect(full).toBe(2);
 snapshot.activity=1;
 for(let n=0;n<60;n++)await tick(1000);
 expect(full).toBe(2);
 for(let n=0;n<80;n++)await tick(1000);
 await Bun.sleep(5);
 expect(full).toBe(3);
});

test("a slow view stretches the spacing to nineteen times its duration and reports its cost",async()=>{
 const store=temp("store"),seen:string[]=[];
 let clock=1_000_000,asked=0;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,codeRoot:code("slow"),git:async()=>git,reachable:async()=>true,view:async()=>{asked++;clock+=30_000;return viewOf(["?? slow"]);},viewMilliseconds:120_000,viewDebounceMilliseconds:5000,timed:(name,cost,delay)=>{seen.push(`${name}:${cost}:${delay}`);}});
 await attester.start();
 let snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1",activity:0};
 await attester.tick(snapshot);await Bun.sleep(5);await attester.tick(snapshot);
 expect(asked).toBe(1);expect(seen).toEqual(["view:30000:570000"]);
 snapshot={...snapshot,activity:1};
 for(let n=0;n<400;n++){clock+=1000;await attester.tick(snapshot);}
 expect(asked).toBe(1);
 for(let n=0;n<200;n++){clock+=1000;await attester.tick(snapshot);}
 await Bun.sleep(5);
 expect(asked).toBe(2);
});

test("the attestation carries the passive latency of the replica and refreshes it with the heartbeat, not with every sample",async()=>{
 const store=temp("store");
 let clock=1_000_000,report={...silent,publishMs:{p50:12,p95:40},samples:{publish:3,apply:0}};
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,codeRoot:code("latency"),git:async()=>({...git,present:false,head:null,commit:null,indexTree:null}),reachable:async()=>true,latency:()=>report});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 expect(await attester.tick(snapshot)).toBe("written");
 expect((await readAttestationV1(store,"a1"))!.latency).toEqual(report);
 report={...silent,publishMs:{p50:99,p95:100},applyMs:{p50:5,p95:6},samples:{publish:4,apply:1},tiers:{...silent.tiers,sweep:{passes:3,lastMs:410,dutyPercent:9.1}}};
 clock+=2000;
 expect(await attester.tick(snapshot)).toBe("idle");
 clock+=4000;
 expect(await attester.tick(snapshot)).toBe("written");
 expect((await readAttestationV1(store,"a1"))!.latency).toEqual(report);
 expect(validator("Latency")(report)).toBe(true);
 expect(validator("Latency")({...report,samples:{publish:-1,apply:0}})).toBe(false);
});

test("the daemon attests liveness while the initial Git sample is blocked",async()=>{
 const entered=Promise.withResolvers<void>(),release=Promise.withResolvers<void>(),database=new Database(":memory:");
 const {store,runners}=await fleet(["a1"],()=>({gitState:async()=>{entered.resolve();await release.promise;return git;}}));
 const [who]=runners as [Runner];
 const results=database.query("select case value when 'certified' then ?1 else null end as tree, case value when 'certified' then ?2 else null end as head from json_each(?3) order by key").all(git.indexTree,git.commit,JSON.stringify(observationFixture.firstGit));
 const actual=async()=>{
  const row=await readAttestationV1(store,who.name);
  return {tree:row?.indexTree??null,head:row?.head?.commit??null};
 };
 try{
  await entered.promise;
  await until(async()=>(await readAttestationV1(store,who.name))!==undefined,3000);
  expect<unknown>(await actual()).toEqual(results[0]);
  expect((await readAttestationV1(store,who.name))?.view).toBeNull();
  release.resolve();
  await until(async()=>(await actual()).tree===git.indexTree,3000);
  expect<unknown>(await actual()).toEqual(results[1]);
 }finally{release.resolve();database.close();await stop(who);}
});

test("a certified stage is reported while its initial worktree view is still reading",async()=>{
 const store=temp("stage-first"),database=new Database(":memory:"),release=Promise.withResolvers<void>(),measured=Promise.withResolvers<void>();
 const view=viewOf(["?? worker.txt"]);
 let clock=1_000_000,calls=0;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",now:()=>clock,codeRoot:code("stage-first"),git:async()=>git,reachable:async()=>true,view:async()=>{calls++;await release.promise;return view;},onView:()=>measured.resolve()});
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 const oracle=database.query("select ?1 as tree, case value when 'measured' then ?2 else null end as view from json_each(?3) order by key").all(git.indexTree,view.digest,JSON.stringify(observationFixture.firstView));
 const observed=async()=>{
  const row=await readAttestationV1(store,"a1");
  return {tree:row?.indexTree??null,view:row?.view?.digest??null};
 };
 try{
  await attester.start();
  expect(await attester.tick(snapshot)).toBe("written");
  expect<unknown>(await observed()).toEqual(oracle[0]);
  expect(existsSync(viewPathV1(store,"a1"))).toBe(false);
  expect(calls).toBe(1);
  release.resolve();await measured.promise;
  clock+=1000;
  expect(await attester.tick(snapshot)).toBe("written");
  expect<unknown>(await observed()).toEqual(oracle[1]);
  expect(await readViewLinesV1(store,"a1",view.digest)).toEqual(view.lines);
  expect(calls).toBe(1);
 }finally{release.resolve();database.close();}
});

test("an attester without git states no view and writes no view.txt, and a view that cannot be read keeps the last one",async()=>{
 const store=temp("store");
 let clock=1_000_000;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,codeRoot:code("noview"),git:async()=>({present:false,head:null,commit:null,indexTree:null}),reachable:async()=>true,view:async()=>viewOf(["?? never"])});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 expect(await attester.tick(snapshot)).toBe("written");
 expect((await readAttestationV1(store,"a1"))!.view).toBeNull();
 expect(existsSync(viewPathV1(store,"a1"))).toBe(false);
 const other=temp("store");
 let fail=false;
 const keeper=createAttesterV1({storeRoot:other,originId:"a2",groupId:"g1",platform:"linux",now:()=>clock,codeRoot:code("keep"),git:async()=>git,reachable:async()=>true,view:async()=>{if(fail)throw Error("boom");return viewOf(["?? kept"]);},viewMilliseconds:1000});
 await keeper.start();
 expect(await keeper.tick(snapshot)).toBe("written");
 clock+=1000;await keeper.tick(snapshot);
 expect((await readAttestationV1(other,"a2"))!.view!.digest).toBe(viewOf(["?? kept"]).digest);
 fail=true;clock+=2000;
 await keeper.tick(snapshot);
 clock+=1000;
 await keeper.tick({...snapshot,files:2});
 expect((await readAttestationV1(other,"a2"))!.view!.digest).toBe(viewOf(["?? kept"]).digest);
});

test("checkpoints are taken only at a cut with nothing left to apply, copy the cursors, widen the span of a git reading over the events that arrive meanwhile, and are absent without a bucket digest",async()=>{
 const store=temp("store");
 let clock=1_000_000,gate:Promise<void>|undefined,release!:()=>void;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("cut"),git:async()=>{await gate;return git;},reachable:async()=>true});
 await attester.start();
 const cursors:Record<string,{segment:number;bytes:number}>={a1:{segment:0,bytes:100}},buckets="0".repeat(1024);
 const tick=async(milliseconds:number,extra:Record<string,unknown>={})=>{clock+=milliseconds;return attester.tick({sync:true,files:1,manifest:"1".repeat(64),buckets,cursors,incoming:0,hlc:"0000001000000-000000-a1",...extra} as never);};
 const read=async()=>(await readAttestationV1(store,"a1"))!.checkpoints!;
 await tick(1000,{incoming:3});await Bun.sleep(5);
 expect(await tick(5000,{incoming:3})).toBe("written");
 expect(await read()).toEqual({});
 expect(await tick(5000)).toBe("written");
 expect((await read()).manifest).toMatchObject({cursors:{a1:{segment:0,bytes:100}},wallMs:clock,files:1,buckets});
 cursors.a1={segment:0,bytes:200};
 await tick(1000,{incoming:2});
 expect(await tick(5000,{incoming:2})).toBe("written");
 expect((await read()).manifest!.cursors).toEqual({a1:{segment:0,bytes:100}});
 expect((await read()).git).toMatchObject({cursors:{a1:{segment:0,bytes:100}},head:git.commit,indexTree:git.indexTree});
 expect((await read()).git!.until).toEqual({a1:{segment:0,bytes:200}});
 gate=new Promise<void>(done=>{release=done;});
 await tick(1000);
 await tick(5000);
 cursors.a1={segment:0,bytes:300};
 await tick(1000);
 release();await Bun.sleep(5);gate=undefined;
 await tick(1000);
 expect(await tick(6000)).toBe("written");
 expect((await read()).git).toMatchObject({cursors:{a1:{segment:0,bytes:200}},until:{a1:{segment:0,bytes:300}}});
 expect(validator("Attestation")((await readAttestationV1(store,"a1"))!)).toBe(true);
 const older=createAttesterV1({storeRoot:store,originId:"a2",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("cut2"),git:async()=>git,reachable:async()=>true});
 await older.start();
 await older.tick({sync:true,files:1,manifest:"1".repeat(64),cursors,incoming:0,hlc:"0000001000000-000000-a2"});await Bun.sleep(5);clock+=6000;
 expect(await older.tick({sync:true,files:1,manifest:"1".repeat(64),cursors,incoming:0,hlc:"0000001000000-000000-a2"})).toBe("written");
 expect((await readAttestationV1(store,"a2"))!.checkpoints).toBeUndefined();
});

test("the attester withdraws a Git checkpoint while its index tree is unavailable and preserves HEAD",async()=>{
 const store=temp("store");
 let clock=1_000_000,reading:AttesterGitV1=git;
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,codeRoot:code("index-evidence"),git:async()=>reading,reachable:async()=>true});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),buckets:"0".repeat(1024),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 const settle=async()=>{
  for(let turn=0;turn<3;turn++){clock+=1000;await attester.tick(snapshot);}
  return (await readAttestationV1(store,"a1"))!;
 };
 expect((await settle()).checkpoints?.git?.indexTree).toBe(git.indexTree);
 const commit="b".repeat(40);
 reading={...git,commit,indexTree:null};
 const unavailable=await settle();
 expect(unavailable.head?.commit).toBe(commit);
 expect(unavailable.indexTree).toBeNull();
 expect(unavailable.checkpoints?.git).toBeUndefined();

 reading={...git,commit:null,indexTree:null};
 expect((await settle()).checkpoints?.git).toBeUndefined();

 reading={present:false,head:null,commit:null,indexTree:null};
 expect((await settle()).checkpoints?.git).toMatchObject({head:null,indexTree:null});
});

test("a fetched remote ref or object does not hold the git reading back, an unpublished change of the stage does for 30 s and then counts as noise",async()=>{
 const store=temp("store");
 let clock=1_000_000,hold:string[]=[".git/refs/remotes/origin/main",".git/objects/ab/cdef"];
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("hold"),git:async()=>git,reachable:async()=>true,unsettled:()=>hold});
 await attester.start();
 const tick=async(milliseconds:number)=>{clock+=milliseconds;const result=await attester.tick({sync:true,files:1,manifest:"1".repeat(64),buckets:"0".repeat(1024),cursors:{a1:{segment:0,bytes:100}},incoming:0,hlc:"0000001000000-000000-a1"} as never);await Bun.sleep(3);return result;};
 const taken=async():Promise<number|undefined>=>(await readAttestationV1(store,"a1"))?.checkpoints?.git?.wallMs;
 for(let n=0;n<4;n++)await tick(1500);
 const first=await taken();
 expect(first).toBeDefined();
 hold=[".git/index"];
 for(let n=0;n<12;n++)await tick(1500);
 expect(await taken()).toBe(first);
 for(let n=0;n<12;n++)await tick(1500);
 expect(await taken()).toBeGreaterThan(first!);
 hold=[];
 for(let n=0;n<8;n++)await tick(1500);
 expect(clock-(await taken())!).toBeLessThan(10_000);
});

test("a view that differs on one of three replicas is named by origin and view.digest, fleet --diff lists the lines on both sides, and equal views exit 0",async()=>{
 const lines=["?? shared.log"],extra=["?? local-only.txt"," M tracked.txt"];
 const {store,runners}=await fleet(["a1","a2","a3"],name=>({gitView:async()=>viewOf(name==="a3"?[...lines,...extra]:lines)}));
 const run=async(who:Runner,args:string[])=>{
  const notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet",...args,"--worktree",who.options.worktree,"--store",store,"--lang","en"],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  return {exitCode:result.exitCode,text:notices.join("\n")};
 };
 try{
  await until(async()=>{const report=await fleetReportV1(store);return report.rows.length===3&&report.rows.every(row=>row.settled&&row.view!==null);});
  await until(async()=>{const report=await fleetReportV1(store);return report.differing.length>0&&report.rows.filter(row=>row.verdict==="ok").length===2;});
  const report=await fleetReportV1(store);
  expect(report.ok).toBe(false);
  expect(report.differing.map(item=>[item.originId,item.field])).toEqual([["a3","view.digest"]]);
  expect(report.differing[0]!.value).toMatch(/^#\d+=[0-9a-f]{4}/u);
  expect(report.rows.find(row=>row.originId==="a3")).toMatchObject({verdict:"diverged",differs:["view.digest"],view:{untracked:3}});
  expect(validator("Report")(report)).toBe(true);
  const table=await run(runners[0]!,[]);
  expect(table.exitCode).toBe(1);
  expect(table.text).toContain("a3 differs in view.digest");
  expect(table.text).toMatch(/a1 \S+ \S+ \S+ cp:\d+s \S+ \S+ 0\/0\/1 \S+ ok/u);
  expect(table.text).toMatch(/a3 \S+ \S+ \S+ cp:\d+s \S+ \S+ 0\/0\/3 \S+ diverged/u);
  const there=await run(runners[0]!,["--diff"]);
  expect(there.exitCode).toBe(1);
  expect(there.text).toContain("a3 differs from this machine: 2 lines only there, 0 lines only here");
  expect(there.text).toContain("only on a3: ?? local-only.txt");
  expect(there.text).toContain("only on a3:  M tracked.txt");
  expect(there.text).not.toContain("a2 differs");
  const here=await run(runners[2]!,["--diff"]);
  expect(here.exitCode).toBe(1);
  expect(here.text).toContain("a1 differs from this machine: 0 lines only there, 2 lines only here");
  expect(here.text).toContain("only on a3 (here): ?? local-only.txt");
  const refused=await runReplicaCommandV1(["fleet","--diff","--json","--worktree",runners[0]!.options.worktree,"--store",store],{root:runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:()=>{}}).catch(error=>String(error));
  expect(String(refused)).toContain("--diff");
  expect(await readViewLinesV1(store,"a3",viewOf([...lines,...extra]).digest)).toEqual([...lines,...extra]);
 }finally{for(const who of runners)await stop(who);}
 const same=await fleet(["b1","b2"],()=>({gitView:async()=>viewOf(lines)}));
 try{
  await until(async()=>{const report=await fleetReportV1(same.store);return report.ok&&report.rows.every(row=>row.view!==null);});
  const notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet","--diff","--worktree",same.runners[0]!.options.worktree,"--store",same.store,"--lang","de"],{root:same.runners[0]!.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  expect(result.exitCode).toBe(0);
  expect(notices.join("\n")).toContain("Arbeitsbaum-Ansicht");
 }finally{for(const who of same.runners)await stop(who);}
});

const guardsOf=(change:Partial<FleetGuardsV1>={}):FleetGuardsV1=>({version:"2.47.3",pins:"4c7bc38fc560",volume:{caseInsensitive:false,normalisationInsensitive:false,symlinks:true,longPaths:null},infoExclude:0,caseIgnore:0,caseMismatch:0,trackedIgnored:0,trackedRefused:0,links:0,overBudget:0,modeMismatch:0,...change});

test("the guards are asked first, again when they are due and soon after the view changed, never concurrently, and the first attestation does not wait for them",async()=>{
 const store=temp("store");
 let clock=1_000_000,asked=0,found:FleetGuardsV1|null=guardsOf(),gate:Promise<void>|undefined,lines=["?? a"];
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:5000,stableMilliseconds:5000,codeRoot:code("guards"),git:async()=>git,reachable:async()=>true,view:async()=>viewOf(lines),viewMilliseconds:60_000,viewDebounceMilliseconds:5000,guards:async()=>{asked++;await gate;return found;},guardsMilliseconds:300_000,guardsMinimumMilliseconds:10_000});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 const tick=async(milliseconds:number)=>{clock+=milliseconds;return attester.tick(snapshot);};
 let release!:()=>void;
 gate=new Promise<void>(resolve=>{release=resolve;});
 expect(await tick(0)).toBe("written");
 await tick(1000);
 expect(asked).toBe(1);
 expect((await readAttestationV1(store,"a1"))!.guards).toBeNull();
 release();await Bun.sleep(10);
 expect(await tick(1000)).toBe("written");
 expect((await readAttestationV1(store,"a1"))!.guards).toEqual(guardsOf());
 expect(validator("Attestation")(await readAttestationV1(store,"a1"))).toBe(true);
 gate=undefined;
 await tick(1000);await Bun.sleep(5);
 expect(asked).toBe(1);
 lines=["?? a","?? b"];clock+=61_000;await attester.tick(snapshot);await Bun.sleep(10);clock+=6000;await attester.tick(snapshot);await Bun.sleep(10);
 expect(asked).toBe(2);
 found=guardsOf({infoExclude:1});
 clock+=300_000;await attester.tick(snapshot);await Bun.sleep(10);
 expect(asked).toBe(3);
 clock+=6000;expect(await attester.tick(snapshot)).toBe("written");
 expect((await readAttestationV1(store,"a1"))!.guards).toEqual(guardsOf({infoExclude:1}));
 found=null;clock+=300_000;await attester.tick(snapshot);await Bun.sleep(10);clock+=6000;await attester.tick(snapshot);
 expect((await readAttestationV1(store,"a1"))!.guards).toEqual(guardsOf({infoExclude:1}));
 const bare=createAttesterV1({storeRoot:temp("bare"),originId:"b1",groupId:"g1",platform:"linux",now:()=>clock,codeRoot:code("bare"),git:async()=>({present:false,head:null,commit:null,indexTree:null}),reachable:async()=>true});
 await bare.start();
 expect(await bare.tick(snapshot)).toBe("written");
});

test("a replica whose guards differ is named by origin and field, capability differences are warnings in the table and the report, and equal guards exit 0",async()=>{
 const mac={version:"2.54.0",volume:{caseInsensitive:true,normalisationInsensitive:true,symlinks:true,longPaths:null}};
 const {store,runners}=await fleet(["a1","a2","a3"],name=>({gitGuards:async()=>name==="a3"?guardsOf({...mac,infoExclude:1,overBudget:2}):guardsOf()}));
 const run=async(who:Runner,language:string="en")=>{
  const notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet","--worktree",who.options.worktree,"--store",store,"--lang",language],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  return {exitCode:result.exitCode,text:notices.join("\n")};
 };
 try{
  await until(async()=>{const report=await fleetReportV1(store);return report.rows.length===3&&report.rows.every(row=>row.settled)&&report.differing.length>0&&report.rows.filter(row=>row.verdict==="ok").length===2;});
  const report=await fleetReportV1(store);
  expect(report.ok).toBe(false);
  expect(report.differing).toEqual([{originId:"a3",field:"guards.infoExclude",value:"1",expected:"0"}]);
  expect(report.rows.find(row=>row.originId==="a3")).toMatchObject({verdict:"diverged",differs:["guards.infoExclude"]});
  expect(report.warnings).toEqual([{originId:"a3",kind:"git-version",detail:"2.54.0"},{originId:"a3",kind:"volume",detail:"case=insensitive normalisation=insensitive"},{originId:"a3",kind:"over-budget",detail:"2"}]);
  expect(validator("Report")(report)).toBe(true);
  const table=await run(runners[0]!);
  expect(table.exitCode).toBe(1);
  expect(table.text).toContain("a3 differs in guards.infoExclude: 1 instead of 0");
  expect(table.text).toContain("warning: a3 runs git 2.54.0");
  expect(table.text).toContain("warning: the volume of a3 behaves differently from most (case=insensitive normalisation=insensitive)");
  expect((await run(runners[0]!,"de")).text).toContain("Warnung: a3 verwendet git 2.54.0");
 }finally{for(const who of runners)await stop(who);}
 const same=await fleet(["b1","b2"],()=>({gitGuards:async()=>guardsOf()}));
 try{
  await until(async()=>{const report=await fleetReportV1(same.store);return report.ok&&report.rows.every(row=>row.view!==null||true);});
  const report=await fleetReportV1(same.store);
  expect(report.warnings).toEqual([]);
  expect(report.rows.every(row=>row.verdict==="ok")).toBe(true);
 }finally{for(const who of same.runners)await stop(who);}
});

test("an origin without git is exempt from the view agreement",()=>{
 const base=fixture.verdicts[0]!,attestations={...base.attestations} as unknown as Record<string,FleetAttestationV1>;
 const view={staged:0,modified:1,deleted:0,untracked:0,digest:"a".repeat(64)};
 const report=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:{...attestations.a!,view},b:{...attestations.b!,view},c:{...attestations.c!,view:null}},retirements:{}});
 expect(report.ok).toBe(true);
 expect(report.differing).toEqual([]);
});

test("a stale view is not settled, so the fleet waits instead of reporting a divergence, and it is shown to the schema",()=>{
 const base=fixture.verdicts[0]!,attestations={...base.attestations} as unknown as Record<string,FleetAttestationV1>;
 const view={staged:0,modified:1,deleted:0,untracked:0,digest:"a".repeat(64)},stale={...view,digest:"b".repeat(64),stale:true};
 const report=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:{...attestations.a!,view},b:{...attestations.b!,view},c:{...attestations.c!,view:stale}},retirements:{}});
 expect(report.ok).toBe(false);
 expect(report.rows.map(row=>[row.originId,row.verdict])).toEqual([["a","ok"],["b","ok"],["c","lagging"]]);
 expect(report.differing).toEqual([]);
 expect(validator("Attestation")({...attestations.c!,view:stale})).toBe(true);
 expect(validator("Attestation")({...attestations.c!,view:{...stale,stale:"yes"}})).toBe(false);
 const fresh=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:{...attestations.a!,view},b:{...attestations.b!,view},c:{...attestations.c!,view:{...stale,stale:false}}},retirements:{}});
 expect(fresh.differing).toHaveLength(1);
});

test("workers that cannot resolve HEAD while another origin has one are diverged even when that origin is not settled, and a git-broken failure names its reason",()=>{
 const base=fixture.verdicts[0]!,attestations={...base.attestations} as unknown as Record<string,FleetAttestationV1>,commit="1".repeat(40);
 const view={staged:0,modified:1,deleted:0,untracked:0,digest:"a".repeat(64)},tree="2".repeat(40);
 const server={...attestations.a!,head:{ref:"refs/heads/main",commit},indexTree:tree,view:{...view,stale:true}},one={...attestations.b!,head:null,indexTree:tree,view},two={...attestations.c!,head:null,indexTree:tree,view};
 const report=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:server,b:one,c:two},retirements:{}});
 expect(report.ok).toBe(false);
 expect(report.rows.map(row=>[row.originId,row.verdict])).toEqual([["a","lagging"],["b","diverged"],["c","diverged"]]);
 expect(report.rows.find(row=>row.originId==="b")!.differs).toContain("head.commit");
 expect(report.differing.filter(item=>item.field==="head.commit").map(item=>[item.originId,item.value,item.expected])).toEqual([["b",null,commit],["c",null,commit]]);
 expect(report.warnings.filter(item=>item.kind==="git-broken").map(item=>[item.originId,item.detail])).toEqual([["b","head-null"],["c","head-null"]]);
 expect(validator("Report")(report)).toBe(true);
 const named={...one,head:null,failures:[{kind:"git-broken-tip-missing",count:3,lastMs:one.wallMs-1000}]},alone=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:{...server,view},b:named,c:{...two,head:{ref:"refs/heads/main",commit}}},retirements:{}});
 expect(alone.warnings.filter(item=>item.kind==="git-broken").map(item=>[item.originId,item.detail])).toEqual([["b","tip-missing"]]);
 expect(alone.rows.find(row=>row.originId==="b")!.verdict).toBe("diverged");
 const noGit=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{a:{...server,view},b:{...one,indexTree:null},c:{...two,head:{ref:"refs/heads/main",commit},indexTree:tree}},retirements:{}});
 expect(noGit.warnings.filter(item=>item.kind==="git-broken")).toEqual([]);
});

test("recent historical observations do not warn after an exact stage is certified",()=>{
 const base=fixture.verdicts[0]!,member=base.attestations.a as unknown as FleetAttestationV1,database=new Database(":memory:");
 const cases=observationFixture.stageWarnings;
 try{
  const oracle=database.query("select case when json_extract(value,'$.treeKnown') and json_extract(value,'$.kind') like 'observe-%' then null else json_extract(value,'$.kind') end as expected from json_each(?) order by key").all(JSON.stringify(cases));
  const observed=cases.map(({treeKnown,kind})=>{
   const item={...member,indexTree:treeKnown?git.indexTree:null,failures:[{kind:"stage-"+kind,count:1,lastMs:member.wallMs-1000}]};
   const report=judgeFleetV1({now:base.now,heads:{a:base.heads.a!},attestations:{a:item},retirements:{}});
   return {expected:report.warnings.find(warning=>warning.kind==="stage")?.detail??null};
  });
  expect<unknown>(observed).toEqual(oracle);
 }finally{database.close();}
});

test("a machine whose index tree differs from the others is diverged in indexTree and head, and the reason its stage waits for is a warning that names it",()=>{
 const base=fixture.verdicts[0]!,attestations={...base.attestations} as unknown as Record<string,FleetAttestationV1>,commit="1".repeat(40),tree="2".repeat(40),other="3".repeat(40);
 const view={staged:0,modified:0,deleted:0,untracked:0,digest:"a".repeat(64)},head={ref:"refs/heads/main",commit};
 const members={a:{...attestations.a!,head,indexTree:tree,view},b:{...attestations.b!,head,indexTree:tree,view},c:{...attestations.c!,head,indexTree:other,view,failures:[{kind:"stage-objects-missing",count:1,lastMs:attestations.c!.wallMs-1000}]}};
 const report=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:members,retirements:{}});
 expect(report.ok).toBe(false);
 expect(report.rows.find(row=>row.originId==="c")).toMatchObject({verdict:"diverged",differs:["indexTree"]});
 expect(report.differing.filter(item=>item.field==="indexTree").map(item=>[item.originId,item.value,item.expected])).toEqual([["c",other,tree]]);
 expect(report.warnings.filter(item=>item.kind==="stage").map(item=>[item.originId,item.detail])).toEqual([["c","objects-missing"]]);
 expect(validator("Report")(report)).toBe(true);
 const behind=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{...members,c:{...members.c,indexTree:tree,head:{ref:"refs/heads/main",commit:"4".repeat(40)},failures:undefined}},retirements:{}});
 expect(behind.rows.find(row=>row.originId==="c")).toMatchObject({verdict:"diverged",differs:["head.commit"]});
 const agreed=judgeFleetV1({now:base.now,heads:headsOf(base.heads),attestations:{...members,c:{...members.c,indexTree:tree,failures:undefined}},retirements:{}});
 expect(agreed.rows.map(row=>row.verdict)).toEqual(["ok","ok","ok"]);
 expect(agreed.warnings.filter(item=>item.kind==="stage")).toEqual([]);
});

test("the relay copies view.txt with attest.json and a compaction keeps it",async()=>{
 const one=temp("s1"),two=temp("s2"),lines=["?? relayed.txt"," D gone.txt"],who=runner("a1",one,{gitView:async()=>viewOf(lines)});
 writeFileSync(join(who.options.worktree,"a.txt"),"alpha\n");
 await seedV1(who.options);
 start(who);
 try{
  await until(async()=>(await readAttestationV1(one,"a1"))?.view!=null&&existsSync(viewPathV1(one,"a1")));
  await relayOnceV1(two,one,await homedOriginsV1(one,"g1"));
  expect(readFileSync(viewPathV1(two,"a1"),"utf8")).toBe(readFileSync(viewPathV1(one,"a1"),"utf8"));
  expect(await readViewLinesV1(two,"a1",(await readAttestationV1(two,"a1"))!.view!.digest)).toEqual(lines);
 }finally{await stop(who);}
 await compactV1(one,"a1",0);
 expect(existsSync(viewPathV1(one,"a1"))).toBe(true);
 await compactV1(two,"a1",0);
 expect(existsSync(viewPathV1(two,"a1"))).toBe(true);
});

test("a store file replaced under the reader is read again, a missing or always replaced one answers undefined and every other failure is thrown",async()=>{
 const {readThroughReplacementV1,REPLACED_READ_V1}=await import("../../📒️journal/🟦️.ts"),failure=(code:string)=>Object.assign(Error(code),{code}),waits:number[]=[],wait=async(milliseconds:number)=>{waits.push(milliseconds);};
 for(const code of ["EBADF","ESTALE"]){
  let calls=0;waits.length=0;
  expect(await readThroughReplacementV1(async()=>{if(++calls<3)throw failure(code);return "whole";},wait),code).toBe("whole");
  expect(calls).toBe(3);expect(waits).toEqual([REPLACED_READ_V1.delayMilliseconds,REPLACED_READ_V1.delayMilliseconds]);
 }
 let calls=0;waits.length=0;
 expect(await readThroughReplacementV1(async()=>{calls++;throw failure("EBADF");},wait)).toBeUndefined();
 expect(calls).toBe(REPLACED_READ_V1.attempts);expect(waits).toHaveLength(REPLACED_READ_V1.attempts-1);
 expect(await readThroughReplacementV1(async()=>{throw failure("ENOENT");},wait)).toBeUndefined();
 await expect(readThroughReplacementV1(async()=>{throw failure("EIO");},wait)).rejects.toThrow("EIO");
});

test("a rejected Git observation withdraws its checkpoint and counts only each failure episode",async()=>{
 const store=temp("git-observation"),rows=observationFixture.observations;let clock=1_000_000,observation="exact";
 const counts=new Map<string,{kind:string;count:number;lastMs:number}>();
 const attester=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,codeRoot:code("observe"),reachable:async()=>true,
  git:async()=>{if(observation==="rejected")throw Error("private-index-reason");return observation==="exact"?git:{...git,indexTree:null,observation:observation==="foreign"?"private-index-reason" as never:"index-changed" as const};},
  failures:()=>[...counts.values()],
  gitObservationFailed:(reason:string)=>{const kind="stage-observe-"+reason;counts.set(kind,{kind,count:(counts.get(kind)?.count??0)+1,lastMs:clock});}
 });
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),buckets:"0".repeat(1024),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 const db=new Database(":memory:");
 try{
  const oracle=db.query(`with rows as (select cast(key as integer) as n,case when value='foreign' then 'rejected' else value end as value from json_each(?)), phases as (select n,value,lag(value) over (order by n) as prior from rows)
  select sum(case when value='rejected' and value is not prior then 1 else 0 end) as rejected,
  sum(case when value='moved' and value is not prior then 1 else 0 end) as moved from phases where n<=?`);
  for(const [index,row] of rows.entries()){
   observation=row;
   for(let turn=0;turn<3;turn++){clock+=1000;await attester.tick(snapshot);}
   const receipt=(await readAttestationV1(store,"a1"))!;
   expect(receipt.indexTree,row).toBe(row==="exact"?git.indexTree:null);
   expect(receipt.checkpoints?.git?.indexTree,row).toBe(row==="exact"?git.indexTree:undefined);
   expect<unknown>({rejected:counts.get("stage-observe-observation-failed")?.count??0,moved:counts.get("stage-observe-index-changed")?.count??0},row).toEqual(oracle.get(JSON.stringify(rows),index));
   expect(JSON.stringify(receipt)).not.toContain("private-index-reason");
   expect(validator("Attestation")(receipt)).toBe(true);
  }
 }finally{db.close();}
});

test("an unavailable Git sample stays at heartbeat cadence and does not start a view",async()=>{
 const store=temp("stage-cadence");
 let clock=1_000_000,reads=0,views=0;
 const replica=createAttesterV1({storeRoot:store,originId:"a1",groupId:"g1",now:()=>clock,heartbeatMilliseconds:5000,codeRoot:code("observe-cadence"),reachable:async()=>true,
  git:async()=>{reads++;return {...git,indexTree:null,observation:"index-changed"};},
  view:async()=>{views++;return null;}
 });
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),buckets:"0".repeat(1024),cursors:{},incoming:0,hlc:"0000001000000-000000-a1"};
 await replica.start();
 for(let step=0;step<4;step++){clock+=1000;await replica.tick(snapshot);}
 expect(reads).toBe(1);expect(views).toBe(0);
 expect((await readAttestationV1(store,"a1"))?.indexTree).toBeNull();
 clock+=1000;await replica.tick(snapshot);await Bun.sleep(1);
 expect(reads).toBe(2);expect(views).toBe(0);
});

test("the replica mirrors one closed observation failure per episode in its existing diagnostics",async()=>{
 let exact=false;
 const {store,runners}=await fleet(["a1"],()=>({
  gitState:async()=>exact?git:{...git,indexTree:null,observation:"locked-snapshot-failed"},
  gitView:async()=>null
 }));
 const read=()=>readAttestationV1(store,"a1");
 const kind="stage-observe-locked-snapshot-failed";
 const count=async()=>(await read())?.failures?.find(row=>row.kind===kind)?.count;
 try{
  await until(async()=>await count()===1);
  expect((await read())?.indexTree).toBeNull();
  await Bun.sleep(250);expect(await count()).toBe(1);
  exact=true;
  await until(async()=>(await read())?.indexTree===git.indexTree);
  exact=false;
  await until(async()=>await count()===2);
  expect((await read())?.checkpoints?.git).toBeUndefined();
 }finally{await Promise.all(runners.map(stop));}
});
