import{afterAll,expect,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{describeSliceReadStepV1,sliceReadStepV1}from"../../../../../🧠️memory/📋️slice-report/🟨️.mjs";
import residentSchema from"../../../../../🏡️resident/🧬️schema/🔣️.json"with{type:"json"};
import slicePolicy from"../../../../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json"with{type:"json"};
import{readyKillRecordLineV1}from"../../🌲️process-tree/🧹️containment/🟦️.ts";
import{READY_TREE_POLICY_V1 as TREE,readyIdentityV1,readyReadTableV1,readyScaledTreePolicyV1,readyStepOfV1,readyStepOverdueSecondsV1,readyStepWatchV1}from"../../🌲️process-tree/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{READY_STAGES_V1 as STAGES,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyEndingV1,readyRefusalV1,runReadyStageV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyStatusV1}from"../../🟦️.ts";

/** 🪜️ A step that declares its bound and runs far past it with nothing else moving is stuck, not waiting: the supervisor ends the stage with the typed reason `step-overdue` and starts it again on current code. Every line of the producer (the slice worker's steps in the resident texts) is read by the consumer (the supervisor), a vanished stage process says how it vanished, and a slow process table no longer turns the stage wrapper into "another process". */
import inventoryHeartbeat from "./🔣️.json" with {type:"json"};

const scratch:string[]=[],alive:number[]=[];afterAll(()=>{for(const pid of alive)try{process.kill(pid,"SIGKILL");}catch{}for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`overdue-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string,timeoutMilliseconds=600000)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds});
const messages=residentSchema.definitions.Policy.const.messages as Record<"en"|"de",Record<string,string>>;
const fill=(text:string,fields:Record<string,string|number>)=>text.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));
const WAIT="waiting for the fleet to infer @semio-tech/s-dev-composition",WARTE="Warte darauf, dass die Flotte @semio-tech/s-dev-composition ableitet";
const stepLine=(seconds:number,done:number,bound:number,german=false)=>german
 ?`Lese den Projektgraph-Ausschnitt (${seconds} s): ${WAIT}${done>bound?` (beyond the expected ${bound} s)`:""} / ${WARTE}${done>bound?` (über den erwarteten ${bound} s)`:""} ${done} von ${bound}.`
 :`Reading the project graph slice (${seconds} s): ${WAIT}${done>bound?` (beyond the expected ${bound} s)`:""} / ${WARTE}${done>bound?` (über den erwarteten ${bound} s)`:""} ${done} of ${bound}.`;
const FAST={...TREE,speedFactor:1,stallStopMilliseconds:6000,stallMilliseconds:6000};

test("the step line of the slice worker is parsed in both languages, with and without the note that it passed its bound",()=>{
 for(const german of[false,true]){
  const inside=readyStepOfV1(["noise",stepLine(900,120,1800,german)])!,beyond=readyStepOfV1([stepLine(11802,11265,1800,german)])!;
  expect(inside.seconds).toBe(120);expect(inside.bound).toBe(1800);expect(beyond.seconds).toBe(11265);expect(beyond.bound).toBe(1800);
  expect(beyond.text,"the over-bound note is not part of the step").toBe(inside.text);expect(inside.text).toContain("s-dev-composition");
 }
 expect(readyStepOfV1(["Reading the project graph slice (50 s): taken 8 of 590.","Reading the project graph slice (50 s): owners done 590 of 590.","Reading the project graph slice (9 s): taken 3 of 590; running (name, kind, seconds, rows): x."])).toBeUndefined();
 expect(readyStepOfV1(["[repo] still working after 410 s; last phase started: door.script"])).toBeUndefined();
});

test("law: every declared step the slice worker can print is a step line the supervisor reads, in both languages, inside and beyond its bound",async()=>{
 const steps=slicePolicy.definitions.Steps.const.steps as Record<string,{boundMilliseconds:number}>,ids=Object.keys(steps);
 expect(ids.length).toBeGreaterThanOrEqual(11);
 for(const id of ids)await sliceReadStepV1(id,async()=>{
  const bound=steps[id]!.boundMilliseconds/1000;
  for(const seconds of[5,bound+77]){
   const row=describeSliceReadStepV1(performance.now()+seconds*1000)!;
   expect(row.id).toBe(id);
   for(const language of["en","de"] as const){
    const line=fill(messages[language]["host-slice-detail"]!,{seconds:600,phase:row.phase,done:row.done,total:row.total}),step=readyStepOfV1([line]);
    expect(step,`${id} ${language} ${seconds}: ${line}`).toBeDefined();
    expect(step!.bound).toBe(bound);expect(Math.abs(step!.seconds-seconds)).toBeLessThanOrEqual(2);expect(step!.text).not.toContain("beyond the expected");
    expect(step!.text).not.toContain("über den erwarteten");
   }
  }
 },{who:"g1-m09",names:"@semio-tech/s-dev-composition",round:1});
});

test("the overdue limit is the declared bound lengthened by the speed factor plus the silence window of the stage, nothing else",()=>{
 expect(readyStepOverdueSecondsV1(1800)).toBe(1800+600);
 expect(readyStepOverdueSecondsV1(600)).toBe(600+600);
 const slow=readyScaledTreePolicyV1(2.83);
 expect(slow.speedFactor).toBe(2.83);expect(readyStepOverdueSecondsV1(1800,slow)).toBe(Math.ceil((Math.ceil(1800000*2.83)+Math.ceil(600000*2.83))/1000));
 expect(readyStepOverdueSecondsV1(1800,readyScaledTreePolicyV1(1))).toBe(2400);
 expect(readyStepOverdueSecondsV1(1800,slow)).toBeGreaterThan(readyStepOverdueSecondsV1(1800));
});

test("law: the watch ends an overdue step after the quiet window, not a step within its bound, not one that moves something else, not another step",()=>{
 const run=(rows:readonly (readonly string[])[],policy=FAST,start=0,every=1000)=>{const watch=readyStepWatchV1(policy);return rows.map((lines,index)=>watch.observe(lines,start+index*every));};
 const many=(count:number,make:(index:number)=>readonly string[])=>Array.from({length:count},(_,index)=>make(index));
 const overdue=run(many(12,index=>[stepLine(3000+index,9000+index,2),"[repo] still working after 30 s; last phase started: door.script"]));
 expect(overdue[0]!.overdue,"the quiet window starts at the first look").toBe(false);expect(overdue[5]!.overdue).toBe(false);expect(overdue[6]!.overdue).toBe(true);expect(overdue[11]!.overdue).toBe(true);
 expect(overdue[11]!.step!.bound).toBe(2);
 const inside=run(many(30,index=>[stepLine(100+index,index+1,600)]));
 expect(inside.some(row=>row.overdue),"a step within its bound is never overdue").toBe(false);
 const justBefore=run(many(30,index=>[stepLine(100+index,7+index%1,2)]),{...FAST,stallStopMilliseconds:6000});
 expect(justBefore.every(row=>row.step!.seconds===7)).toBe(true);expect(justBefore[29]!.overdue,"7 s is below the limit of 8 s").toBe(false);
 const moving=run(many(30,index=>[stepLine(3000+index,9000+index,2),`Reading the project graph slice (${3000+index} s): taken ${index} of 590.`]));
 expect(moving.some(row=>row.overdue),"a step beyond its bound that makes other progress is not stopped").toBe(false);
 const stopped=run(many(30,index=>[stepLine(3000+index,9000+index,2),index<10?`Reading the project graph slice (${3000+index} s): taken ${index} of 590.`:"[repo] still working after 30 s; last phase started: door.script"]));
 expect(stopped[10]!.overdue).toBe(false);expect(stopped.slice(0,15).some(row=>row.overdue)).toBe(false);expect(stopped[29]!.overdue,"the quiet window counts from the last other progress").toBe(true);
 const changing=run(many(30,index=>[stepLine(3000+index,9000+index,2).replace("s-dev-composition",index<12?"s-dev-composition":"norm-js")]));
 expect(changing[11]!.overdue,"the same step for 12 s").toBe(true);expect(changing[12]!.overdue,"another step is progress").toBe(false);expect(changing[17]!.overdue).toBe(false);expect(changing[18]!.overdue).toBe(true);
 const slow=run(many(11,index=>[stepLine(3000+index,9000+index,2)]),{...FAST,speedFactor:1,stallStopMilliseconds:12000});
 expect(slow.some(row=>row.overdue),"a longer silence window of a slower machine ends later").toBe(false);
});

test("the typed reason, its refusal and its kill record exist in both languages",()=>{
 expect(schema.definitions.Refusal.properties.code.enum).toContain("stage-step-overdue");expect(schema.definitions.Reason.properties.code.enum).toContain("step-overdue");
 const refusal=readyRefusalV1("stage-step-overdue",{stage:"workspace",step:`${WAIT} / ${WARTE}`,seconds:11265,bound:1800,limit:2400,quiet:600,pause:30,count:1,maximum:5});
 for(const text of[refusal.en,refusal.de]){expect(text).toContain("workspace");expect(text).toContain("11265");expect(text).toContain("1800");expect(text).toContain("2400");expect(text).toContain("s-dev-composition");expect(text).not.toContain("{");}
 expect(refusal.en).toContain("Stage workspace");expect(refusal.de).toContain("Stufe workspace");
 const kill=readyKillRecordLineV1({pid:7,name:"bun",signal:"SIGTERM",killer:1,stage:"workspace",owner:"o",reason:"overdue",seconds:600});
 expect(kill).toContain("step of the read");expect(kill).toContain("Schritt des Lesens");expect(kill).not.toContain("{");
});

const overdueRepo=(language:"en"|"de",lines=120)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${lines} ]; do i=$((i+1)); echo "${stepLine(4000,9000,2,language==="de")}" | sed "s/ 9000 / $((9000+i)) /"; sleep 0.3; done\nexit 0\n`;
const SLOW={...TREE,speedFactor:1,stallStopMilliseconds:12000,stallMilliseconds:12000};
const run=(home:string,root:string,tree=FAST)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree});

for(const language of["en","de"] as const)posix(`a stage whose step ran far past its bound with nothing else moving is stopped with the step named (${language})`,async()=>{
 const root=temp("root"),home=fakeHome(overdueRepo(language)),began=Date.now(),result=await run(home,root);
 expect(result.stalled).toBe(true);expect(result.timedOut).toBe(false);expect(result.exit).not.toBe(0);
 expect(result.stepOverdue,"the result names the step").toBeDefined();expect(result.stepOverdue!.text).toContain("s-dev-composition");expect(result.stepOverdue!.bound).toBe(2);expect(result.stepOverdue!.seconds).toBeGreaterThan(8);expect(result.stepOverdue!.limit).toBe(8);expect(result.stepOverdue!.quiet).toBeGreaterThanOrEqual(6);
 expect(result.codeChanged).toBeUndefined();expect(result.holderSilent).toBeUndefined();
 expect(Date.now()-began).toBeLessThan(30000);
 const kills=existsSync(join(root,".cache","f","ready","kills.log"))?readFileSync(join(root,".cache","f","ready","kills.log"),"utf8"):"";
 expect(kills===""||kills.includes("overdue")||kills.includes("überfällig")||kills.includes("Schritt")).toBe(true);
},60000);

posix("a stage whose step is within its bound goes on",async()=>{
 const root=temp("root"),repo=`#!/bin/sh\ni=0\nwhile [ $i -lt 36 ]; do i=$((i+1)); echo "${stepLine(100,1,600)}" | sed "s/ 1 of / $i of /"; sleep 0.3; done\nexit 0\n`,result=await run(fakeHome(repo),root);
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.stepOverdue).toBeUndefined();
},60000);

