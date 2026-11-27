/** 🔁️ Contract: a sidecar restarts only for a byte of the files it loads, only after its closure was quiet (or at the latest after the maximum deferral), drains its work first, and says which files changed and how long the restart was deferred. */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {appendFileSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createSidecarClosureWatchV1,createSidecarRestartDebounceV1} from "../../🧮️closure/🟦️.ts";
import {superviseGraphChildV1} from "../../👷️agent/🟦️.ts";
import {createSidecarRestartV1,sidecarRestartPolicyForV1,sidecarRestartPolicyV1,sidecarRestartTextV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import type {SidecarWorkV1} from "../../🧰️shared/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(root:string,name:string,text:string)=>{const path=join(root,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const tree=()=>{
 const root=mkdtempSync(join(tmpdir(),"sidecar-restart-"));scratch.push(root);
 write(root,"main.ts",'import {a} from "./a.ts";\nexport const run=a;\n');
 const inside=write(root,"a.ts","export const a=1;\n"),outside=write(root,"outside.ts","export const b=1;\n");
 return{root,inside,outside,watch:createSidecarClosureWatchV1([join(root,"main.ts")],[],root)};
};
const policy={quietMilliseconds:90_000,maximumDeferralMilliseconds:600_000,graceMilliseconds:300_000,serveDeferralMilliseconds:3_600_000,minimumIntervalMilliseconds:0};
const SECOND=1000;
const rig=(role:"agent"|"cache"|"graph"|"ready"|"pool",watch:ReturnType<typeof tree>["watch"],overrides:Partial<typeof policy>={},work?:{busy:boolean;restartPending:boolean;wake?:()=>void})=>{
 let clock=1_000_000;const lines:string[]=[],events:string[]=[],timers:{ms:number;run:()=>void}[]=[];
 const restart=createSidecarRestartV1({role,watch,...work===undefined?{}:{work},policy:{...policy,...overrides},begin:()=>events.push(`begin@${clock}`),drain:()=>events.push(`drain@${clock}`),abort:()=>events.push(`abort@${clock}`),write:line=>lines.push(line),now:()=>clock,after:(ms,run)=>{timers.push({ms,run});}});
 return{lines,events,timers,restart,at:()=>clock,advance:(ms:number)=>{clock+=ms;restart.tick();}};
};

test("the numbers come from the schema: 5 min quiet, 30 min at most, 5 min grace, a 30 s look",()=>{
 expect(sidecarRestartPolicyV1).toEqual(schema.definitions.Restart.const);
 expect(sidecarRestartPolicyV1.quietMilliseconds).toBe(300_000);expect(sidecarRestartPolicyV1.maximumDeferralMilliseconds).toBe(1_800_000);expect(sidecarRestartPolicyV1.minimumIntervalMilliseconds).toBe(1_800_000);expect(sidecarRestartPolicyV1.graceMilliseconds).toBe(300_000);expect(sidecarRestartPolicyV1.watchMilliseconds).toBe(30_000);expect(sidecarRestartPolicyV1.serveDeferralMilliseconds).toBe(28_800_000);
});

test("an edit outside the closure never restarts, however often it happens",()=>{
 const world=tree(),test=rig("agent",world.watch);
 for(let tick=0;tick<40;tick++){appendFileSync(world.outside,`// ${tick}\n`);test.advance(30*SECOND);}
 expect(test.events).toEqual([]);expect(test.lines).toEqual([]);
});

test("ten edits in 60 s inside the closure restart once, 90 s after the last one, with the changed file and the deferral in the log",()=>{
 const world=tree(),test=rig("cache",world.watch);
 for(let edit=0;edit<10;edit++){appendFileSync(world.inside,`// ${edit}\n`);test.advance(6*SECOND);}
 const lastEdit=test.at();
 expect(test.events,"nothing yet").toEqual([]);
 while(test.events.length===0&&test.at()-lastEdit<600*SECOND)test.advance(6*SECOND);
 expect(test.events.length).toBe(2);
 const due=Number(test.events[0]!.replace("begin@",""));
 expect(due-lastEdit,"after the quiet period").toBeGreaterThanOrEqual(90*SECOND);expect(due-lastEdit).toBeLessThan(96*SECOND);
 expect(test.events[1]).toBe(`abort@${due}`);
 for(let more=0;more<30;more++)test.advance(30*SECOND);
 expect(test.events.length,"one restart for ten edits").toBe(2);
 expect(test.lines.length).toBe(2);
 expect(test.lines[0]).toContain("a.ts");expect(test.lines[0]).toContain("edited");
 expect(test.lines[1]).toContain("cache code changed, restarting");expect(test.lines[1]).toContain("a.ts");expect(test.lines[1]).toMatch(/deferred 14[0-9] s/u);expect(test.lines[1]).toContain("quiet period over");expect(test.lines[1]).toContain("Neustart");
});

