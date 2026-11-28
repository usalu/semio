import {afterEach,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
import {countPlatformPeersV1,createSidecarsV1,sidecarBackoffMillisecondsV1,wantedSidecarsV1,type SidecarChildV1,type SidecarSettingsV1} from "../🟦️.ts";
import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

const here=import.meta.dir,scratch:string[]=[],stoppers:(()=>Promise<void>)[]=[];
afterEach(async()=>{for(const stop of stoppers.splice(0))await stop();for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const until=async(condition:()=>boolean,milliseconds=10000,message="condition")=>{const end=Date.now()+milliseconds;while(Date.now()<end){if(condition())return;await sleep(15);}throw Error(`not reached: ${message}`);};

const child=(name:string,mode:string,extra:Partial<SidecarChildV1>={}):SidecarChildV1=>({name,script:"child.ts",roles:["member","server"],requires:"none",enabled:true,...extra,arguments:[mode,...(extra.arguments??[])]});
const settings=(children:readonly SidecarChildV1[],over:Partial<SidecarSettingsV1>={}):SidecarSettingsV1=>({enabled:true,startAfterSeconds:0,restart:{initialMilliseconds:60,maximumMilliseconds:240,stableMilliseconds:60_000},graceSeconds:1,children,...over});
function machine(){
 const worktree=mkdtempSync(join(tmpdir(),"sidecars-"));scratch.push(worktree);
 writeFileSync(join(worktree,"child.ts"),readFileSync(join(here,"🧬️child.ts")));
 return {worktree,storeRoot:join(worktree,"store"),originId:"g1-m03",groupId:"g1",platform:"macos" as const};
}
const run=(facts:ReturnType<typeof machine>,config:SidecarSettingsV1,extra:Partial<Parameters<typeof createSidecarsV1>[0]>={})=>{
 const supervisor=createSidecarsV1({settings:config,facts,peers:async()=>3,environment:{...process.env,SIDECAR_TEST_LOG:join(facts.worktree,"starts.log"),SIDECAR_TEST_ENV:join(facts.worktree,"env.json")},...extra});
 stoppers.push(()=>supervisor.stop());return supervisor;
};
const starts=(facts:ReturnType<typeof machine>):number[]=>existsSync(join(facts.worktree,"starts.log"))?readFileSync(join(facts.worktree,"starts.log"),"utf8").split("\n").filter(Boolean).map(line=>Number(line.split(" ")[1])):[];
const drive=(supervisor:ReturnType<typeof createSidecarsV1>,healthy:()=>boolean)=>{const timer=setInterval(()=>{supervisor.tick(healthy());},20);stoppers.push(async()=>{clearInterval(timer);});};

test("an independent python implementation agrees with the back-off schedule and with the choice of children, over a table of machines",()=>{
 const restart={initialMilliseconds:1000,maximumMilliseconds:60000,stableMilliseconds:60000},all:SidecarChildV1[]=[child("worker","stay",{requires:"platform-peer"}),child("scheduler","stay",{roles:["server"]}),child("off","stay",{enabled:false}),child("both","stay",{roles:["server","member"],requires:"platform-peer"})];
 const cases:unknown[]=[];
 for(let failures=1;failures<=12;failures++)cases.push({kind:"backoff",restart,failures});
 for(const enabled of [true,false])for(const server of [true,false])for(const peers of [0,3])for(const override of [undefined,"","off","worker","scheduler,both"])cases.push({kind:"wanted",settings:{enabled,children:all},server,peers,override});
 const oracle=spawnSync("python3",["-I",join(here,"🐍️oracle","🐍️.py")],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);
 const expected=JSON.parse(oracle.stdout) as unknown[];
 cases.forEach((value,index)=>{
  const row=value as {kind:string;failures:number;settings:{enabled:boolean;children:SidecarChildV1[]};server:boolean;peers:number;override?:string};
  const mine=row.kind==="backoff"?sidecarBackoffMillisecondsV1(restart,row.failures):wantedSidecarsV1(settings(row.settings.children,{enabled:row.settings.enabled}),row.server,row.peers,row.override).map(item=>item.name);
  expect<unknown>(mine,JSON.stringify(row)).toEqual(expected[index]);
 });
});

test("nothing starts before the daemon is healthy and the delay has passed; then every wanted child runs with the identity of the machine",async()=>{
 const facts=machine(),supervisor=run(facts,settings([child("a","stay")],{startAfterSeconds:1}));
 for(let round=0;round<3;round++){supervisor.tick(false);await sleep(1100);}
 expect(supervisor.report()[0]).toMatchObject({name:"a",state:"waiting",pid:null});expect(starts(facts)).toEqual([]);
 supervisor.tick(true);
 await until(()=>starts(facts).length===1,5000,"started once healthy");
 expect(supervisor.report()[0]).toMatchObject({state:"running"});expect(typeof supervisor.report()[0]!.pid).toBe("number");
 await until(()=>existsSync(join(facts.worktree,"env.json")),5000,"environment written");
 expect(JSON.parse(readFileSync(join(facts.worktree,"env.json"),"utf8"))).toEqual({origin:"g1-m03",group:"g1",store:facts.storeRoot,root:facts.worktree,name:"a",record:join(facts.worktree,".cache","f","sidecars","a.json"),lease:join(facts.worktree,".cache","f","sidecars","a.lease"),leaseMs:"7200000"});
 expect(JSON.parse(readFileSync(join(facts.worktree,".cache","f","sidecars",`a.${supervisor.report()[0]!.pid}.lease`),"utf8"))).toMatchObject({version:1,pid:process.pid});
 expect(existsSync(join(facts.worktree,".cache","f","sidecars","a.lease")),"the shared lease of older launchers is gone: they leave by themselves").toBe(false);
 expect(readFileSync(join(facts.worktree,".cache","f","sidecars","a.log"),"utf8")).toContain("child stay started");
});

test("a crashing child never takes the daemon down: it restarts with growing back-off, the failure is visible in the report, and a run longer than the stable time starts the count again",async()=>{
 const facts=machine(),config=settings([child("crashy","crash")],{restart:{initialMilliseconds:100,maximumMilliseconds:400,stableMilliseconds:60_000}}),supervisor=run(facts,config);
 drive(supervisor,()=>true);
 await until(()=>starts(facts).length>=5,15000,"five starts");
 const times=starts(facts),gaps=times.slice(1,5).map((time,index)=>time-times[index]!);
 expect(gaps[0]!,"first back-off").toBeGreaterThanOrEqual(120);expect(gaps[1]!,"second back-off doubles").toBeGreaterThanOrEqual(220);expect(gaps[2]!,"third is capped").toBeGreaterThanOrEqual(420);expect(gaps[3]!,"fourth stays capped").toBeLessThan(900);
 const report=supervisor.report()[0]!;expect(report.restarts).toBeGreaterThanOrEqual(4);expect(report.lastExit).toBe("exit 3");expect(["backoff","running"]).toContain(report.state);
 await supervisor.stop();
 const quick=machine(),stable=run(quick,settings([child("flaky","crash",{arguments:["300"]})],{restart:{initialMilliseconds:100,maximumMilliseconds:3200,stableMilliseconds:200}}));
 drive(stable,()=>true);
 await until(()=>starts(quick).length>=4,15000,"four starts of the flaky child");
 const gapsFlaky=starts(quick).slice(1,4).map((time,index)=>time-starts(quick)[index]!);
 for(const gap of gapsFlaky)expect(gap,"a run past the stable time resets the back-off to the first delay").toBeLessThan(900);
});

test("a child that cannot be started at all (the launcher throws) is a failed start with back-off, never an exception in the daemon",async()=>{
 const facts=machine();let calls=0;
 const supervisor=run(facts,settings([child("broken","stay")]),{launch:()=>{calls++;throw Error("spawn refused");}});
 drive(supervisor,()=>true);
 await until(()=>calls>=2,10000,"retried after back-off");
 expect(supervisor.report()[0]).toMatchObject({state:"backoff",pid:null});expect(supervisor.report()[0]!.lastExit).toContain("spawn refused");
});

test("stop ends every child: a well-behaved one on SIGTERM, a stubborn one killed after the grace",async()=>{
 const facts=machine(),supervisor=run(facts,settings([child("polite","stay"),child("stubborn","stubborn")]));
 supervisor.tick(true);await until(()=>starts(facts).length===2,10000,"both started");
 const pids=supervisor.report().map(row=>row.pid!),alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
 expect(pids.every(alive)).toBe(true);
 const began=Date.now();await supervisor.stop();
 expect(pids.some(alive)).toBe(false);expect(Date.now()-began).toBeGreaterThanOrEqual(900);expect(supervisor.report().map(row=>row.state)).toEqual(["stopped","stopped"]);
 supervisor.tick(true);await sleep(100);expect(supervisor.report().map(row=>row.state)).toEqual(["stopped","stopped"]);
});

test("a child that needs a platform peer starts when one exists, stops when none is left for three reads, and the group server alone runs a server-only child",async()=>{
 const facts=machine();let peers=0;
 const supervisor=run(facts,settings([child("worker","stay",{requires:"platform-peer"}),child("scheduler","stay",{roles:["server"]})]),{peers:async()=>peers,peersMilliseconds:30});
 drive(supervisor,()=>true);
 await sleep(300);expect(supervisor.report().map(row=>[row.name,row.state])).toEqual([["worker","off"],["scheduler","off"]]);
 peers=2;await until(()=>supervisor.report()[0]!.state==="running",10000,"worker starts with a peer");expect(supervisor.report()[1]!.state,"a member does not run the scheduler").toBe("off");
 peers=0;await until(()=>supervisor.report()[0]!.state==="off",10000,"worker stops when the peers are gone");
 const server={...machine(),originId:"g1-server"},head=run(server,settings([child("scheduler","stay",{roles:["server"]})]));
 drive(head,()=>true);await until(()=>head.report()[0]!.state==="running",10000,"the group server runs the scheduler");
});

test("the platform peers are counted from the fresh attestations of other origins of the same platform in the store, and a store that does not answer is unknown",async()=>{
 const store=mkdtempSync(join(tmpdir(),"peers-"));scratch.push(store);const now=Date.now();
 for(const [origin,platform,age] of [["me","macos",0],["a","macos",1000],["b","macos",700_000],["c","linux",1000],["d","macos",5000]] as const){mkdirSync(join(store,"v1","origins",origin),{recursive:true});writeFileSync(join(store,"v1","origins",origin,"attest.json"),JSON.stringify({platform,wallMs:now-age}));}
 mkdirSync(join(store,"v1","origins","e"),{recursive:true});writeFileSync(join(store,"v1","origins","e","attest.json"),"{broken");
 expect(await countPlatformPeersV1({storeRoot:store,originId:"me",platform:"macos"},now)).toBe(2);
 expect(await countPlatformPeersV1({storeRoot:join(store,"nowhere"),originId:"me",platform:"macos"},now)).toBeUndefined();
});

test("disabled by the topology or by SEMIO_FLEET_SIDECARS=off nothing ever starts",async()=>{
 const facts=machine(),off=run(facts,settings([child("a","stay")],{enabled:false}));off.tick(true);await sleep(200);expect(off.report()[0]!.state).toBe("off");
 const other=machine(),env=run(other,settings([child("a","stay")]),{environment:{...process.env,SEMIO_FLEET_SIDECARS:"off"}});env.tick(true);await sleep(200);expect(env.report()[0]!.state).toBe("off");expect(starts(facts)).toEqual([]);void spawn;
});

test("real processes: a daemon whose loop is blocked leaves its sidecar running and restartable by its own launcher, a killed daemon leaves the launcher alone, and the next daemon adopts the same launcher instead of starting another",async()=>{
 const worktree=mkdtempSync(join(tmpdir(),"sidecar-block-"));scratch.push(worktree);
 const launcher=relative(worktree,join(here,"🧬️launcher.ts")),env={...process.env,SIDECAR_TEST_DIR:worktree,SIDECAR_TEST_LEASE_MS:"6000"} as Record<string,string>;
 const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}};
 const lines=():string[]=>existsSync(join(worktree,"starts.log"))?readFileSync(join(worktree,"starts.log"),"utf8").split("\n").filter(Boolean):[];
 const reported=(pid:number):{pid:number|null;state:string;restarts:number}|undefined=>{try{return (JSON.parse(readFileSync(join(worktree,`report-${pid}.json`),"utf8")) as {pid:number|null;state:string;restarts:number}[])[0];}catch{return undefined;}};
 const daemon=Bun.spawn([process.execPath,join(here,"🧬️daemon.ts"),worktree,"block",launcher],{env,stdout:"inherit",stderr:"inherit"});
 stoppers.push(async()=>{try{daemon.kill("SIGKILL");}catch{}});
 await until(()=>existsSync(join(worktree,"daemon-state"))&&readFileSync(join(worktree,"daemon-state"),"utf8")==="blocked",20000,"daemon blocked");
 await until(()=>lines().length>=2,20000,"the launcher restarted its child while the daemon loop was blocked");
 const second=Number(lines()[1]!.split(" ")[1]),launcherPid=reported(daemon.pid)!.pid!;
 expect(alive(second)).toBe(true);expect(alive(daemon.pid)).toBe(true);expect(alive(launcherPid)).toBe(true);
 daemon.kill("SIGKILL");await daemon.exited;
 await sleep(3000);
 expect(alive(second),"the sidecar does not end with its dead daemon").toBe(true);expect(alive(launcherPid)).toBe(true);
 const next=Bun.spawn([process.execPath,join(here,"🧬️daemon.ts"),worktree,"run",launcher],{env,stdout:"inherit",stderr:"inherit"});
 stoppers.push(async()=>{try{next.kill("SIGKILL");}catch{}});
 await until(()=>reported(next.pid)?.state==="running",20000,"the next daemon reports the sidecar");
 expect(reported(next.pid)).toMatchObject({pid:launcherPid,state:"running",restarts:0});
 await sleep(1500);
 expect(lines().length,"no second start").toBe(2);expect(alive(second)).toBe(true);
 next.kill("SIGKILL");await next.exited;
 await until(()=>!alive(second)&&!alive(launcherPid),30000,"nobody renews the lease: the sidecar ends itself");
});

function adoption(){
 const worktree=mkdtempSync(join(tmpdir(),"sidecar-adopt-"));scratch.push(worktree);
 const source=readFileSync(join(here,"🧬️launcher.ts"),"utf8").replace('"../../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts"',JSON.stringify(join(here,"..","..","..","..","📚️library","🏗️build","🌐️remote-execution","🛰️sidecar","🟦️.ts")));
 writeFileSync(join(worktree,"launcher.ts"),source);
 const env={...process.env,SIDECAR_TEST_DIR:worktree},facts={worktree,storeRoot:join(worktree,"store"),originId:"g1-m03",groupId:"g1",platform:"macos" as const};
 const lines=():string[]=>existsSync(join(worktree,"starts.log"))?readFileSync(join(worktree,"starts.log"),"utf8").split("\n").filter(Boolean):[];
 const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}};
 const daemon=(lease?:number)=>{
  const supervisor=createSidecarsV1({settings:settings([child("role","role",{script:"launcher.ts",arguments:["role"]})]),facts,peers:async()=>3,environment:env,protocol:lease===undefined?{}:{leaseMilliseconds:lease}});
  stoppers.push(()=>supervisor.stop());
  const timer=setInterval(()=>{supervisor.tick(true);},20);
  stoppers.push(async()=>{clearInterval(timer);});
  return {supervisor,quiet:()=>{clearInterval(timer);}};
 };
 return {worktree,facts,lines,alive,daemon,entry:join(worktree,"launcher.ts"),record:join(worktree,".cache","f","sidecars","role.json")};
}

