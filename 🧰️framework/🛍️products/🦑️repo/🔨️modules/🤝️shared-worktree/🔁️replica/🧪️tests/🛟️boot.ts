import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {createHash} from "node:crypto";
import {copyFileSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir,uptime} from "node:os";
import {dirname,join} from "node:path";
import {codeDigestV1,createAttesterV1,fleetReportV1,readAttestationV1,writeAttestationV1} from "../../🩺️attest/🟦️.ts";
import {silentLatencyV1} from "../../🎣️tiers/🟦️.ts";
import {installBootStageV1,listCodeSnapshotsV1,listStoreCodeV1,publishCodeSnapshotV1,readBeatV1,readBlockedRestartV1,readHealthV1,readRunHistoryV1,sealCodeV1} from "../../🛡️resilience/🟦️.ts";
import {admitCodeV1,createAdmissionStateV1,applyQueue,closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,ReplicaBusyV1,runV1,seedV1,statusV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../🟦️.ts";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";
import {bootLimitsV1} from "../🛟️supervisor/🟦️.ts";
import {readProcessStartV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

setDefaultTimeout(180_000);
const scratch:string[]=[],module_="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`boot-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=30_000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const realScript=join(import.meta.dirname,"..","📜️script.ts"),realSupervisor=join(import.meta.dirname,"..","🛟️supervisor","🟦️.ts");
const cleanEnvironment=(extra:Record<string,string>={}):Record<string,string>=>{const environment:Record<string,string>={};for(const [name,value] of Object.entries(process.env))if(value!==undefined&&!name.startsWith("SEMIO_FLEET_"))environment[name]=value;return {...environment,...extra};};

type Mode="ok"|"throws"|"hold"|"block"|"late"|"busy"|"hang"|"wait";
const command=(label:string,mode:Mode):string=>`export async function runReplicaCommandV1(args:string[],context:{environment:Record<string,string|undefined>;write:(line:string)=>void;signal:AbortSignal}){
 if(args[0]!=="run"){context.write(JSON.stringify({ran:${JSON.stringify(label)},args,watch:context.environment.SEMIO_FLEET_CODE_WATCH??null}));return {exitCode:0};}
 ${mode==="throws"?"throw new TypeError(\"tables.dirs.adopt is not a function\");":mode==="busy"?"return {exitCode:75};":mode==="hang"||mode==="wait"?`${mode==="wait"?"context.write(JSON.stringify({version:1,event:\"blocked\",reason:\"worktree-access-denied\"}));":""}await new Promise<void>(done=>{if(context.signal.aborted)done();context.signal.addEventListener(\"abort\",()=>done(),{once:true});});return {exitCode:0};`:`context.write(JSON.stringify({version:1,event:"started"}));
 context.write(JSON.stringify({ran:${JSON.stringify(label)},args,watch:context.environment.SEMIO_FLEET_CODE_WATCH??null,fallback:context.environment.SEMIO_FLEET_CODE_FALLBACK??null,restart:(globalThis as unknown as Record<symbol,unknown>)[Symbol.for(\"semio.fleet.blockedRestart\")]??null}));
 ${mode==="hold"?"await new Promise<void>(done=>{if(context.signal.aborted)done();context.signal.addEventListener(\"abort\",()=>done(),{once:true});});":""}
 ${mode==="block"?"await new Promise<void>(done=>setTimeout(done,300));(globalThis as unknown as Record<symbol,((name:string)=>void)|undefined>)[Symbol.for(\"semio.fleet.activity\")]?.(\"publish\");const end=Date.now()+Number(context.environment.BLOCK_FOR??\"8000\");while(Date.now()<end){}":""}
 ${mode==="late"?"await new Promise<void>(done=>setTimeout(done,1500));throw new Error(\"late failure\");":""}`}
 return {exitCode:0};
}
`;
const resilienceStub="import {existsSync} from \"node:fs\";\nimport {join} from \"node:path\";\nexport async function selfCheckV1(root:string){return {ok:!existsSync(join(root,\"broken.flag\")),error:\"broken.flag present\"};}\nexport async function startRejectionV1(root:string){return existsSync(join(root,\"revert.flag\"))?\"ran before and was superseded (revert)\":null;}\n";
const tree=(label:string,worktree:string):{repo:string;module:string;script:string}=>{
 const repo=temp(label),module=join(repo,...module_.split("/")),script=join(module,"🔁️replica","📜️script.ts");
 mkdirSync(dirname(script),{recursive:true});copyFileSync(realScript,script);
 mkdirSync(join(module,"🔁️replica","🛟️supervisor"),{recursive:true});copyFileSync(realSupervisor,join(module,"🔁️replica","🛟️supervisor","🟦️.ts"));
 put(module,"🔁️replica/⌨️command/🟦️.ts",worktree);
 put(module,"🛡️resilience/🟦️.ts",resilienceStub);
 put(module,"🩺️attest/🟦️.ts","export async function codeDigestV1(){return \"d\".repeat(64);}\n");
 return {repo,module,script};
};
const seal=async(where:{repo:string;module:string},label:string,mode:Mode,now:number):Promise<string>=>{
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command(label,mode));
 const digest=await codeDigestV1(where.module);
 expect((await sealCodeV1({root:where.repo,moduleRoot:where.module,digest,digestOf:codeDigestV1,now:()=>now})).created).toBe(true);
 return digest;
};
type Run={code:number;out:string;err:string;lines:Record<string,unknown>[];seconds:number};
const stage=async(where:{repo:string;script:string},args:string[],extra:Record<string,string>={}):Promise<Run>=>{
 const started=Date.now(),child=Bun.spawn([process.execPath,where.script,...args],{stdout:"pipe",stderr:"pipe",cwd:where.repo,env:cleanEnvironment({SEMIO_FLEET_BOOT_STORE_MS:"2000",...extra})});
 const [out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 return {code,out,err,seconds:(Date.now()-started)/1000,lines:out.split("\n").filter(line=>line.startsWith("{")).map(line=>JSON.parse(line) as Record<string,unknown>)};
};
const ran=(run:Run):Record<string,unknown>|undefined=>run.lines.find(line=>"ran" in line);
const machine=(repo:string,store:string):void=>{put(repo,".cache/f/machine.json",JSON.stringify({version:1,originId:"w1",groupId:"g1",storeRoot:store,key:"0".repeat(32)}));};

test("a worktree daemon that throws while opening its state falls back to the newest sealed snapshot, which keeps running with the code-fallback guard, and the failing code is not retried as a snapshot",async()=>{
 const where=tree("open-throws",command("worktree","throws"));
 const older=await seal(where,"snapshot-old","ok",1000),newer=await seal(where,"snapshot-new","ok",2000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","throws"));
 const run=await stage(where,["run"]);
 expect(run.code).toBe(0);
 expect(ran(run)).toMatchObject({ran:"snapshot-new",args:["run"],watch:expect.any(String)});
 const fallback=JSON.parse(ran(run)!["fallback"] as string) as {guard:string;source:string;reason:string};
 expect(fallback).toMatchObject({guard:"code-fallback",source:"snapshot"});expect(fallback.reason).toContain("worktree");
 expect(run.err).toContain("tables.dirs.adopt is not a function");expect(run.err).toContain("Running the sealed code");expect(run.err).toContain("Der versiegelte Code");
 expect((await listCodeSnapshotsV1(where.repo)).map(item=>item.digest)).toEqual([newer,older]);
 put(where.repo,`.cache/f/code/${newer}/files/${module_}/🔁️replica/⌨️command/🟦️.ts`,command("snapshot-new","throws"));
 const again=await stage(where,["run"]);
 expect(ran(again)!["ran"]).toBe("snapshot-old");
 void older;
});

test("a worktree module that does not even import (syntax error, missing module) falls back to the sealed snapshot with the reason in both languages",async()=>{
 const where=tree("import-throws",command("worktree","ok"));
 const sealed=await seal(where,"snapshot","ok",1000);
 for(const broken of ["export const broken=;\n","import {nothing} from \"./gone.ts\";\nexport const run=nothing;\n"]){
  put(where.module,"🔁️replica/⌨️command/🟦️.ts",broken);
  const run=await stage(where,["run"]);
  expect(run.code).toBe(0);expect(ran(run)!["ran"]).toBe("snapshot");
  expect(run.err).toContain("does not start");expect(run.err).toContain("startet nicht");
 }
 expect((await listCodeSnapshotsV1(where.repo)).map(item=>item.digest)).toEqual([sealed]);
});

test("a snapshot that fails early is refused for good and the next start does not try it again",async()=>{
 const where=tree("refused",command("worktree","throws"));
 const older=await seal(where,"snapshot-old","ok",1000);
 const newer=await seal(where,"snapshot-new","throws",2000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","throws"));
 const first=await stage(where,["run"]);
 expect(ran(first)!["ran"]).toBe("snapshot-old");
 expect((await readRunHistoryV1(where.repo)).refused).toContain(newer);
 const second=await stage(where,["run"]);
 expect(ran(second)!["ran"]).toBe("snapshot-old");
 expect(second.err).not.toContain(newer);
 void older;
});

test("without any local snapshot the stage fetches a verified snapshot from the store, prefers the one that a live origin attests, and skips a tampered one",async()=>{
 const store=temp("store"),donor=tree("donor",command("donor","ok"));
 const first=await seal(donor,"store-first","ok",1000),second=await seal(donor,"store-second","ok",2000);
 for(const digest of [first,second])expect((await publishCodeSnapshotV1({root:donor.repo,store,digest,originId:"p1"})).published).toBe(true);
 expect((await listStoreCodeV1(store)).map(item=>item.digest)).toEqual([second,first]);
 const where=tree("fetch",command("worktree","throws"));machine(where.repo,store);
 put(store,"v1/origins/peer/attest.json",JSON.stringify({code:first,wallMs:Date.now()}));
 const attested=await stage(where,["run"],{SEMIO_FLEET_BOOT_STORE_MS:"5000"});
 expect(attested.code).toBe(0);
 expect(ran(attested)).toMatchObject({ran:"store-first"});
 expect(JSON.parse(ran(attested)!["fallback"] as string)).toMatchObject({guard:"code-fallback",source:"store"});
 expect((await listCodeSnapshotsV1(where.repo)).map(item=>item.digest)).toContain(first);
 rmSync(join(where.repo,".cache","f","code"),{recursive:true,force:true});
 rmSync(join(store,"v1","origins"),{recursive:true,force:true});
 put(store,`v1/bootstrap/code/${second}/files/${module_}/🔁️replica/⌨️command/🟦️.ts`,"export const tampered=;\n");
 const skipped=await stage(where,["run"],{SEMIO_FLEET_BOOT_STORE_MS:"5000"});
 expect(ran(skipped)).toMatchObject({ran:"store-first"});
 expect(skipped.err).toContain("could not be fetched");
 expect((await listCodeSnapshotsV1(where.repo)).map(item=>item.digest)).not.toContain(second);
});

test("a store snapshot that two live origins run is preferred over an older local snapshot, one that only a single origin runs is not",async()=>{
 const store=temp("store"),donor=tree("donor-proven",command("donor","ok"));
 const proven=await seal(donor,"store-proven","ok",3000);
 expect((await publishCodeSnapshotV1({root:donor.repo,store,digest:proven,originId:"p1"})).published).toBe(true);
 const where=tree("prefer",command("worktree","throws"));machine(where.repo,store);
 await seal(where,"local-old","ok",1000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","throws"));
 put(store,"v1/origins/peer1/attest.json",JSON.stringify({code:proven,wallMs:Date.now()}));
 const single=await stage(where,["run"],{SEMIO_FLEET_BOOT_STORE_MS:"5000"});
 expect(ran(single)).toMatchObject({ran:"local-old"});
 put(store,"v1/origins/peer2/attest.json",JSON.stringify({code:proven,wallMs:Date.now()}));
 const both=await stage(where,["run"],{SEMIO_FLEET_BOOT_STORE_MS:"5000"});
 expect(ran(both)).toMatchObject({ran:"store-proven"});
 expect(JSON.parse(ran(both)!["fallback"] as string)).toMatchObject({guard:"code-fallback",source:"store"});
 put(store,"v1/origins/peer1/attest.json",JSON.stringify({code:proven,wallMs:Date.now()-3_600_000}));
 rmSync(join(where.repo,".cache","f","code",proven),{recursive:true,force:true});
 put(store,"v1/origins/peer2/attest.json",JSON.stringify({code:proven,wallMs:Date.now()-3_600_000}));
 const none=await stage(where,["run"],{SEMIO_FLEET_BOOT_STORE_MS:"5000"});
 expect(ran(none)).toMatchObject({ran:"local-old"});
});

test("with the store unreachable a local snapshot runs at once, and with neither the stage gives up in bounded time without a half-installed snapshot",async()=>{
 const where=tree("offline",command("worktree","throws"));machine(where.repo,join(temp("gone"),"missing","store"));
 await seal(where,"snapshot","ok",1000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","throws"));
 const local=await stage(where,["run"]);
 expect(ran(local)!["ran"]).toBe("snapshot");expect(local.seconds).toBeLessThan(20);
 rmSync(join(where.repo,".cache","f","code"),{recursive:true,force:true});
 const none=await stage(where,["run"],{SEMIO_FLEET_BOOT_EARLY_MS:"500"});
 expect(none.code).toBe(1);expect(none.err).toContain("No replica code starts");expect(none.err).toContain("Kein Replica-Code startet");
 expect(existsSync(join(where.repo,".cache","f","code"))).toBe(false);
});

test("code that ran before and was superseded is not started while a snapshot starts, and runs as a last resort when nothing else does",async()=>{
 const where=tree("revert",command("worktree","ok"));
 await seal(where,"snapshot","ok",1000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","ok"));writeFileSync(join(where.repo,"revert.flag"),"");
 const preferred=await stage(where,["run"]);
 expect(ran(preferred)!["ran"]).toBe("snapshot");
 rmSync(join(where.repo,".cache","f","code"),{recursive:true,force:true});
 const last=await stage(where,["run"]);
 expect(ran(last)!["ran"]).toBe("worktree");expect(last.err).toContain("running the worktree code after all");
});

test("a daemon that finds its lock held or its origin taken is not a code fault: the stage ends with that code and starts no fallback",async()=>{
 const where=tree("busy",command("worktree","busy"));
 await seal(where,"snapshot","ok",1000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","busy"));
 const run=await stage(where,["run"]);
 expect(run.code).toBe(75);expect(run.lines.filter(line=>"ran" in line)).toEqual([]);
 expect((await readRunHistoryV1(where.repo)).refused).toEqual([]);
});

test("a daemon that never becomes ready is killed and the next code is tried, but one that waits for the consent to read the worktree is left alone",async()=>{
 const stuck=tree("hang",command("worktree","hang"));
 await seal(stuck,"snapshot","ok",1000);
 put(stuck.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","hang"));
 const run=await stage(stuck,["run"],{SEMIO_FLEET_BOOT_READY_MS:"600",SEMIO_FLEET_BOOT_BEAT_MS:"100"});
 expect(run.code).toBe(0);expect(ran(run)!["ran"]).toBe("snapshot");expect(run.err).toContain("not ready after");
 const waiting=tree("wait",command("worktree","wait"));
 const child=Bun.spawn([process.execPath,waiting.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:waiting.repo,env:cleanEnvironment({SEMIO_FLEET_BOOT_READY_MS:"600",SEMIO_FLEET_BOOT_BEAT_MS:"100"})});
 await Bun.sleep(2500);
 expect(child.exitCode).toBeNull();
 child.kill("SIGTERM");
 expect(await child.exited).toBe(0);
});

test("a daemon that ran healthily is not replaced when it fails later, and a clean exit is final",async()=>{
 const where=tree("late",command("worktree","late"));
 await seal(where,"snapshot","ok",1000);
 put(where.module,"🔁️replica/⌨️command/🟦️.ts",command("worktree","late"));
 const run=await stage(where,["run"],{SEMIO_FLEET_BOOT_EARLY_MS:"400",SEMIO_FLEET_BOOT_BEAT_MS:"100"});
 expect(run.code).not.toBe(0);
 expect(run.lines.filter(line=>"ran" in line).map(line=>line["ran"])).toEqual(["worktree"]);
 expect(run.err).toContain("The daemon ended with");
 const clean=tree("clean",command("worktree","ok"));
 const finished=await stage(clean,["run"]);
 expect(finished.code).toBe(0);expect(ran(finished)!["ran"]).toBe("worktree");expect(ran(finished)!["fallback"]).toBeNull();
});

test("a blocked event loop shows in the beat file while it lasts, is recorded in health.json as a loop-blocked failure and ends the daemon after the kill limit",async()=>{
 const where=tree("blocked",command("worktree","block"));
 put(where.repo,".cache/f/health.json",JSON.stringify({version:1,wallMs:1,pid:1,code:"a".repeat(64),store:"ok",storeSinceMs:1,pending:null,failures:[],trust:{scope:"shared-source",verifiedMs:null,lostMs:null,lostKind:null}}));
 const child=Bun.spawn([process.execPath,where.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:where.repo,env:cleanEnvironment({BLOCK_FOR:"60000",SEMIO_FLEET_BOOT_BLOCK_MS:"1000",SEMIO_FLEET_BOOT_KILL_MS:"4000",SEMIO_FLEET_BOOT_BEAT_MS:"150",SEMIO_FLEET_BOOT_EARLY_MS:"300"})});
 const started=Date.now();
 await until(async()=>((await readBeatV1(where.repo))?.lagMs??0)>=1500,15000);
 const beat=(await readBeatV1(where.repo))!;
 expect(beat.phase).toBe("run");expect(beat.source).toBe("worktree");
 await until(()=>(JSON.parse(readFileSync(join(where.repo,".cache","f","health.json"),"utf8")) as {failures:{kind:string}[]}).failures.some(item=>item.kind==="loop-blocked"),10000);
 const code=await child.exited;
 expect(code).not.toBe(0);
 expect((Date.now()-started)/1000).toBeLessThan(25);
 const ended=await readBlockedRestartV1(where.repo);
 expect(ended?.reason).toBe("blocked");expect(ended?.lagMs).toBeGreaterThanOrEqual(4000);
 expect((await readHealthV1(where.repo))?.failures.map(item=>item.kind)).toContain("loop-blocked");
 expect(existsSync(join(where.repo,".cache","f","boot","beat.json"))).toBe(false);
});

test("a daemon that runs and whose loop stays blocked for 120 s by default is ended with a loop-blocked-restart record naming what it was doing and a stack sample, and the next daemon of the machine reports the end",async()=>{
 expect(bootLimitsV1({}).killMs).toBe(120_000);
 const where=tree("blocked-restart",command("worktree","block"));
 const child=Bun.spawn([process.execPath,where.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:where.repo,env:cleanEnvironment({BLOCK_FOR:"60000",SEMIO_FLEET_BOOT_BLOCK_MS:"500",SEMIO_FLEET_BOOT_KILL_MS:"2500",SEMIO_FLEET_BOOT_BEAT_MS:"100",SEMIO_FLEET_BOOT_EARLY_MS:"300"})});
 const started=Date.now(),code=await child.exited;
 expect(code).not.toBe(0);expect((Date.now()-started)/1000).toBeLessThan(25);
 const record=(await readBlockedRestartV1(where.repo))!;
 expect(record.count).toBe(1);expect(record.reason).toBe("blocked");expect(record.phase).toBe("run");expect(record.activity).toBe("publish");expect(record.lagMs).toBeGreaterThanOrEqual(2500);expect(record.longestActivity).toBe("publish");expect(record.longestMs!).toBeGreaterThanOrEqual(2500);
 if(process.platform==="darwin"){expect(record.sample).not.toBeNull();expect(readFileSync(join(where.repo,".cache","f","boot",record.sample!),"utf8")).toContain("Call graph");}
 expect(new Response(child.stderr).text()).resolves.toContain("blocked for");
 const next=tree("blocked-restart-next",command("worktree","ok"));
 mkdirSync(join(next.repo,".cache","f","boot"),{recursive:true});copyFileSync(join(where.repo,".cache","f","boot","loop-blocked-restart.json"),join(next.repo,".cache","f","boot","loop-blocked-restart.json"));
 const finished=await stage(next,["run"]);
 expect(finished.code).toBe(0);expect(ran(finished)!["restart"]).toEqual({count:1,lastMs:record.wallMs});
 const again=await stage(where,["run"],{BLOCK_FOR:"60000",SEMIO_FLEET_BOOT_BLOCK_MS:"500",SEMIO_FLEET_BOOT_KILL_MS:"2500",SEMIO_FLEET_BOOT_BEAT_MS:"100",SEMIO_FLEET_BOOT_EARLY_MS:"300"});
 expect(again.code).not.toBe(0);expect((await readBlockedRestartV1(where.repo))!.count).toBe(2);
});

test("SIGTERM ends the stage within the grace period even when the daemon is blocked, and a daemon that is not blocked stops cleanly on it",async()=>{
 const blocked=tree("term-blocked",command("worktree","block"));
 const child=Bun.spawn([process.execPath,blocked.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:blocked.repo,env:cleanEnvironment({BLOCK_FOR:"30000",SEMIO_FLEET_BOOT_GRACE_MS:"800",SEMIO_FLEET_BOOT_BEAT_MS:"150"})});
 await until(async()=>((await readBeatV1(blocked.repo))?.lagMs??0)>=1000,15000);
 const sent=Date.now();child.kill("SIGTERM");
 await child.exited;
 expect((Date.now()-sent)/1000).toBeLessThan(6);
 const healthy=tree("term-healthy",command("worktree","hold"));
 const calm=Bun.spawn([process.execPath,healthy.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:healthy.repo,env:cleanEnvironment({SEMIO_FLEET_BOOT_BEAT_MS:"150"})});
 await until(async()=>(await readBeatV1(healthy.repo))?.phase==="run",15000);
 calm.kill("SIGTERM");
 expect(await calm.exited).toBe(0);
});

test("a daemon whose supervisor was killed ends by itself",async()=>{
 const where=tree("orphan",command("worktree","hold"));
 const child=Bun.spawn([process.execPath,where.script,"run"],{stdout:"pipe",stderr:"pipe",cwd:where.repo,env:cleanEnvironment({SEMIO_FLEET_BOOT_BEAT_MS:"100"})});
 await until(async()=>(await readBeatV1(where.repo))?.phase==="run",15000);
 const pid=(await readBeatV1(where.repo))!.pid;
 expect(pid).not.toBe(child.pid);
 process.kill(child.pid,"SIGKILL");
 await child.exited;
 await until(()=>{try{process.kill(pid,0);return false;}catch{return true;}},10000);
});

test("the worktree entry prefers the verified copy outside the module and ignores a torn or tampered one",async()=>{
 const where=tree("copy",command("worktree","ok"));
 const marked=join(temp("marked"),"script.ts");
 writeFileSync(marked,readFileSync(realSupervisor,"utf8")+"process.stderr.write(\"from-copy\\n\");\n");
 expect((await installBootStageV1(where.repo,marked)).installed).toBe(true);
 expect((await installBootStageV1(where.repo,marked)).installed).toBe(false);
 const copied=await stage(where,["run"]);
 expect(copied.err).toContain("from-copy");expect(ran(copied)!["ran"]).toBe("worktree");
 const status=await stage(where,["status"]);
 expect(status.err).not.toContain("from-copy");
 writeFileSync(join(where.repo,".cache","f","boot","stage.ts"),readFileSync(marked,"utf8")+"// tampered\n");
 expect((await stage(where,["run"])).err).not.toContain("from-copy");
 writeFileSync(join(where.repo,".cache","f","boot","stage.ts"),"export const torn=;\n");
 writeFileSync(join(where.repo,".cache","f","boot","stage.json"),JSON.stringify({version:1,sha256:createHash("sha256").update("export const torn=;\n").digest("hex"),bytes:20,wallMs:1}));
 const torn=await stage(where,["run"]);
 expect(torn.code).toBe(0);expect(ran(torn)!["ran"]).toBe("worktree");
});

const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[]} as const;
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};

test("a healthy daemon seals its code, refreshes the boot stage copy and publishes its sealed snapshot to the store, and prunes the old ones",async()=>{
 const where=tree("healthy",command("worktree","ok")),store=temp("store");
 put(where.module,"x/🟦️.ts","export const a=1;\n");
 const options:ReplicaOptionsV1={worktree:where.repo,storeRoot:store,originId:"h1",groupId:"g1",...fast,codeRoot:where.module,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),publishCodeMilliseconds:300,codeKeep:1};
 put(where.repo,"a.txt","alpha\n");
 await seedV1(options);
 const controller=new AbortController(),events:ReplicaEventV1[]=[],done=runV1(options,controller.signal,event=>events.push(event));
 const digest=await codeDigestV1(where.module);
 await until(async()=>(await listStoreCodeV1(store)).some(item=>item.digest===digest),20000);
 expect(existsSync(join(where.repo,".cache","f","boot","stage.ts"))).toBe(true);
 expect(readFileSync(join(where.repo,".cache","f","boot","stage.ts"),"utf8")).toBe(readFileSync(realSupervisor,"utf8"));
 const published=join(store,"v1","bootstrap","code",digest);
 expect(existsSync(join(published,"sealed.json"))).toBe(true);
 const seal=JSON.parse(readFileSync(join(published,"sealed.json"),"utf8")) as {digest:string;files:{path:string;sha256:string}[]};
 expect(seal.digest).toBe(digest);
 for(const file of seal.files)expect(existsSync(join(published,"files",...file.path.split("/")))).toBe(true);
 controller.abort();await done;
});

test("a refused first bootstrap snapshot publication is retried after the store becomes writable",async()=>{
 const row=JSON.parse(readFileSync(join(import.meta.dirname,"🧫️fixtures","📤️code-publication.json"),"utf8"));
 const where=tree("share-retry",command("worktree","ok")),store=temp("retry-store");
 const options:ReplicaOptionsV1={worktree:where.repo,storeRoot:store,originId:"h1",groupId:"g1",...fast,codeRoot:where.module,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),publishCodeMilliseconds:row.publishAfter,entryDirectory:join(where.repo,".cache","entry")};
 await seedV1(options);
 put(store,"v1/bootstrap/code",row.blocker);
 const controller=new AbortController(),events:ReplicaEventV1[]=[],done=runV1(options,controller.signal,event=>events.push(event));
 try{
  const digest=await codeDigestV1(where.module),target=join(store,"v1","bootstrap","code",digest);
  const blocker=join(store,"v1","bootstrap","code");
  await until(()=>events.some(event=>event.event==="error"&&event.stage==="code-snapshot"&&event.message.includes(blocker)),row.timeout);
  expect(existsSync(target)).toBe(false);
  rmSync(blocker);
  await until(async()=>(await listStoreCodeV1(store)).some(item=>item.digest===digest),row.timeout);
  const seal=JSON.parse(readFileSync(join(target,"sealed.json"),"utf8")) as {files:{path:string;sha256:string}[]};
  expect(seal.files.length).toBeGreaterThan(0);
  for(const file of seal.files)expect(createHash("sha256").update(readFileSync(join(target,"files",...file.path.split("/")))).digest("hex")).toBe(file.sha256);
 }finally{controller.abort();await done;}
});

test("the attestation of a machine on a fallback code carries the code-fallback guard and the fleet report warns about it",async()=>{
 const store=temp("store"),worktree=temp("w"),code=temp("c");put(code,"x/🟦️.ts","export const a=1;\n");
 const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"f1",groupId:"g1",...fast,codeRoot:code,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),fallback:{guard:"code-fallback",source:"store",reason:"the worktree code does not start"}};
 put(worktree,"a.txt","alpha\n");
 await seedV1(options);
 const controller=new AbortController(),done=runV1(options,controller.signal);
 await until(async()=>(await readAttestationV1(store,"f1"))?.fallback!==undefined);
 expect((await readAttestationV1(store,"f1"))!.fallback).toEqual({guard:"code-fallback",source:"store",reason:"the worktree code does not start"});
 const report=await fleetReportV1(store);
 expect(report.warnings).toContainEqual({originId:"f1",kind:"code-fallback",detail:"store: the worktree code does not start"});
 controller.abort();await done;
});

test("a daemon on a fallback code returns to the identical worktree code after the self-check and the return delay, and not before",async()=>{
 const store=temp("store"),worktree=temp("w"),code=temp("c");put(code,"x/🟦️.ts","export const a=1;\n");
 const checks:string[]=[];
 const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"r1",groupId:"g1",...fast,codeRoot:code,watchRoot:code,gitState:async()=>git,gitView:async()=>null,selfCheck:async digest=>{checks.push(digest);return {ok:true};},fallback:{guard:"code-fallback",source:"snapshot",reason:"x"},returnMilliseconds:1500} as ReplicaOptionsV1;
 put(worktree,"a.txt","alpha\n");
 await seedV1(options);
 const started=Date.now(),events:ReplicaEventV1[]=[],controller=new AbortController(),done=runV1(options,controller.signal,event=>events.push(event));
 await until(()=>events.some(event=>event.event==="restarting"),30000);
 expect(Date.now()-started).toBeGreaterThanOrEqual(1400);
 expect(checks).toHaveLength(1);
 await done;controller.abort();
});

const moduleCopy=(label:string):{repo:string;module:string;script:string}=>{
 const repo=temp(label),framework=join(repo,"🧰️framework"),live=join(import.meta.dirname,"..","..","..","..","..","..");
 mkdirSync(join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules"),{recursive:true});
 symlinkSync(join(live,"🔨️modules"),join(framework,"🔨️modules"));symlinkSync(join(live,"📦️packages"),join(framework,"📦️packages"));
 const modules=join(import.meta.dirname,"..","..",".."),module=join(repo,...module_.split("/"));
 for(const entry of readdirSync(modules))if(entry!=="🤝️shared-worktree")symlinkSync(join(modules,entry),join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules",entry));
 cpSync(join(import.meta.dirname,"..",".."),module,{recursive:true,filter:source=>!source.includes("node_modules")});
 return {repo,module,script:join(module,"🔁️replica","📜️script.ts")};
};
const sabotage=(module:string):void=>{
 const file=join(module,"🎣️tiers","🟦️.ts"),text=readFileSync(file,"utf8"),at=text.indexOf(" adopt(mtime:");
 expect(at).toBeGreaterThan(0);
 writeFileSync(file,`${text.slice(0,at)} adoptStale(mtime:${text.slice(at+" adopt(mtime:".length)}`);
};

test("the incident: a worktree module whose files do not fit together is refused by the self-check and, if it is started anyway, the machine runs its sealed snapshot",async()=>{
 const good=moduleCopy("mix"),store=temp("store");
 const options:ReplicaOptionsV1={worktree:good.repo,storeRoot:store,originId:"w1",groupId:"g1",pollMilliseconds:20,scanSeconds:5,persistMilliseconds:0};
 put(good.repo,"a.txt","alpha\n");
 await seedV1(options);
 const ok=Bun.spawnSync([process.execPath,good.script,"self-check","--lang","en"],{cwd:good.repo,env:cleanEnvironment(),stdout:"pipe",stderr:"pipe"});
 expect(ok.exitCode,ok.stdout.toString()+ok.stderr.toString()).toBe(0);
 const digest=await codeDigestV1(good.module);
 expect((await sealCodeV1({root:good.repo,moduleRoot:good.module,digest,digestOf:codeDigestV1})).created).toBe(true);
 sabotage(good.module);
 const bad=Bun.spawnSync([process.execPath,good.script,"self-check","--lang","en"],{cwd:good.repo,env:cleanEnvironment(),stdout:"pipe",stderr:"pipe"});
 expect(bad.exitCode).toBe(1);expect(bad.stdout.toString()+bad.stderr.toString()).toMatch(/adopt|opened|boot/u);
 const child=Bun.spawn([process.execPath,good.script,"run","--worktree",good.repo,"--store",store,"--lang","en"],{stdout:"pipe",stderr:"pipe",cwd:good.repo,env:cleanEnvironment()});
 const output=new Response(child.stderr).text();
 await until(async()=>(await readAttestationV1(store,"w1"))?.fallback!==undefined,90_000);
 const attestation=(await readAttestationV1(store,"w1"))!;
 expect(attestation.code).toBe(digest);
 expect(attestation.fallback).toMatchObject({guard:"code-fallback",source:"snapshot"});
 await until(async()=>(await readBeatV1(good.repo))?.phase==="run",15000);
 expect((await readBeatV1(good.repo))?.source).toBe("snapshot");
 const report=await statusV1({worktree:good.repo,storeRoot:store,originId:"w1",groupId:"g1"});
 expect(report.code).toEqual({source:"snapshot",phase:"run"});expect(report.daemon.running).toBe(true);
 child.kill("SIGTERM");
 expect(await child.exited).toBe(0);
 expect(await output).toContain("adopt");
});

const attest=(store:string,origin:string,code:string,wallMs:number=Date.now()):Promise<void>=>writeAttestationV1(store,{version:1,originId:origin,groupId:"g1",platform:"linux",wallMs,hlc:`0000001000000-000000-${origin}`,sync:true,code,cursors:{},files:1,manifest:"1".repeat(64),head:null,indexTree:null,view:null,guards:null,latency:silentLatencyV1(),queue:{incoming:0}});

test("new code is admitted when it was written on this machine, when another origin has run it healthy for a minute or left a snapshot of it in the store, and never at once when it arrived from elsewhere unproven, whole or in part",async()=>{
 const store=temp("store"),a=temp("a"),b=temp("b"),module=join(a,"mod");
 put(a,"mod/x/🟦️.ts","export const x=1;\n");put(a,"mod/y/🟦️.ts","export const y=1;\n");
 const optionsA:ReplicaOptionsV1={worktree:a,storeRoot:store,originId:"a1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false},optionsB:ReplicaOptionsV1={worktree:b,storeRoot:store,originId:"b1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 await seedV1(optionsA);
 mkdirSync(b,{recursive:true});await joinV1(optionsB);
 const running=await codeDigestV1(module);
 expect((await sealCodeV1({root:a,moduleRoot:module,digest:running,digestOf:codeDigestV1})).created).toBe(true);
 const ra=await openReplicaV1(optionsA),rb=await openReplicaV1(optionsB);
 let clock=Date.now();
 const bounded=<T>(work:()=>Promise<T>):Promise<T>=>work(),state=createAdmissionStateV1(),ask=(digest:string)=>admitCodeV1(ra,{digest,running,moduleRoot:module,bounded,state,now:()=>clock});
 expect(await ask(running)).toEqual({ok:true});
 put(a,"mod/x/🟦️.ts","export const x=2;\n");await publishOnceV1(ra);
 const authored=await codeDigestV1(module);
 expect(await ask(authored)).toEqual({ok:true});
 put(b,"mod/y/🟦️.ts","export const y=2;\n");await publishOnceV1(rb);
 await followOnceV1(ra);await applyQueue(ra,undefined);
 expect(readFileSync(join(a,"mod","y","🟦️.ts"),"utf8")).toBe("export const y=2;\n");
 const mixed=await codeDigestV1(module);
 expect(mixed).not.toBe(authored);
 expect(await ask(mixed)).toMatchObject({ok:false,reason:expect.stringContaining("no origin runs this code yet")});
 await attest(store,"a1",mixed,clock);
 expect((await ask(mixed)).ok).toBe(false);
 await attest(store,"b1",mixed,clock);
 expect(await ask(mixed)).toMatchObject({ok:false,reason:expect.stringContaining("b1 runs this code and has to stay healthy for 60 s more")});
 clock+=30_000;await attest(store,"b1",mixed,clock);
 expect(await ask(mixed)).toMatchObject({ok:false,reason:expect.stringContaining("30 s more")});
 clock+=30_000;await attest(store,"b1",mixed,clock);
 expect(await ask(mixed)).toEqual({ok:true});
 put(b,"mod/x/🟦️.ts","export const x=3;\n");await publishOnceV1(rb);
 await followOnceV1(ra);await applyQueue(ra,undefined);
 const partial=await codeDigestV1(module);
 expect(partial).not.toBe(mixed);
 expect((await ask(partial)).ok).toBe(false);
 expect((await sealCodeV1({root:a,moduleRoot:module,digest:partial,digestOf:codeDigestV1})).created).toBe(true);
 expect((await publishCodeSnapshotV1({root:a,store,digest:partial,originId:"b1"})).published).toBe(true);
 expect(await ask(partial)).toEqual({ok:true});
 await closeReplicaV1(ra);await closeReplicaV1(rb);
});

test("the attester waits for the admission, shows the wait as pending without refusing the code, asks again every heartbeat and restarts only after it is granted",async()=>{
 let clock=1_000_000,allowed=false;
 const root=temp("code"),asked:string[]=[],checks:string[]=[];put(root,"x/🟦️.ts","export const a=1;\n");
 const attester=createAttesterV1({storeRoot:temp("store"),originId:"a",groupId:"g1",platform:"linux",now:()=>clock,heartbeatMilliseconds:1000,stableMilliseconds:1000,jitterSpanMilliseconds:0,minimumUptimeMilliseconds:0,codeRoot:root,git:async()=>git,reachable:async()=>false,admit:async digest=>{asked.push(digest);return allowed?{ok:true}:{ok:false,reason:"nobody runs it"};},check:async digest=>{checks.push(digest);return {ok:true};}});
 await attester.start();
 const snapshot={sync:true,files:1,manifest:"1".repeat(64),cursors:{},incoming:0,hlc:"0000001000000-000000-a",activity:0};
 const step=async():Promise<string>=>{clock+=1000;const result=await attester.tick(snapshot);await Bun.sleep(2);return result;};
 put(root,"x/🟦️.ts","export const a=2;\n");
 const first=await codeDigestV1(root);
 for(let n=0;n<12;n++)expect(await step()).toBe("idle");
 expect(checks).toEqual([]);expect(new Set(asked)).toEqual(new Set([first]));expect(asked.length).toBeGreaterThanOrEqual(4);
 expect(attester.pending()).toEqual({code:first,ok:false,reason:"waiting: nobody runs it"});
 put(root,"x/🟦️.ts","export const a=3;\n");
 const second=await codeDigestV1(root);
 for(let n=0;n<6;n++)await step();
 expect(asked).toContain(second);expect(attester.pending()?.code).toBe(second);
 allowed=true;
 let restarted=false;
 for(let n=0;n<12&&!restarted;n++)restarted=await step()==="restart";
 expect(restarted).toBe(true);expect(checks).toEqual([second]);expect(attester.pending()).toBeNull();
});

test("status reports a daemon with a blocked event loop as running and busy, and the lock of such a daemon is not taken over, until its beat stops",async()=>{
 const store=temp("store"),worktree=temp("w");
 put(worktree,"a.txt","alpha\n");
 const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"s1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 await seedV1(options);
 const holder=Bun.spawn([process.execPath,"-e","setTimeout(()=>{},60000)"],{stdout:"ignore",stderr:"ignore"});
 try{
  const lock=join(worktree,".cache","f","daemon.lock");
  writeFileSync(lock,JSON.stringify({version:1,pid:holder.pid,startMs:(()=>{const read=readProcessStartV1(holder.pid);return typeof read==="number"?read:1;})(),originId:"s1",bootMs:Date.now()-Math.round(uptime()*1000),hb:true}));
  const old=(Date.now()-300_000)/1000;utimesSync(lock,old,old);
  expect((await statusV1(options)).daemon).toEqual({running:false,pid:null});
  const beat=(lagMs:number)=>JSON.stringify({version:1,pid:holder.pid,startMs:(()=>{const read=readProcessStartV1(holder.pid);return typeof read==="number"?read:1;})(),parent:1,startedMs:Date.now()-600_000,wallMs:Date.now(),phase:"run",base:worktree,source:"worktree",lagMs});
  put(worktree,".cache/f/boot/beat.json",beat(95_000));
  const busy=await statusV1(options);
  expect(busy.daemon).toEqual({running:true,pid:holder.pid,busySeconds:95});expect(busy.code).toEqual({source:"worktree",phase:"run"});
  await expect(openReplicaV1(options)).rejects.toBeInstanceOf(ReplicaBusyV1);
  const lines:string[]=[];
  await runReplicaCommandV1(["status","--worktree",worktree,"--store",store,"--lang","en"],{root:worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{lines.push(line);}});
  expect(lines.join("\n")).toContain("Daemon: running");expect(lines.join("\n")).toContain("busy, the event loop has not answered for 95 s");
  rmSync(join(worktree,".cache","f","boot","beat.json"));
  expect((await statusV1(options)).daemon.running).toBe(false);
  await closeReplicaV1(await openReplicaV1(options));
 }finally{holder.kill("SIGKILL");}
});

test("the self-check boots the daemon of the checked code in a scratch worktree, so a module that fits together passes and one that cannot load its own checkpoint fails even on a machine without state, and provisioning installs the boot stage",async()=>{
 const fresh=moduleCopy("smoke"),run=(where:{repo:string;script:string})=>Bun.spawnSync([process.execPath,where.script,"self-check","--lang","en"],{cwd:where.repo,env:cleanEnvironment(),stdout:"pipe",stderr:"pipe"});
 const good=run(fresh);
 expect(good.exitCode,good.stdout.toString()+good.stderr.toString()).toBe(0);
 sabotage(fresh.module);
 const bad=run(fresh);
 expect(bad.exitCode).toBe(1);expect(bad.stdout.toString()+bad.stderr.toString()).toContain("does not boot");
 const worktree=temp("seeded"),store=temp("store");
 put(worktree,"a.txt","alpha\n");
 const lines:string[]=[];
 const result=await runReplicaCommandV1(["seed","--worktree",worktree,"--store",store,"--group","g1","--origin","s2","--lang","en"],{root:worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{lines.push(line);}});
 expect(result.exitCode,lines.join("\n")).toBe(0);
 expect(readFileSync(join(worktree,".cache","f","boot","stage.ts"),"utf8")).toBe(readFileSync(realSupervisor,"utf8"));
});