test("a closure that is edited without a pause restarts after the maximum deferral and says so",()=>{
 const world=tree(),test=rig("cache",world.watch);
 for(let tick=0;tick<120&&test.events.length===0;tick++){appendFileSync(world.inside,`// ${tick}\n`);test.advance(10*SECOND);}
 expect(test.events.length).toBe(2);
 const waited=Number(test.events[0]!.replace("begin@",""))-1_000_000;
 expect(waited).toBeGreaterThanOrEqual(600*SECOND);expect(waited).toBeLessThan(620*SECOND);
 expect(test.lines.at(-1)).toContain("maximum deferral reached");
});

test("an edit that is reverted before the quiet period is over cancels the restart, and a later edit begins a new wait",()=>{
 const world=tree(),test=rig("cache",world.watch);
 appendFileSync(world.inside,"// x\n");test.advance(30*SECOND);
 writeFileSync(world.inside,"export const a=1;\n");test.advance(30*SECOND);
 for(let tick=0;tick<20;tick++)test.advance(30*SECOND);
 expect(test.events).toEqual([]);
 appendFileSync(world.inside,"// y\n");
 for(let tick=0;tick<4;tick++)test.advance(30*SECOND);
 expect(test.events.map(event=>event.split("@")[0])).toEqual(["begin","abort"]);
});

test("the agent drains: it stops taking work at once, aborts only when the grace is over, and never aborts early on its own",()=>{
 const world=tree(),test=rig("agent",world.watch);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<8&&test.events.length===0;tick++)test.advance(30*SECOND);
 expect(test.events.map(event=>event.split("@")[0])).toEqual(["begin","drain"]);
 expect(test.timers.length).toBe(1);expect(test.timers[0]!.ms).toBe(300_000);
 expect(test.lines.some(line=>line.includes("finishing the work in flight for up to 300 s")&&line.includes("beende die laufende Arbeit"))).toBe(true);
 test.timers[0]!.run();
 expect(test.events.map(event=>event.split("@")[0])).toEqual(["begin","drain","abort"]);
});

const MINUTE=60*SECOND;
test("a graph server that serves is not restarted by edits every 5 minutes for 40 minutes: 0 restarts until the serve is done, then exactly 1, at once, and the deferral is told once",()=>{
 const world=tree(),work={busy:true,restartPending:false} as {busy:boolean;restartPending:boolean;wake?:()=>void},test=rig("graph",world.watch,{},work);
 for(let tick=0;tick<80;tick++){if(tick%10===0)appendFileSync(world.inside,`// ${tick}\n`);test.advance(30*SECOND);}
 expect(test.at()-1_000_000).toBe(40*MINUTE);
 expect(test.events,"no restart during the serve").toEqual([]);
 expect(work.restartPending).toBe(true);
 expect(test.lines.length,"one line, not one per look").toBe(1);
 expect(test.lines[0]).toContain("graph: code changed (");expect(test.lines[0]).toContain("a.ts");expect(test.lines[0]).toContain("restarting when the serve in flight is done (at most 60 min)");expect(test.lines[0]).toContain("Serve");
 const before=test.at();work.busy=false;work.wake?.();
 expect(test.events,"restarts the moment the serve is done, no further wait for a tick").toEqual([`begin@${before}`,`abort@${before}`]);
 for(let more=0;more<20;more++)test.advance(30*SECOND);
 expect(test.events.length,"exactly one restart").toBe(2);
 expect(test.lines.length).toBe(2);expect(test.lines[1]).toContain("graph code changed, restarting");expect(test.lines[1]).toContain("serve in flight is done");expect(test.lines[1]).toMatch(/deferred 23[0-9][0-9] s/u);
});