test("a restarted daemon adopts a living sidecar of unchanged code: same process, no new start, and its stop ends it",async()=>{
 const box=adoption(),first=box.daemon();
 await until(()=>first.supervisor.report()[0]!.state==="running"&&box.lines().length>=2,30000,"the sidecar runs");
 const pid=first.supervisor.report()[0]!.pid!,starts=box.lines().length;
 first.quiet();first.supervisor.release();
 await sleep(2500);
 expect(box.alive(pid),"released, not ended").toBe(true);
 const second=box.daemon();
 await until(()=>second.supervisor.report()[0]!.state==="running",10000,"adopted");
 expect(second.supervisor.report()[0]).toMatchObject({pid,state:"running",restarts:0});
 await sleep(1500);
 expect(box.lines().length,"no new start").toBe(starts);expect(box.alive(pid)).toBe(true);
 await second.supervisor.stop();
 expect(box.alive(pid),"an explicit stop ends an adopted sidecar").toBe(false);
 expect(existsSync(box.record),"the launcher removes its record").toBe(false);
});

test("a sidecar whose entry script changed is ended and started again by the next daemon, and one whose process died is started again at once",async()=>{
 const box=adoption(),first=box.daemon();
 await until(()=>first.supervisor.report()[0]!.state==="running"&&box.lines().length>=2,30000,"the sidecar runs");
 const old=first.supervisor.report()[0]!.pid!;
 first.quiet();first.supervisor.release();
 writeFileSync(box.entry,`${readFileSync(box.entry,"utf8")}\n// changed\n`);
 const second=box.daemon();
 await until(()=>{const row=second.supervisor.report()[0]!;return row.state==="running"&&row.pid!==old&&row.pid!==null;},30000,"started again with the new entry");
 expect(box.alive(old),"the old launcher ended").toBe(false);
 const fresh=second.supervisor.report()[0]!.pid!;
 second.quiet();second.supervisor.release();
 process.kill(fresh,"SIGKILL");
 await until(()=>!box.alive(fresh),10000,"killed");
 const third=box.daemon();
 await until(()=>{const row=third.supervisor.report()[0]!;return row.state==="running"&&row.pid!==fresh&&row.pid!==null;},30000,"a dead process is not adopted");
 await third.supervisor.stop();
});

