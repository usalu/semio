import {afterAll,expect,setDefaultTimeout,spyOn,test} from "bun:test";
import * as fsPromises from "node:fs/promises";
import Ajv from "ajv";
import {appendFileSync,chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync,existsSync,readdirSync,openSync,writeSync,closeSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import attestSchema from "../../🩺️attest/🧬️schema/🔣️.json" with {type:"json"};
import replicaSchema from "../../🔁️replica/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {codeDigestV1} from "../../🩺️attest/🟦️.ts";
import {codeDirectoryV1,createFailuresV1,noteRefusedPathV1,NO_REFUSED_PATHS_V1,readRefusedPathsV1,refusedPathsPathV1,REFUSED_PATH_LIMIT_V1,writeRefusedPathsV1,createStoreGuardV1,healthPathV1,importClosureV1,listCodeSnapshotsV1,moduleSourcesV1,pruneCodeSnapshotsV1,pruneLostV1,readHealthV1,requireResilienceShapeV1,restartJitterMillisecondsV1,rotateLogV1,runSelfCheckV1,sealCodeV1,selfCheckV1,StoreStalledV1,textsProblemV1,verifyCodeSnapshotV1,writeHealthV1,type StoreStateV1,type TargetSharingCodeV1} from "../🟦️.ts";
import {stretchedBudgetMillisecondsV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

import {activityNamesV1,activityV1,blockedFailuresV1,installBootStageV1,listStoreCodeV1,publishCodeSnapshotV1,pruneStoreCodeV1,readBeatV1,readFallbackV1,storeCodeDirectoryV1,verifyCodeSnapshotV1,yieldingV1} from "../🟦️.ts";
import {createHash} from "node:crypto";
import {utimesSync} from "node:fs";
import {ownProcessIdentityV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

setDefaultTimeout(60_000);
const ajv=new Ajv({strict:true,allowUnionTypes:true}),scratch:string[]=[],python=testPythonV1();
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`resilience-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const until=async(condition:()=>boolean,milliseconds=5000):Promise<void>=>{const end=Date.now()+milliseconds;while(!condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(10);}};
const digestOf=(moduleRoot:string):Promise<string>=>codeDigestV1(moduleRoot);

test("the fixture satisfies the closed schema and the Python oracle agrees with every vector",()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(validate(fixture),JSON.stringify(validate.errors?.[0])).toBe(true);
 if(python===undefined)return;
 const result=Bun.spawnSync([python,"-I",join(import.meta.dirname,"🐍️.py")],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stdout.toString()+result.stderr.toString()).toBe(0);
});

test("the restart jitter matches the fixture, stays inside its span, is deterministic and spreads a fleet",()=>{
 for(const item of fixture.jitter)expect(restartJitterMillisecondsV1(item.originId,item.digest,item.span)).toBe(item.expected);
 const digest="ab".repeat(32),values=Array.from({length:60},(_,index)=>restartJitterMillisecondsV1(`g1-m${index}`,digest));
 for(const value of values){expect(value).toBeGreaterThanOrEqual(0);expect(value).toBeLessThan(60_000);}
 expect(new Set(values).size).toBeGreaterThan(50);
 expect(Math.max(...values)-Math.min(...values)).toBeGreaterThan(30_000);
 expect(restartJitterMillisecondsV1("g1-m01",digest)).toBe(restartJitterMillisecondsV1("g1-m01",digest));
 expect(restartJitterMillisecondsV1("g1-m01","cd".repeat(32))).not.toBe(restartJitterMillisecondsV1("g1-m01",digest));
 expect(restartJitterMillisecondsV1("g1-m01",digest,0)).toBe(0);
});

test("the import closure of the fixture trees follows relative imports once and ignores everything else",async()=>{
 for(const item of fixture.closure){
  const root=temp("closure");
  for(const file of item.files)put(root,file.path,file.text);
  const module=join(root,"mod");
  expect(await importClosureV1(module,item.entries),item.name).toEqual(item.expected);
 }
});

test("shared relative import targets need only one physical admission probe",async()=>{
 const item=fixture.closure.find(row=>row.name==="outside imports, json and transitive files are followed once")!;
 const root=temp("closure-probe"),module=join(root,"mod"),target=join(root,"shared","x.ts");
 for(const file of item.files)put(root,file.path,file.text);
 let reads=0;
 const stat=fsPromises.stat,watch=spyOn(fsPromises,"stat").mockImplementation((path,options)=>{
  if(path===target)reads++;
  return stat(path,options);
 });
 try{
  expect(await importClosureV1(module,item.entries)).toEqual(item.expected);
  expect(reads).toBe(1);
 }finally{watch.mockRestore();}
});

test("module sources are the code files without tests and the text verdicts match the fixture",async()=>{
 const root=temp("sources");
 put(root,"a/🟦️.ts","export {};");put(root,"a/🧪️tests/🟦️.ts","x");put(root,"b/📜️script.ts","y");put(root,"b/other.ts","z");put(root,"node_modules/p/🟦️.ts","q");
 expect(await moduleSourcesV1(root)).toEqual(["a/🟦️.ts","b/📜️script.ts"]);
 for(const item of fixture.texts)expect(textsProblemV1(item.catalogue),item.name).toBe(item.error);
});

const goodModule=(root:string):void=>{
 put(root,"a/🟦️.ts","import {b} from \"../b/🟦️.ts\";\nexport const a=b+1;\n");
 put(root,"b/🟦️.ts","import data from \"./data.json\" with {type:\"json\"};\nexport const b=data.n;\n");
 put(root,"b/data.json","{\"n\":1}");
 put(root,"c/🧬️schema/🔣️.json",JSON.stringify({definitions:{One:{type:"string"},Two:{type:"object",properties:{one:{$ref:"#/definitions/One"}}}}}));
 put(root,"c/📖️texts/🔣️.json",JSON.stringify({version:1,languages:["en","de"],messages:{x:{en:"Hello {n}",de:"Hallo {n}"}}}));
};

test("the self-check loads every module and validates schemas and texts, and names the first fault",async()=>{
 const good=temp("good");goodModule(good);
 const report=await selfCheckV1(good);
 expect(report).toEqual({version:1,ok:true,digest:null,modules:2,schemas:1,texts:1,error:null});
 requireResilienceShapeV1("SelfCheck",report,"report");

 const syntax=temp("syntax");goodModule(syntax);put(syntax,"b/🟦️.ts","export const b=;\n");
 const broken=await selfCheckV1(syntax);
 expect(broken.ok).toBe(false);expect(broken.error).toContain("does not load");expect(broken.error).toContain("Unexpected");

 const missing=temp("missing");goodModule(missing);put(missing,"a/🟦️.ts","import {b} from \"../b/gone.ts\";\nexport const a=b;\n");
 expect((await selfCheckV1(missing)).error).toContain("module a/🟦️.ts does not load");

 const named=temp("named");goodModule(named);put(named,"a/🟦️.ts","import {nothing} from \"../b/🟦️.ts\";\nexport const a=nothing;\n");
 expect((await selfCheckV1(named)).ok).toBe(false);

 const texts=temp("texts");goodModule(texts);put(texts,"c/📖️texts/🔣️.json",JSON.stringify({version:1,languages:["en","de"],messages:{x:{en:"Hello"}}}));
 expect((await selfCheckV1(texts)).error).toContain("must have exactly the languages");

 const json=temp("json");goodModule(json);put(json,"c/🧬️schema/🔣️.json","{broken");
 expect((await selfCheckV1(json)).error).toContain("is not valid JSON");

 const definition=temp("definition");goodModule(definition);put(definition,"c/🧬️schema/🔣️.json",JSON.stringify({definitions:{Bad:{$ref:"#/definitions/Nowhere"}}}));
 expect((await selfCheckV1(definition)).ok).toBe(false);
});

test("a self-check child process is bounded by a deadline, cancellable and judged by its exit code and report",async()=>{
 const ok=JSON.stringify({version:1,ok:true,digest:"a".repeat(64),modules:1,schemas:0,texts:0,error:null});
 const run=(code:string,timeoutMs=20000,signal?:AbortSignal)=>runSelfCheckV1({command:process.execPath,args:["-e",code],cwd:tmpdir(),timeoutMs,signal});
 expect(await run(`console.log(${JSON.stringify(ok)})`)).toEqual({ok:true,reason:"",digest:"a".repeat(64)});
 const failed=await run(`console.log(${JSON.stringify(JSON.stringify({version:1,ok:false,digest:null,modules:0,schemas:0,texts:0,error:"module x does not load"}))});process.exit(1)`);
 expect(failed.ok).toBe(false);expect(failed.reason).toContain("module x does not load");
 const silent=await run("console.error('boom');process.exit(3)");
 expect(silent.ok).toBe(false);expect(silent.reason).toContain("boom");
 expect((await run(`console.log(${JSON.stringify(ok)});process.exit(2)`)).ok).toBe(false);
 const started=Date.now(),hung=await run("setInterval(()=>{},1000)",300);
 expect(hung.ok).toBe(false);expect(hung.reason).toContain("timed out");expect(Date.now()-started).toBeLessThan(5000);
 const controller=new AbortController(),waiting=run("setInterval(()=>{},1000)",20000,controller.signal);
 setTimeout(()=>{controller.abort();},100);
 expect((await waiting).reason).toBe("self-check cancelled");
 expect((await runSelfCheckV1({command:"/nonexistent/runtime",args:[],cwd:tmpdir(),timeoutMs:5000})).ok).toBe(false);
});

test("a seal mirrors the checked closure, verifies its digest, keeps the newest two and refuses a change during the copy",async()=>{
 const root=temp("seal"),module=join(root,"mod");
 put(root,"shared/x.ts","export const x=1;\n");
 const write=(version:number):void=>{put(root,"mod/a/🟦️.ts",`import {x} from "../../shared/x.ts";\nimport data from "./data.json" with {type:"json"};\nexport const a=x+data.n+${version};\n`);put(root,"mod/a/data.json","{\"n\":1}");};
 const digests:string[]=[];
 let clock=1000;
 for(const version of [1,2,3]){
  write(version);
  const digest=await digestOf(module);digests.push(digest);
  const sealed=await sealCodeV1({root,moduleRoot:module,digest,digestOf,now:()=>++clock});
  expect(sealed).toEqual({created:true,directory:join(codeDirectoryV1(root),digest),reason:null});
  expect(await verifyCodeSnapshotV1(sealed.directory!)).toBeNull();
  expect(existsSync(join(sealed.directory!,"files","mod","a","🟦️.ts"))).toBe(true);
  expect(existsSync(join(sealed.directory!,"files","shared","x.ts"))).toBe(true);
  expect(existsSync(join(sealed.directory!,"files","mod","a","data.json"))).toBe(true);
  requireResilienceShapeV1("SealedCode",JSON.parse(readFileSync(join(sealed.directory!,"sealed.json"),"utf8")),"seal");
  expect(await digestOf(join(sealed.directory!,"files","mod"))).toBe(digest);
 }
 const listed=await listCodeSnapshotsV1(root);
 expect(listed.map(item=>item.digest)).toEqual([digests[2]!,digests[1]!]);
 expect(readdirSync(codeDirectoryV1(root)).filter(name=>name.startsWith(".tmp-"))).toEqual([]);
 const again=await sealCodeV1({root,moduleRoot:module,digest:digests[2]!,digestOf,now:()=>++clock});
 expect(again.created).toBe(false);
 write(9);
 const stale=await sealCodeV1({root,moduleRoot:module,digest:digests[2]!,digestOf:async()=>"f".repeat(64),now:()=>++clock});
 expect(stale.created).toBe(true===false);
 const changed=await sealCodeV1({root,moduleRoot:module,digest:"e".repeat(64),digestOf,now:()=>++clock});
 expect(changed).toEqual({created:false,directory:null,reason:"code-changed-while-copying"});
 expect(existsSync(join(codeDirectoryV1(root),"e".repeat(64)))).toBe(false);
 expect(readdirSync(codeDirectoryV1(root)).filter(name=>name.startsWith(".tmp-"))).toEqual([]);
 const tamper=listed[0]!.directory;
 writeFileSync(join(tamper,"files","shared","x.ts"),"export const x=2;\n");
 expect(await verifyCodeSnapshotV1(tamper)).toContain("changed shared/x.ts");
 expect(await sealCodeV1({root,moduleRoot:root,digest:"d".repeat(64),digestOf})).toEqual({created:false,directory:null,reason:"module-outside-root"});
});

test("snapshot pruning follows the fixture order and removes stale unfinished directories",async()=>{
 for(const item of fixture.prune){
  const root=temp("prune"),base=codeDirectoryV1(root);
  for(const [name,created] of Object.entries(item.created)){
   const digest=name.padEnd(64,"0"),directory=join(base,digest);
   put(directory,"sealed.json",JSON.stringify({version:1,digest,createdMs:created,moduleRoot:"mod",files:[]}));
  }
  const removed=await pruneCodeSnapshotsV1(root,item.keep);
  expect((await listCodeSnapshotsV1(root)).map(snapshot=>snapshot.digest.slice(0,1)),item.name).toEqual(item.kept);
  expect(removed.length+item.kept.length).toBe(Object.keys(item.created).length);
 }
 const root=temp("unfinished"),base=codeDirectoryV1(root);
 put(base,".tmp-old/files/x","1");put(base,".tmp-new/files/x","1");
 const old=new Date(Date.now()-2*3_600_000);
 (await import("node:fs")).utimesSync(join(base,".tmp-old"),old,old);
 await pruneCodeSnapshotsV1(root,2);
 expect(existsSync(join(base,".tmp-old"))).toBe(false);expect(existsSync(join(base,".tmp-new"))).toBe(true);
});

const guarded=(options:{deadlineMs?:number}={})=>{
 const states:StoreStateV1[]=[],probes:{release:(ok:boolean)=>void}[]=[];
 let mode:"fast"|"hang"="fast";
 const guard=createStoreGuardV1({deadlineMs:options.deadlineMs??120,now:Date.now,changed:state=>states.push(state),probe:()=>{
  if(mode==="fast")return Promise.resolve(true);
  return new Promise<boolean>(done=>{probes.push({release:done});});
 }});
 return {guard,states,probes,hang:()=>{mode="hang";},heal:()=>{mode="fast";}};
};

test("a healthy store answers probes and runs turns without ever reporting a stall",async()=>{
 const {guard,states}=guarded();
 expect(await guard.reachable()).toBe(true);
 const slow=await guard.turn(async()=>{await Bun.sleep(400);return 7;});
 expect(slow).toEqual({status:"done",value:7});
 await expect(guard.turn(async()=>{throw Error("boom");})).rejects.toThrow("boom");
 expect(await guard.call(async()=>"x")).toBe("x");
 expect(states).toEqual([]);expect(guard.state()).toBe("ok");
});

test("a hanging probe makes the store stalled within the deadline, stops new calls and recovers without a restart",async()=>{
 const {guard,states,probes,hang,heal}=guarded();
 hang();
 const started=Date.now();
 expect(await guard.reachable()).toBe(false);
 expect(Date.now()-started).toBeLessThan(2000);expect(Date.now()-started).toBeGreaterThanOrEqual(100);
 expect(guard.state()).toBe("stalled");expect(states).toEqual(["stalled"]);
 const again=Date.now();
 expect(await guard.reachable()).toBe(false);expect(Date.now()-again).toBeLessThan(50);
 expect(probes.length).toBe(1);
 let called=0;
 expect(await guard.turn(async()=>{called++;return 1;})).toEqual({status:"stalled"});
 await expect(guard.call(async()=>{called++;})).rejects.toBeInstanceOf(StoreStalledV1);
 expect(called).toBe(0);expect(guard.outstanding()).toBe(1);
 heal();
 probes[0]!.release(true);
 await until(()=>guard.state()==="ok");
 expect(states).toEqual(["stalled","ok"]);
 expect(await guard.reachable()).toBe(true);
 expect(await guard.turn(async()=>3)).toEqual({status:"done",value:3});
});

test("a turn blocked in an uncancellable call is abandoned and aborted, issues nothing new and the loop stays free",async()=>{
 const {guard,states,probes,hang,heal}=guarded();
 let aborted=false,unblock:()=>void=()=>{};
 const blocked=new Promise<void>(done=>{unblock=done;});
 const started=Date.now();
 const turn=guard.turn(async signal=>{signal.addEventListener("abort",()=>{aborted=true;},{once:true});hang();await blocked;return "late";});
 await Bun.sleep(30);
 const outcome=await turn;
 expect(outcome).toEqual({status:"stalled"});
 expect(Date.now()-started).toBeLessThan(3000);
 expect(aborted).toBe(true);expect(states).toEqual(["stalled"]);
 expect(await guard.turn(async()=>1)).toEqual({status:"stalled"});
 let ticks=0;
 const timer=setInterval(()=>{ticks++;},10);
 await Bun.sleep(100);clearInterval(timer);
 expect(ticks).toBeGreaterThan(Math.floor(5/(stretchedBudgetMillisecondsV1(1000)/1000)));
 heal();unblock();
 for(const probe of probes)probe.release(true);
 await until(()=>guard.state()==="ok");
 expect(await guard.turn(async()=>2)).toEqual({status:"done",value:2});
 expect(states).toEqual(["stalled","ok"]);
});

test("a single store call that exceeds its deadline is abandoned and later calls are refused until it returns",async()=>{
 const {guard,states}=guarded({deadlineMs:80});
 let unblock:()=>void=()=>{};
 const blocked=new Promise<void>(done=>{unblock=done;});
 await expect(guard.call(()=>blocked)).rejects.toBeInstanceOf(StoreStalledV1);
 expect(guard.state()).toBe("stalled");
 await expect(guard.call(async()=>1)).rejects.toBeInstanceOf(StoreStalledV1);
 unblock();
 await until(()=>guard.state()==="ok");
 expect(states).toEqual(["stalled","ok"]);
 expect(await guard.call(async()=>5)).toBe(5);
});

test("an unreachable store (probe false) is reported as unreachable and recovers when the probe answers",async()=>{
 const states:StoreStateV1[]=[];
 let up=false;
 const guard=createStoreGuardV1({deadlineMs:100,now:Date.now,changed:state=>states.push(state),probe:async()=>up});
 expect(await guard.reachable()).toBe(false);
 expect(guard.state()).toBe("unreachable");
 up=true;
 expect(await guard.reachable()).toBe(true);
 expect(states).toEqual(["unreachable","ok"]);
});

test("replica diagnostics keep bounded classified block reasons alongside unchanged totals",()=>{
 const attestFailures=ajv.compile({definitions:attestSchema.definitions,$ref:"#/definitions/Attestation/properties/failures"});
 const statusFailures=ajv.compile({definitions:replicaSchema.definitions,$ref:"#/definitions/Status/properties/health/oneOf/1/properties/failures"});
 for(const row of fixture.blocked){let now=0;const tracker=createFailuresV1({now:()=>now,capacity:2});
  for(const [key,reason,at,nativeCode,operation] of row.events){now=Number(at);tracker.note(String(key),String(reason),false,(nativeCode as TargetSharingCodeV1|null)??undefined,(operation as "unlink"|"rename"|null)??undefined);}
  expect(tracker.summary(row.limit),row.name).toEqual(row.totals);
  const diagnostics=tracker.diagnostics(row.limit);
  expect(diagnostics,row.name).toEqual(row.results);
  expect(attestFailures(diagnostics),row.name).toBe(true);
  expect(statusFailures(diagnostics),row.name).toBe(true);
  for(const item of diagnostics)requireResilienceShapeV1("Failure",item,row.name);
 }
});

test("recurring failures stay visible: announced first, again once per window, counted per kind and forgotten on success",()=>{
 let now=1000;
 const failures=createFailuresV1({now:()=>now,windowMs:60_000,capacity:3});
 expect(failures.note("blocked:a:EBUSY","m")).toEqual({announce:true,count:1});
 now+=1000;expect(failures.note("blocked:a:EBUSY","m")).toEqual({announce:false,count:2});
 now+=60_000;expect(failures.note("blocked:a:EBUSY","m")).toEqual({announce:true,count:3});
 failures.note("error:git:x","m");now+=5;failures.note("error:git:x","m");
 expect(failures.summary()).toEqual([{kind:"error",count:2,lastMs:now},{kind:"blocked",count:3,lastMs:now-5}]);
 failures.clear("blocked:a:EBUSY");
 expect(failures.has("blocked:a:EBUSY")).toBe(false);
 expect(failures.note("blocked:a:EBUSY","m")).toEqual({announce:true,count:1});
 expect(failures.summary().find(item=>item.kind==="blocked")!.count).toBe(4);
 for(const key of ["k1:x","k2:x","k3:x","k4:x"])failures.note(key,"m");
 expect(failures.size()).toBe(3);
 now+=10_000;
 expect(failures.forgetOlderThan(5000)).toBe(3);expect(failures.size()).toBe(0);
 for(const item of failures.summary())requireResilienceShapeV1("Failure",item,"failure");
});

test("old preserved overwritten files and oversized logs are bounded, and a rotation survives an appending writer",async()=>{
 const root=temp("lost"),now=Date.now();
 const name=(ageMs:number,suffix:string):string=>`${String(now-ageMs).padStart(13,"0")}-000000-${suffix}`;
 for(const [age,suffix] of [[20*86_400_000,"old"],[3*86_400_000,"mid"],[1000,"new"],[500,"newer"]] as [number,string][])put(root,`${name(age,suffix)}/a.txt`,"x");
 expect(await pruneLostV1(root,{now,retainMs:14*86_400_000,maxEntries:10})).toBe(1);
 expect(await pruneLostV1(root,{now,retainMs:14*86_400_000,maxEntries:2})).toBe(1);
 expect(readdirSync(root).map(item=>item.split("-")[2]).sort()).toEqual(["new","newer"]);
 for(const item of fixture.rotate){
  const dir=temp("rotate"),file=join(dir,"service.log");
  writeFileSync(file,item.text);
  expect(await rotateLogV1(file,{maxBytes:item.maxBytes,keepBytes:item.keepBytes}),item.name).toBe(item.rotated);
  if(item.rotated){expect(readFileSync(`${file}.1`,"utf8")).toBe(item.tail);expect(statSync(file).size).toBe(0);}
  else expect(existsSync(`${file}.1`)).toBe(false);
 }
 const dir=temp("append"),file=join(dir,"service.log"),descriptor=openSync(file,"a");
 writeSync(descriptor,"x".repeat(100));
 expect(await rotateLogV1(file,{maxBytes:50,keepBytes:10})).toBe(true);
 writeSync(descriptor,"after\n");closeSync(descriptor);
 expect(readFileSync(file,"utf8")).toBe("after\n");expect(readFileSync(`${file}.1`,"utf8")).toBe("x".repeat(10));
 appendFileSync(file,"");
});

test("the health file round-trips and a torn or invalid copy reads as absent",async()=>{
 const root=temp("health");
 expect(await readHealthV1(root)).toBeUndefined();
 const health={version:1 as const,wallMs:5,pid:7,code:"a".repeat(64),store:"stalled" as const,storeSinceMs:3,pending:{code:"b".repeat(64),ok:false as const,reason:"module x does not load"},failures:[{kind:"blocked",count:2,lastMs:4}],trust:{scope:"shared-source" as const,verifiedMs:3,lostMs:null,lostKind:null}};
 await writeHealthV1(root,health);
 expect(await readHealthV1(root)).toEqual(health);
 writeFileSync(healthPathV1(root),"{\"version\":1,");
 expect(await readHealthV1(root)).toBeUndefined();
 await expect(writeHealthV1(root,{...health,store:"x" as never})).rejects.toThrow("Closed health required");
});

test("a sealed snapshot is published to the store with the per-file hashes of the language-agnostic fixture, replaced when damaged, listed, pruned and never half written",async()=>{
 for(const item of fixture.store){
  const root=temp("store-code"),store=temp("store");
  for(const file of item.files){put(root,file.path,file.text);if(file.executable)chmodSync(join(root,...file.path.split("/")),0o755);}
  const moduleRoot=join(root,"m"),digest=await digestOf(moduleRoot),sealed=await sealCodeV1({root,moduleRoot,digest,digestOf});
  expect(sealed.created,item.name).toBe(true);
  expect((JSON.parse(readFileSync(join(codeDirectoryV1(root),digest,"sealed.json"),"utf8")) as {files:unknown}).files,item.name).toEqual(item.expected);
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:true,reason:null});
  const target=join(storeCodeDirectoryV1(store),digest);
  expect((JSON.parse(readFileSync(join(target,"sealed.json"),"utf8")) as {files:unknown}).files,item.name).toEqual(item.expected);
  for(const file of item.files){const stored=join(target,"files",...file.path.split("/"));expect(readFileSync(stored,"utf8")).toBe(file.text);if(process.platform!=="win32")expect((statSync(stored).mode&0o111)!==0,file.path).toBe(file.executable);}
  expect(await verifyCodeSnapshotV1(target)).toBeNull();
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:false,reason:"present"});
  expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digest]);
  put(store,`v1/bootstrap/code/${digest}/files/${item.expected[0]!.path}`,"damaged");
  expect(await verifyCodeSnapshotV1(target)).toContain("changed");
  expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-m01"})).toEqual({published:true,reason:null});
  expect(await verifyCodeSnapshotV1(target)).toBeNull();
  expect(readdirSync(storeCodeDirectoryV1(store)).filter(name=>name.endsWith("-old"))).toEqual([]);
  put(root,"m/a/🟦️.ts","export const changed=1;\n");
  expect((await publishCodeSnapshotV1({root,store,digest:"0".repeat(64),originId:"g1-m01"})).reason).toContain("local snapshot");
  mkdirSync(join(storeCodeDirectoryV1(store),".tmp-g1-m09-dead"),{recursive:true});
  expect(await listStoreCodeV1(store)).toHaveLength(1);
 }
});

test("old store snapshots are pruned down to the newest ones, but never one that a live origin attests, and old unfinished directories go",async()=>{
 const store=temp("store-prune"),base=storeCodeDirectoryV1(store),digests=["1","2","3","4"].map(n=>n.repeat(64));
 digests.forEach((digest,index)=>{
  put(store,`v1/bootstrap/code/${digest}/sealed.json`,JSON.stringify({version:1,digest,createdMs:1000+index,moduleRoot:"m",files:[]}));
 });
 mkdirSync(join(base,".tmp-old"),{recursive:true});utimesSync(join(base,".tmp-old"),1000,1000);mkdirSync(join(base,".tmp-new"),{recursive:true});
 expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digests[3]!,digests[2]!,digests[1]!,digests[0]!]);
 const removed=await pruneStoreCodeV1(store,2,new Set([digests[0]!]));
 expect(removed).toEqual([digests[1]!]);
 expect((await listStoreCodeV1(store)).map(entry=>entry.digest)).toEqual([digests[3]!,digests[2]!,digests[0]!]);
 expect(existsSync(join(base,".tmp-old"))).toBe(false);expect(existsSync(join(base,".tmp-new"))).toBe(true);
 writeFileSync(join(base,"._"+digests[0]!),"x");utimesSync(join(base,"._"+digests[0]!),1000,1000);writeFileSync(join(base,"._fresh"),"x");
 await pruneStoreCodeV1(store,2,new Set([digests[0]!]));
 expect(existsSync(join(base,"._"+digests[0]!))).toBe(false);expect(existsSync(join(base,"._fresh"))).toBe(true);
});

test("the boot stage copy is installed through a signature that is written last, and only changes when the stage changes",async()=>{
 const root=temp("stage"),script=join(temp("script"),"script.ts");
 writeFileSync(script,"export const a=1;\n");
 expect(await installBootStageV1(root,script)).toEqual({installed:true});
 expect(await installBootStageV1(root,script)).toEqual({installed:false});
 const signature=JSON.parse(readFileSync(join(root,".cache","f","boot","stage.json"),"utf8")) as {sha256:string;bytes:number};
 requireResilienceShapeV1("BootStage",signature,"Closed boot stage required");
 expect(signature.sha256).toBe(createHash("sha256").update("export const a=1;\n").digest("hex"));
 writeFileSync(script,"export const a=2;\n");
 expect(await installBootStageV1(root,script)).toEqual({installed:true});
 expect(readFileSync(join(root,".cache","f","boot","stage.ts"),"utf8")).toBe("export const a=2;\n");
});

test("the beat of a live daemon is read while the file is fresh and the process exists, and the fallback guard is read only when it is closed",async()=>{
 const root=temp("beat"),now=Date.now(),beat={version:1,...ownProcessIdentityV1(),parent:1,startedMs:now-5000,wallMs:now,phase:"run",base:"/x",source:"snapshot",lagMs:42_000};
 put(root,".cache/f/boot/beat.json",JSON.stringify(beat));
 requireResilienceShapeV1("Beat",beat,"Closed beat required");
 expect(await readBeatV1(root,now)).toEqual(beat);
 expect(await readBeatV1(root,now+30_000)).toBeUndefined();
 put(root,".cache/f/boot/beat.json",JSON.stringify({...beat,pid:2_000_000_000}));
 expect(await readBeatV1(root,now)).toBeUndefined();
 put(root,".cache/f/boot/beat.json",JSON.stringify({...beat,phase:"later"}));
 expect(await readBeatV1(root,now)).toBeUndefined();
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:JSON.stringify({guard:"code-fallback",source:"store",reason:"x"})})).toEqual({guard:"code-fallback",source:"store",reason:"x"});
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:JSON.stringify({guard:"other",source:"store",reason:"x"})})).toBeUndefined();
 expect(readFallbackV1({SEMIO_FLEET_CODE_FALLBACK:"{"})).toBeUndefined();
 expect(readFallbackV1({})).toBeUndefined();
});

test("a cooperative yield hands the event loop back within its budget while a long loop runs, and costs almost nothing when the work is short",async()=>{
 let worst=0,last=performance.now(),ticks=0;
 const timer=setInterval(()=>{const now=performance.now();worst=Math.max(worst,now-last);last=now;ticks++;},2);
 const turn=yieldingV1(5),end=performance.now()+300;
 let sum=0;
 while(performance.now()<end){for(let index=0;index<2000;index++)sum+=Math.sqrt(index);await turn();}
 clearInterval(timer);
 expect(sum).toBeGreaterThan(0);expect(ticks).toBeGreaterThan(Math.floor(20/(stretchedBudgetMillisecondsV1(1000)/1000)));expect(worst).toBeLessThan(stretchedBudgetMillisecondsV1(120));
 const quick=yieldingV1(1000);let paused=0;
 for(let index=0;index<100_000;index++)if(quick()!==undefined)paused++;
 expect(paused).toBe(0);
 let blocked=0,started=performance.now();
 const heavy=yieldingV1(5),blocker=setInterval(()=>{blocked++;},1);
 while(performance.now()-started<100){for(let index=0;index<5000;index++)Math.sqrt(index);await heavy();}
 clearInterval(blocker);
 expect(blocked).toBeGreaterThan(5);
});

test("the activity slot of the watchdog: the names of the supervisor and of the resilience module agree, a declaration without a watchdog is a no-op, and loop-blocked-restart is reported with its count",()=>{
 const supervisor=readFileSync(join(import.meta.dirname,"..","..","🔁️replica","🛟️supervisor","🟦️.ts"),"utf8"),listed=/const activities=\[([^\]]*)\] as const;/u.exec(supervisor)![1]!.split(",").map(name=>name.trim().slice(1,-1));
 expect(listed).toEqual([...activityNamesV1]);
 expect(()=>activityV1("publish")).not.toThrow();
 const holder=globalThis as unknown as Record<symbol,unknown>,key=Symbol.for("semio.fleet.blockedRestart");
 holder[key]={count:3,lastMs:42};
 try{expect(blockedFailuresV1()).toEqual([{kind:"loop-blocked-restart",count:3,lastMs:42}]);}finally{delete holder[key];}
 expect(blockedFailuresV1()).toEqual([]);
});

test.skipIf(process.platform==="win32")("a seal records the execute bit of every file and a chmod alone, without any change of content, makes the seal stale so that the next seal records the new bit",async()=>{
 const root=temp("bit"),moduleRoot=join(root,"m");
 put(root,"m/a/🟦️.ts","export const a=1;\n");put(root,"m/b/📜️script.ts","#!/usr/bin/env repo\n");
 chmodSync(join(moduleRoot,"b","📜️script.ts"),0o755);
 const digest=await digestOf(moduleRoot),first=await sealCodeV1({root,moduleRoot,digest,digestOf}),file=join(codeDirectoryV1(root),digest,"sealed.json");
 const bits=():Record<string,boolean>=>Object.fromEntries((JSON.parse(readFileSync(file,"utf8")) as {files:{path:string;executable:boolean}[]}).files.map(row=>[row.path,row.executable]));
 expect(first.created).toBe(true);
 expect(bits()).toEqual({"m/a/🟦️.ts":false,"m/b/📜️script.ts":true});
 expect(statSync(join(codeDirectoryV1(root),digest,"files","m","b","📜️script.ts")).mode&0o111).not.toBe(0);
 expect(statSync(join(codeDirectoryV1(root),digest,"files","m","a","🟦️.ts")).mode&0o111).toBe(0);
 expect((await sealCodeV1({root,moduleRoot,digest,digestOf})).created).toBe(false);
 chmodSync(join(moduleRoot,"b","📜️script.ts"),0o644);
 expect(await digestOf(moduleRoot)).toBe(digest);
 expect((await sealCodeV1({root,moduleRoot,digest,digestOf})).created).toBe(true);
 expect(bits()).toEqual({"m/a/🟦️.ts":false,"m/b/📜️script.ts":false});
 expect(() => requireResilienceShapeV1("SealedCode",{version:1,digest,createdMs:1,moduleRoot:"m",files:[{path:"m/a/🟦️.ts",bytes:1,sha256:digest}]},"seal")).toThrow();
});

test("the refused-path record keeps the 200 newest distinct paths exactly like an independent model, survives a write and a read, and rejects a torn or oversized file",async()=>{
 let seed=7;const next=():number=>{seed=(Math.imul(seed,1103515245)+12345)>>>0;return seed;};
 let record=NO_REFUSED_PATHS_V1;const model:{path:string;reason:string;wallMs:number}[]=[];let total=0;
 for(let step=0;step<1500;step++){
  const path=`tickets/${(next()>>>8)%350}/file.txt`,reason=["too-long","illegal-character","unsafe-link"][(next()>>>8)%3]!,wallMs=1_000+step;
  record=noteRefusedPathV1(record,path,reason,wallMs);
  const index=model.findIndex(item=>item.path===path);
  if(index>=0)model.splice(index,1);else total++;
  model.push({path,reason,wallMs});
  if(model.length>REFUSED_PATH_LIMIT_V1)model.shift();
 }
 expect(record.recent).toEqual(model);expect(record.total).toBe(total);expect(record.recent.length).toBe(REFUSED_PATH_LIMIT_V1);
 expect(NO_REFUSED_PATHS_V1).toEqual({version:1,total:0,recent:[]});
 const root=temp("refusals");
 expect(await readRefusedPathsV1(root)).toBeUndefined();
 await writeRefusedPathsV1(root,record);
 expect(await readRefusedPathsV1(root)).toEqual(record);
 writeFileSync(refusedPathsPathV1(root),"{\"version\":1,\"total\":");
 expect(await readRefusedPathsV1(root)).toBeUndefined();
 expect(()=>requireResilienceShapeV1("RefusedPaths",{version:1,total:0,recent:Array.from({length:201},(_,index)=>({path:`p${index}`,reason:"too-long",wallMs:1}))},"x")).toThrow();
 expect(()=>requireResilienceShapeV1("RefusedPaths",{version:1,total:0,recent:[{path:"",reason:"too-long",wallMs:1}]},"x")).toThrow();
 expect(noteRefusedPathV1(NO_REFUSED_PATHS_V1,"p".repeat(5000),"r".repeat(500),1).recent[0]).toEqual({path:"p".repeat(4096),reason:"r".repeat(200),wallMs:1});
});