test("a graph server that is busy for ever restarts after the maximum deferral of the serve, counted from the first edit, and says so",()=>{
 const world=tree(),work={busy:true,restartPending:false},test=rig("graph",world.watch,{},work);
 for(let tick=0;tick<400&&test.events.length===0;tick++){if(tick%10===0)appendFileSync(world.inside,`// ${tick}\n`);test.advance(30*SECOND);}
 expect(test.events.length).toBe(2);
 const waited=Number(test.events[0]!.replace("begin@",""))-1_000_000;
 expect(waited).toBeGreaterThanOrEqual(60*MINUTE);expect(waited).toBeLessThan(61*MINUTE);
 expect(test.lines.at(-1)).toContain("maximum deferral of the serve reached");
});

test("the maximum deferral of the serve is a value of the policy",()=>{
 const world=tree(),work={busy:true,restartPending:false},test=rig("graph",world.watch,{serveDeferralMilliseconds:20*MINUTE},work);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<200&&test.events.length===0;tick++)test.advance(30*SECOND);
 const waited=Number(test.events[0]!.replace("begin@",""))-1_000_000;
 expect(waited).toBeGreaterThanOrEqual(20*MINUTE);expect(waited).toBeLessThan(21*MINUTE);expect(test.lines[0]).toContain("at most 20 min");
});

test("a graph server that is idle restarts by the ordinary rule, and goes on to wait when a serve starts before the quiet period is over",()=>{
 const world=tree(),work={busy:false,restartPending:false},idle=rig("graph",world.watch,{},work);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<8&&idle.events.length===0;tick++)idle.advance(30*SECOND);
 expect(idle.events.map(event=>event.split("@")[0])).toEqual(["begin","abort"]);expect(idle.lines[0]).toContain("restarting after 90 s");
 const other=tree(),busy={busy:false,restartPending:false},late=rig("graph",other.watch,{},busy);
 appendFileSync(other.inside,"// x\n");late.advance(30*SECOND);busy.busy=true;
 for(let tick=0;tick<20;tick++)late.advance(30*SECOND);
 expect(late.events).toEqual([]);expect(busy.restartPending).toBe(true);expect(late.lines.at(-1)).toContain("restarting when the serve in flight is done");
});

test("an edit that is reverted while the serve runs cancels the pending restart and lifts the block on new reads",()=>{
 const world=tree(),work:SidecarWorkV1={busy:true,restartPending:false},test=rig("graph",world.watch,{},work);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<10;tick++)test.advance(30*SECOND);
 expect(work.restartPending).toBe(true);
 writeFileSync(world.inside,"export const a=1;\n");test.advance(30*SECOND);
 expect(work.restartPending).toBe(false);work.busy=false;work.wake?.();test.advance(30*SECOND);
 expect(test.events).toEqual([]);
});

test("the agent keeps the 90 s / 600 s rule whatever work is reported",()=>{
 const world=tree(),work={busy:true,restartPending:false},test=rig("agent",world.watch,{},work);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<8&&test.events.length===0;tick++)test.advance(30*SECOND);
 expect(test.events.map(event=>event.split("@")[0])).toEqual(["begin","drain"]);
 expect(work.restartPending).toBe(false);expect(test.lines[0]).toContain("restarting after 90 s");
});

test("every text exists in both languages with the same placeholders",()=>{
 const en=schema.definitions.Texts.const.en as Record<string,string>,de=schema.definitions.Texts.const.de as Record<string,string>;
 expect(Object.keys(en).sort()).toEqual(Object.keys(de).sort());
 for(const key of Object.keys(en))expect([...en[key]!.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort(),key).toEqual([...de[key]!.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort());
 expect(sidecarRestartTextV1("deferred",{role:"agent",files:"a.ts",quiet:90,maximum:600})).toContain("restarting after 90 s");
});

test("the debounce agrees with an independent implementation on 300 random edit histories",async()=>{
 let seed=11;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:300},()=>({quiet:Math.floor(next()*5+1)*10,maximum:Math.floor(next()*20+6)*10,ticks:Array.from({length:60},()=>Math.floor(next()*4))}));
 const python=Bun.spawnSync(["python3","-I","-c","import json,sys\nfor c in json.load(sys.stdin):\n first=None;last=0;seen=None;out=-1\n for i,d in enumerate(c['ticks']):\n  t=(i+1)*10\n  if d==0:\n   first=None;seen=None;continue\n  if first is None:first=t\n  if d!=seen:\n   seen=d;last=t\n  if t-last>=c['quiet'] or t-first>=c['maximum']:\n   out=i;break\n print(out)"],{stdin:new TextEncoder().encode(JSON.stringify(cases)),stdout:"pipe",stderr:"pipe"});
 expect(python.exitCode,python.stderr.toString()).toBe(0);
 const oracle=python.stdout.toString().trim().split("\n").map(Number);
 cases.forEach((row,index)=>{
  let clock=0;const debounce=createSidecarRestartDebounceV1({quietMilliseconds:row.quiet*SECOND,maximumDeferralMilliseconds:row.maximum*SECOND},()=>clock);let due=-1;
  for(let tick=0;tick<row.ticks.length;tick++){clock=(tick+1)*10*SECOND;if(debounce.observe(String(row.ticks[tick]),"0").state==="due"){due=tick;break;}}
  expect(due,JSON.stringify(row)).toBe(oracle[index]!);
 });
});