posix("the executing phase is never stopped by the rule, even when an overdue step line stands in the log (the tasks use processor time only)",async()=>{
 const root=temp("root"),repo=`#!/bin/sh\ni=0\necho "${stepLine(4000,9000,2)}"; echo "NX   Running target build for project @semio-tech/x and 2 tasks they depend on:"; while [ $SECONDS -lt 30 ]; do i=$((i+1)); done\nexit 0\n`,result=await run(fakeHome(repo),root,SLOW);
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.stepOverdue).toBeUndefined();
},60000);

posix("processor time alone is no progress for a step beyond its bound: the wait that re-reads the same thing for hours burns it too",async()=>{
 const root=temp("root"),repo=`#!/bin/sh\necho "${stepLine(4000,9000,2)}"; while [ $SECONDS -lt 80 ]; do i=$((i+1)); done\nexit 0\n`,began=Date.now(),result=await run(fakeHome(repo),root,SLOW);
 expect(result.stalled).toBe(true);expect(result.stepOverdue).toBeDefined();expect(result.stepOverdue!.bound).toBe(2);expect(result.stepOverdue!.limit).toBe(14);expect(Date.now()-began).toBeLessThan(60000);
},60000);

posix("a step that advances beyond its bound while the read moves something else is not stopped",async()=>{
 const root=temp("root"),repo=`#!/bin/sh\ni=0\nwhile [ $i -lt 40 ]; do i=$((i+1)); echo "${stepLine(4000,9000,2)}" | sed "s/ 9000 / $((9000+i)) /"; echo "Reading the project graph slice (4000 s): taken $i of 590."; sleep 0.3; done\nexit 0\n`,result=await run(fakeHome(repo),root);
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.stepOverdue).toBeUndefined();
},60000);

