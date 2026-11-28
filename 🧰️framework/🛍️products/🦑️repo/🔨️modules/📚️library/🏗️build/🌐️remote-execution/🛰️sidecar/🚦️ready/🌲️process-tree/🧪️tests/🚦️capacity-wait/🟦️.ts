/** 🚦️ Contract: a stage that waits in the capacity gate of the host is alive, not quiet. The stage runner stopped a whole stage after `stallStopMilliseconds` without new output or processor time, and a planner that waits for a capacity decision shows nothing new by design, so a read queued behind other agents was stopped as a whole (the SIGTERM of the ready stage runner in `kills.log`). The declared rows list the wait lines of both languages, the liveness law (current wait, fresh log, bounded duration) and stages run by the real runner: a live wait is not stopped, a wait that lasts past `capacityWaitMaximumMilliseconds`, a wait line followed by silence and a line that is no wait still are.
 * @see ../../🟦️.ts `readyCapacityWaitLiveV1`
 * @see ../../../🧪️tests/🕰️stale-waiting/🟦️.ts the stage fixture */
import{afterAll,expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_TREE_POLICY_V1 as TREE,readyCapacityWaitLineV1,readyCapacityWaitLiveV1,readyCapacityWaitingV1}from"../../🟦️.ts";
import{READY_STAGES_V1 as STAGES,runReadyStageV1}from"../../../🟦️.ts";

type Rows={lines:{line:string;wait:boolean}[];liveness:{id:string;since:number|null;written:number|null;now:number;fresh:number;maximum:number;live:boolean}[];stages:{id:string;line:string;iterations:number;silenceAfter?:boolean;policy:Record<string,number>;stalled:boolean;exit?:number}[]};
const rows=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")) as Rows;
const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`capacity-wait-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const emit=(line:string,iterations:number,silenceAfter:boolean)=>`#!/bin/sh\n"${process.execPath}" -e 'let i=0;const t=setInterval(()=>{i++;console.log(${line.split("${i}").map(part=>JSON.stringify(part)).join("+i+")});if(i>=${iterations}){clearInterval(t);${silenceAfter?"setTimeout(()=>{},60000)":""}}},300);'\n`;

test("wait lines of the capacity gate are recognised in both languages and no other line is",()=>{
 for(const row of rows.lines)expect(readyCapacityWaitLineV1(row.line),row.line).toBe(row.wait);
});

test("a wait among the last lines counts, one that scrolled out of them does not",()=>{
 const wait=rows.lines[2]!.line,other="built unit 3 of 99";
 expect(readyCapacityWaitingV1([other,other,wait,other])).toBe(true);
 expect(readyCapacityWaitingV1([wait,other,other,other,other])).toBe(false);
 expect(readyCapacityWaitingV1([])).toBe(false);
});

for(const row of rows.liveness)test("liveness: "+row.id,()=>{
 const policy={...TREE,capacityWaitFreshMilliseconds:row.fresh,capacityWaitMaximumMilliseconds:row.maximum};
 expect(readyCapacityWaitLiveV1({sinceMs:row.since,logWrittenMs:row.written,nowMs:row.now,policy})).toBe(row.live);
});

for(const row of rows.stages)posix("stage: "+row.id,async()=>{
 const home=fakeHome(emit(row.line,row.iterations,row.silenceAfter===true)),began=Date.now();
 const result=await runReadyStageV1({home,root:temp("root"),environment:{},stage:{...STAGES.find(stage=>stage.name==="tools")!,timeoutMilliseconds:600000},signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:{...TREE,semanticsSources:[],...row.policy}});
 expect(result.stalled,row.id).toBe(row.stalled);
 if(row.stalled){expect(Date.now()-began).toBeGreaterThan(4000);expect(Date.now()-began).toBeLessThan(60000);}
 else expect(result.exit).toBe(row.exit??null);
},90000);