test("the agent runs the graph server as a child of its own: new code (75) starts it again at once, a crash after a back-off, the agent is never touched, a stop ends it with SIGTERM so that it saves its state, and SEMIO_GRAPH_SERVER=off starts nothing",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"sidecar-graph-child-"));scratch.push(directory);
 const log=join(directory,"starts.log"),entry=join(directory,"entry.ts");
 writeFileSync(entry,`import {appendFileSync,readFileSync} from "node:fs";\nappendFileSync(${JSON.stringify(log)},process.argv[2]+"\\n");\nconst starts=readFileSync(${JSON.stringify(log)},"utf8").split("\\n").filter(line=>line==="graph").length;\nif(starts===1)process.exit(75);\nif(starts===2)process.exit(1);\nprocess.on("SIGTERM",()=>{appendFileSync(${JSON.stringify(log)},"term\\n");process.exit(0);});setInterval(()=>{},1000);\n`);
 const lines:string[]=[],controller=new AbortController(),context={root:directory,environment:{...process.env} as Record<string,string>,signal:controller.signal,write:(line:string)=>lines.push(line)};
 const starts=()=>{try{return readFileSync(log,"utf8").split("\n").filter(line=>line==="graph").length;}catch{return 0;}};
 const child=superviseGraphChildV1(context,entry);
 const deadline=Date.now()+20000;while(Date.now()<deadline&&starts()<3)await Bun.sleep(50);
 expect(starts(),"started, restarted for new code, restarted after a crash").toBe(3);
 expect(lines,"status 75 is no failure: only the crash is told").toEqual(["graph ended with status 1, starting again in 1 s"]);
 await Bun.sleep(300);expect(starts()).toBe(3);
 await child.stop();
 expect(readFileSync(log,"utf8").split("\n").filter(line=>line==="term").length,"asked to end with SIGTERM").toBe(1);
 await Bun.sleep(1500);expect(starts(),"nothing starts after the stop").toBe(3);
 rmSync(log,{force:true});
 const off=superviseGraphChildV1({...context,signal:new AbortController().signal,environment:{...context.environment,SEMIO_GRAPH_SERVER:"off"}},entry);
 await Bun.sleep(300);await off.stop();expect(starts(),"switched off").toBe(0);
},60000);

test("a ready role restarts for new code even in the middle of a stage, because the next child adopts the detached stage: it takes no new stage while the restart waits, restarts when the quiet period is over and never waits for the stage",()=>{
 expect(sidecarRestartPolicyForV1("ready").serveDeferralMilliseconds).toBe(0);expect(sidecarRestartPolicyForV1("graph").serveDeferralMilliseconds).toBe(28_800_000);expect(sidecarRestartPolicyForV1("pool").serveDeferralMilliseconds).toBe(600_000);
 expect(sidecarRestartPolicyForV1("ready").quietMilliseconds).toBe(90_000);
 const world=tree(),work={busy:true,restartPending:false} as {busy:boolean;restartPending:boolean;wake?:()=>void},test=rig("ready",world.watch,{serveDeferralMilliseconds:sidecarRestartPolicyForV1("ready").serveDeferralMilliseconds},work);
 appendFileSync(world.inside,"// edit\n");test.advance(30*SECOND);
 expect(test.events).toEqual([]);expect(work.restartPending).toBe(true);expect(test.lines.length).toBe(1);expect(test.lines[0]).toContain("ready:");expect(test.lines[0]).not.toContain("serve in flight");
 for(let tick=0;tick<4;tick++)test.advance(30*SECOND);
 expect(test.events.map(event=>event.split("@")[0])).toEqual(["begin","abort"]);expect(work.busy).toBe(true);expect(test.lines[test.lines.length-1]).toContain("quiet period over");
});