type Options=Parameters<ReadyPortsV1["run"]>[1];
const world=(adopt:(options:Options)=>ReturnType<typeof runReadyStageV1>)=>{
 const shared={state:emptyReadyStateV1() as ReadyLocalStateV1,now:1_000_000},controller=new AbortController(),mirrors=new Map<string,string[]>(),updates:Partial<ReadyStatusV1>[]=[],said:string[]=[],directory=temp("lease");
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)});
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,1));if(updates.some(update=>update.refusal?.code==="stage-step-overdue"||update.refusal?.code==="stage-failed")||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>false,write:()=>{}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  run:async(stage,options)=>stage.name==="workspace"?adopt(options):{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:1,tail:[]},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
  mirror:(stage,lines)=>{mirrors.set(stage,[...lines]);},
 };
 return{ports,controller,mirrors,shared,updates,said};
};

posix("the loop turns the stop into the typed refusal and reason, not a failure, and says it in both languages",async()=>{
 const root=temp("root"),home=fakeHome(overdueRepo("en")),w=world(options=>runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{},tree:FAST,...options}));
 await runReadyV1(w.ports,w.controller.signal);
 const refusal=w.updates.find(update=>update.refusal?.code==="stage-step-overdue")!.refusal!;
 expect(refusal.en).toContain("Stage workspace");expect(refusal.en).toContain("s-dev-composition");expect(refusal.de).toContain("Stufe workspace");expect(refusal.de).toContain("Schritt");
 expect(w.shared.state.failure,"a stop for an overdue step is not a failure").toBeNull();
 expect(w.said.some(line=>line.includes("ran the step")&&line.includes(" / Stufe workspace führte den Schritt"))).toBe(true);
 expect(w.mirrors.get("workspace-stalled")).toBeDefined();
},90000);