test("a record that nobody refreshes is not adopted even when its process id lives, and a launcher whose lease nobody renews ends itself",async()=>{
 const box=adoption();
 mkdirSync(join(box.worktree,".cache","f","sidecars"),{recursive:true});
 const hash=createHash("sha256").update(readFileSync(box.entry)).digest("hex");
 writeFileSync(box.record,JSON.stringify({version:1,name:"role",...ownProcessIdentityV1(),startedMs:Date.now()-60000,wallMs:Date.now()-60000,entrySha256:hash}));
 const first=box.daemon(2500);
 await until(()=>first.supervisor.report()[0]!.state==="running"&&box.lines().length>=2,30000,"a fresh sidecar started");
 const pid=first.supervisor.report()[0]!.pid!;
 expect(pid).not.toBe(process.pid);
 first.quiet();first.supervisor.release();
 await until(()=>!box.alive(pid),30000,"the launcher ended itself without a lease");
});

test("a launcher that a newer live launcher of the same sidecar superseded leaves, one that is newer takes the record",async()=>{
 const box=adoption();
 mkdirSync(join(box.worktree,".cache","f","sidecars"),{recursive:true});
 const hash=createHash("sha256").update(readFileSync(box.entry)).digest("hex"),environment={...process.env,SIDECAR_TEST_DIR:box.worktree,SEMIO_FLEET_SIDECAR:"role",SEMIO_FLEET_SIDECAR_RECORD:box.record};
 writeFileSync(box.record,JSON.stringify({version:1,name:"role",...ownProcessIdentityV1(),startedMs:Date.now()+600000,wallMs:Date.now(),entrySha256:hash}));
 const older=Bun.spawn([process.execPath,box.entry,"role"],{env:environment,stdout:"ignore",stderr:"pipe"});stoppers.push(async()=>{try{older.kill("SIGKILL");}catch{}});
 expect(await Promise.race([older.exited,sleep(20000).then(()=>"alive")]),"the older launcher leaves at once").toBe(0);
 expect(await new Response(older.stderr).text()).toContain("superseded by launcher");
 expect(JSON.parse(readFileSync(box.record,"utf8")).pid,"the record of the newer launcher is untouched").toBe(process.pid);
 writeFileSync(box.record,JSON.stringify({version:1,name:"role",...ownProcessIdentityV1(),startedMs:Date.now()-600000,wallMs:Date.now(),entrySha256:hash}));
 const newer=Bun.spawn([process.execPath,box.entry,"role"],{env:environment,stdout:"ignore",stderr:"ignore"});stoppers.push(async()=>{try{newer.kill("SIGKILL");}catch{}});
 await until(()=>{try{return JSON.parse(readFileSync(box.record,"utf8")).pid===newer.pid;}catch{return false;}},20000,"the newer launcher took over the record");
 expect(newer.exitCode).toBeNull();
});