test("a graph role in the middle of a serve still waits for it, up to the serve deferral",()=>{
 const world=tree(),work={busy:true,restartPending:false} as {busy:boolean;restartPending:boolean;wake?:()=>void},test=rig("graph",world.watch,{},work);
 for(let tick=0;tick<80;tick++){if(tick%10===0)appendFileSync(world.inside,`// ${tick}\n`);test.advance(30*SECOND);}
 expect(test.events).toEqual([]);expect(work.restartPending).toBe(true);expect(test.lines[0]).toContain("graph: code changed (");
 const before=test.at();work.busy=false;work.wake?.();
 expect(test.events).toEqual([`begin@${before}`,`abort@${before}`]);
});

test("the pool role never stops taking work for a pending restart, waits 90 s of quiet and hands over at most once in 10 minutes, however often its closure is edited",()=>{
 expect(sidecarRestartPolicyForV1("pool")).toMatchObject({quietMilliseconds:90_000,maximumDeferralMilliseconds:600_000,minimumIntervalMilliseconds:600_000});
 const world=tree(),work={busy:false,restartPending:false},test=rig("pool",world.watch,sidecarRestartPolicyForV1("pool"),work);
 appendFileSync(world.inside,"// 1\n");
 for(let tick=0;tick<30;tick++){appendFileSync(world.inside,`// ${tick}\n`);test.advance(6*SECOND);expect(work.restartPending,"the pool keeps taking owners").toBe(false);}
 expect(test.events,"edited every 6 s for 3 min: nothing yet (and not before 10 min of life)").toEqual([]);
 for(let tick=0;tick<80&&test.events.length===0;tick++)test.advance(6*SECOND);
 expect(test.events.length,"one handover after the quiet period once the process is 10 minutes old").toBe(2);
 expect(Number(test.events[0]!.replace("begin@",""))-1_000_000,"not before 10 minutes of life").toBeGreaterThanOrEqual(600*SECOND);
});

const drainRun=(row:(typeof fixture.drain)[number]):{abortAt:number;lines:string[];events:string[]}=>{
 const world=tree(),work={busy:true,running:2,restartPending:false},test=rig("agent",world.watch,{graceMilliseconds:row.graceMs,drainMaximumMilliseconds:row.drainMaxMs,drainPollMilliseconds:row.pollMs} as never,work);
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<8&&test.events.length===0;tick++)test.advance(30*SECOND);
 let elapsed=0,abortAt=-1;
 for(let guard=0;guard<100000&&abortAt<0;guard++){
  const timer=test.timers.shift()!;
  elapsed+=timer.ms;test.advance(timer.ms);work.busy=elapsed<row.busyUntilMs;timer.run();
  if(test.events.some(event=>event.startsWith("abort@")))abortAt=elapsed;
 }
 return{abortAt,lines:test.lines,events:test.events};
};

test("an agent that serves actions for other agents is drained, never aborted: it waits past the grace until they end by their own deadline, bounded only by the drain maximum, and says so once in both languages",()=>{
 const results=fixture.drain.map(row=>drainRun(row));
 expect(results.map(row=>row.abortAt)).toEqual(fixture.drain.map(row=>row.abortAtMs));
 const waited=results[1]!;
 expect(waited.lines.filter(line=>line.includes("still running after the grace")).length,"told once").toBe(1);
 expect(waited.lines.some(line=>line.includes("never aborted")&&line.includes("nie abgebrochen")&&line.includes("2 remote action(s)"))).toBe(true);
 expect(waited.events.filter(event=>event.startsWith("abort@")).length).toBe(1);
 const bounded=results[3]!;expect(bounded.lines.some(line=>line.includes("the bound of the drain")&&line.includes("Grenze des Auslaufens"))).toBe(true);
 expect(results[0]!.lines.some(line=>line.includes("never aborted"))).toBe(false);
 expect(sidecarRestartPolicyForV1("agent").drainMaximumMilliseconds).toBe(7_200_000);
});