posix("a stage process that vanishes says how: no exit file, no signal, last seen alive, no signal sent by the supervisor; the refusal says it too",async()=>{
 const root=temp("root"),home=fakeHome("#!/bin/sh\necho started\nsleep 12\nexit 0\n"),directory=join(root,".cache","f","ready"),running=join(directory,"running.json");
 const w=world(options=>{const pending=runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{},tree:FAST,...options});(async()=>{for(let waited=0;waited<10000&&!existsSync(running);waited+=50)await new Promise<void>(done=>setTimeout(done,50));const record=JSON.parse(readFileSync(running,"utf8")) as {pid:number};alive.push(record.pid);await new Promise<void>(done=>setTimeout(done,800));process.kill(record.pid,"SIGKILL");})();return pending;});
 await runReadyV1(w.ports,w.controller.signal);
 const failure=w.shared.state.failure!;
 expect(failure.exit).toBeNull();expect(failure.signal??null).toBeNull();
 const text=failure.tail.join("\n");
 expect(text).toContain("is gone");expect(text).toContain("no exit file");expect(text).toContain("sent no signal");expect(text).toContain("last seen alive");expect(text).toContain("Der Stufenprozess");expect(text).toContain("sandte kein Signal");
 const mirror=w.mirrors.get("workspace-failed")!.join("\n");expect(mirror).toContain("is gone");expect(mirror).toContain("supervisor");
 const refusal=w.updates.find(update=>update.refusal?.code==="stage-failed")!.refusal!;
 expect(refusal.en).toContain("no exit status and no signal");expect(refusal.de).toContain("kein Exit-Status und kein Signal");expect(refusal.en).not.toContain("status -");
},90000);