test("the Python oracle derives the same abort times of the drain",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("./🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(fixture.drain.map(row=>drainRun(row).abortAt));
});

test("a cache server restarts for a code change although requests are in flight (its stop drains them and the successor takes the new connections), and a request that never ends cannot hold the restart back",()=>{
 const world=tree(),work={busy:true,restartPending:false} as {busy:boolean;restartPending:boolean;wake?:()=>void},test=rig("cache",world.watch,sidecarRestartPolicyForV1("cache"),work);
 for(let tick=0;tick<400;tick++){if(tick%10===0)appendFileSync(world.inside,`// ${tick}\n`);test.advance(30*SECOND);}
 expect(test.events.length,"restarted once, in flight or not").toBe(2);expect(test.events[0]).toMatch(/^begin@/u);expect(test.events[1]).toMatch(/^abort@/u);
});

test("with the declared policy ten landings, two minutes apart, cause one restart of the cache role and a long graph read is never cancelled by them",()=>{
 const real=sidecarRestartPolicyV1,cache=tree(),run=rig("cache",cache.watch,real);
 for(let landing=0;landing<10;landing++){appendFileSync(cache.inside,`// ${landing}\n`);run.advance(120*SECOND);}
 expect(run.events,"inside the burst nothing restarts").toEqual([]);
 while(run.events.length===0)run.advance(30*SECOND);
 expect(run.events.length).toBe(2);
 const graph=tree(),work={busy:true,restartPending:false},busy=rig("graph",graph.watch,real,work);
 appendFileSync(graph.inside,"// x\n");
 for(let tick=0;tick<2*60*60/30;tick++)busy.advance(30*SECOND);
 expect(busy.events,"two hours of a busy read: no restart").toEqual([]);
 expect(work.restartPending).toBe(true);
});

test("the cache server, whose restart is a hand-over, restarts for landings at most every three hours however often its closure is edited",()=>{
 const cache=sidecarRestartPolicyForV1("cache");
 expect(cache).toMatchObject({quietMilliseconds:900_000,maximumDeferralMilliseconds:10_800_000,minimumIntervalMilliseconds:10_800_000,serveDeferralMilliseconds:0});
 const world=tree(),run=rig("cache",world.watch,cache);
 let restartedAt:number|undefined;
 for(let tick=0;tick<24*60*2&&restartedAt===undefined;tick++){if(tick%20===0)appendFileSync(world.inside,`// ${tick}\n`);run.advance(30*SECOND);if(run.events.length>0)restartedAt=run.at();}
 expect(restartedAt,"a closure edited every ten minutes still restarts").toBeDefined();
 expect(restartedAt!-1_000_000,"not before the minimum interval").toBeGreaterThanOrEqual(10_800_000);
 expect(restartedAt!-1_000_000,"and within one look of it").toBeLessThanOrEqual(10_800_000+30*SECOND);
 const quiet=tree(),calm=rig("cache",quiet.watch,cache);
 appendFileSync(quiet.inside,"// x\n");
 while(calm.events.length===0)calm.advance(30*SECOND);
 expect(calm.at()-1_000_000,"a single landing restarts when the closure was quiet and the interval is over").toBeGreaterThanOrEqual(10_800_000);
});

test("a role that reports progress is not restarted while the progress advances, whatever the clock says, and restarts when it stalls",()=>{
 const world=tree(),work:{busy:boolean;restartPending:boolean;progressAtMs?:()=>number|null}={busy:true,restartPending:false},run=rig("graph",world.watch,sidecarRestartPolicyV1,work);
 work.progressAtMs=()=>run.at()-60*SECOND;
 appendFileSync(world.inside,"// x\n");
 for(let tick=0;tick<24*60*60/30;tick++)run.advance(30*SECOND);
 expect(run.events,"a day of advancing progress: no restart").toEqual([]);
 const stalledAt=run.at();work.progressAtMs=()=>stalledAt;
 while(run.events.length===0&&run.at()-stalledAt<3600*SECOND)run.advance(30*SECOND);
 expect(run.events.length).toBe(2);expect(run.at()-stalledAt).toBeLessThan(11*60*SECOND);
});