posix("a stage ended by a signal says which one in the refusal and in the failure",async()=>{
 const root=temp("root"),home=fakeHome("#!/bin/sh\nkill -KILL $$\n"),w=world(options=>runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{},tree:FAST,...options}));
 await runReadyV1(w.ports,w.controller.signal);
 const failure=w.shared.state.failure!;expect(failure.exit).toBeNull();expect(failure.signal).toBe("SIGKILL");
 const refusal=w.updates.find(update=>update.refusal?.code==="stage-failed")!.refusal!;
 expect(refusal.en).toContain("ended by signal SIGKILL");expect(refusal.de).toContain("durch das Signal SIGKILL beendet");
 expect(failure.tail.join("\n")).toContain("SIGKILL");
},90000);

test("the ending of a stage in words: an exit status, a signal, or neither",()=>{
 expect(readyEndingV1(7,null).en).toBe("exit status 7");expect(readyEndingV1(7,null).de).toBe("Exit-Status 7");
 expect(readyEndingV1(null,"SIGTERM").en).toContain("SIGTERM");expect(readyEndingV1(null,"SIGTERM").de).toContain("SIGTERM");
 expect(readyEndingV1(null,null).en).toContain("vanished");expect(readyEndingV1(null,undefined).de).toContain("verschwand");
 for(const exit of[0,1,75,null])for(const signal of[null,"SIGKILL"]){const refusal=readyRefusalV1("stage-failed",{stage:"workspace",ending:readyEndingV1(exit,signal)});expect(refusal.en).not.toContain("{");expect(refusal.en).not.toContain("status -");expect(refusal.de).not.toContain("{");}
});

const PS="  1234     1    01:40 bun\n";
test("law: the start time of a process counts from the moment the process table was asked for, so a table that takes 30 s does not make the stage wrapper another process",async()=>{
 let clock=1_000_000;
 const table=await readyReadTableV1(20000,"linux",()=>clock,async()=>{clock+=30_000;return PS;});
 expect(table).toHaveLength(1);expect(table[0]!.startMs,"counted from the start of the read").toBe(1_000_000-100_000);
 expect(readyIdentityV1({pid:1234,startMs:1_000_000-100_000-2_000},table)).toBe("same");
 expect(readyIdentityV1({pid:1234,startMs:1_000_000-100_000-40_000},table)).toBe("reused");
 expect(await readyReadTableV1(20000,"linux",()=>0,async()=>null)).toEqual([]);expect(await readyReadTableV1(20000,"linux",()=>0,async()=>{throw Error("x");})).toEqual([]);
});

test("the real process table agrees with the clock of the runtime about this process",async()=>{
 const before=Date.now(),table=await readyReadTableV1(),row=table.find(entry=>entry.pid===process.pid);
 expect(row).toBeDefined();
 expect(Math.abs(row!.startMs-(before-process.uptime()*1000))).toBeLessThanOrEqual(3000);
});

test("a lower count in a new inventory observation still advances the explicit step",()=>{
 const watch=readyStepWatchV1(FAST),{line,nextCount}=inventoryHeartbeat;
 expect(watch.observe([line],0).step).toMatchObject(inventoryHeartbeat.step);
 const clocks=line.replace("(2303 s)","(2310 s)").replace("2093 of 600","2100 of 600");
 expect(watch.observe([clocks],6000).overdue).toBe(true);
 const next=line.replaceAll("333961",String(nextCount));
 expect(watch.observe([next],6500).overdue).toBe(false);
 expect(watch.observe([next],12499).overdue).toBe(false);
 expect(watch.observe([next],12500).overdue).toBe(true);
});

posix("the supervisor drops byte-truncated inventory heartbeats and reads the complete bound",async()=>{
 const root=temp("inventory"),replay=inventoryHeartbeat.stationary;
 const line=inventoryHeartbeat.line.replaceAll("333961",String(replay.observations)).replace(" (elapsed seconds",` (${replay.owners} completed owner checks) (elapsed seconds`).replace(" (verstrichene Sekunden",` (${replay.owners} abgeschlossene Projektprüfungen) (verstrichene Sekunden`);
 const burst=Array.from({length:replay.initialLines},(_,index)=>line.replace("2093 of",`${2093+index} of`)).join("\n");
 expect(Buffer.byteLength(burst)).toBeGreaterThan(schema.definitions.Policy.const.tailBytes);
 const script=`#!/bin/sh
printf '%s\\n' '${burst}'
i=${replay.initialLines}
while [ $i -lt ${replay.initialLines+48} ]; do i=$((i+1)); printf '%s\\n' '${line}' | sed "s/ 2093 of / $((2093+i)) of /"; sleep 0.3; done
exit 0
`;
 const displayed:string[]=[];
 const result=await runReadyStageV1({home:fakeHome(script),root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:text=>displayed.push(text),tree:FAST});
 expect(result.stepOverdue?.text).toContain(`${replay.observations} distinct source observations`);
 expect(result.stepOverdue?.text).toContain(`${replay.owners} completed owner checks`);
 expect(result.stepOverdue?.bound).toBe(600);
 expect(result.stepOverdue?.quiet).toBeGreaterThanOrEqual(6);
 expect(result.timedOut).toBe(false);
 expect(displayed.some(text=>text.includes(`${replay.observations} distinct source observations`))).toBe(true);
 expect(displayed.filter(text=>text.startsWith("Reading the project graph slice")).every(text=>text.length<=300)).toBe(true);
},60000);

test("verified memo rows advance the bilingual step only at an increased high-water mark",async()=>{
 for(const language of ["en","de"] as const){
  let rows=0;const watch=readyStepWatchV1(FAST),started=performance.now();
  await sliceReadStepV1("inventory",async()=>{
   for(const sample of inventoryHeartbeat.verifiedMemo){
    rows=sample.rows;
    const step=describeSliceReadStepV1(started+1800000+sample.milliseconds)!;
    const line=fill(messages[language]["host-slice-detail"]!,{seconds:2000+Math.floor(sample.milliseconds/1000),phase:step.phase,done:step.done,total:step.total});
    expect(watch.observe([line],sample.milliseconds).overdue).toBe(sample.overdue);
   }
  },{},()=>({version:1,active:{phase:"scoped-discovery",completedOwners:0,verifiedMemoRows:rows},completed:[]}));
 }
});

test("completed owner checks advance an inventory step with a stationary source high-water in both languages",()=>{
 const phase=inventoryHeartbeat.line.replace("Reading the project graph slice (2303 s): ","").replace(" 2093 of 600.","");
 const first=inventoryHeartbeat.ownerCounts[0]!,next=inventoryHeartbeat.ownerCounts[1]!;
 for(const language of ["en","de"] as const){
  const watch=readyStepWatchV1(FAST);
  const line=(count:number,clock:number)=>fill(messages[language]["host-slice-detail"]!,{seconds:2303+clock,done:2093+clock,total:600,
   phase:phase.replace(" (elapsed seconds",` (${count} completed owner checks) (elapsed seconds`).replace(" (verstrichene Sekunden",` (${count} abgeschlossene Projektprüfungen) (verstrichene Sekunden`)});
  expect(watch.observe([line(first,0)],0).overdue).toBe(false);
  expect(watch.observe([line(first,6)],6000).overdue).toBe(true);
  const changed=watch.observe([line(next,7)],7000);
  expect(changed.step?.text).toContain(`${next} completed owner checks`);
  expect(changed.step?.text).toContain("333961 distinct source observations");
  expect(changed.overdue).toBe(false);
  expect(watch.observe([line(next,12)],12999).overdue).toBe(false);
  expect(watch.observe([line(next,13)],13000).overdue).toBe(true);
 }
});
