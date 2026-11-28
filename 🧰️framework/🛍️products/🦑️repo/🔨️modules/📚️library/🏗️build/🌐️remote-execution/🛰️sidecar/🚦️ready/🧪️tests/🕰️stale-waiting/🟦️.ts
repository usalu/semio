/** 🕰️ Contract: a stage that was started by older code (its recorded code digest differs from the digest of the code now running, or it carries none and another supervisor adopted it) and only WAITS (its last line is a capacity, holder or grant wait, no line other than waits is new, its tree spends no processor time) for `staleWaitingMilliseconds` is stopped and starts again on the current code, as a restart that is no failure (`stage-code-stale-waiting`, both languages). A stage that makes real progress, or runs the current code, is never stopped by it.
 * @see ../../🌲️process-tree/🟦️.ts `readyStaleWaitingV1` */
import{afterAll,expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_TREE_POLICY_V1 as TREE,readyScaledTreePolicyV1,readyStaleWaitingV1,readyWaitLineV1,readyWaitedMillisecondsV1}from"../../🌲️process-tree/🟦️.ts";
import{READY_STAGES_V1 as STAGES,readyRefusalV1,readyReasonV1,runReadyStageV1}from"../../🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`stale-waiting-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds:600000});
const policy={...TREE,staleWaitingMilliseconds:2500,stallStopMilliseconds:600000,holderWaitRestartMilliseconds:600000,staleStageMilliseconds:600000};
const emit=(line:string,seconds:number)=>`#!/bin/sh\nexec "${process.execPath}" -e 'let i=0;const t=setInterval(()=>{i++;console.log(${line});if(i>=${Math.round(seconds*3)})clearInterval(t);},330);'\n`;
const waiting=(seconds:number)=>emit("`[capacity] waiting ${i}s for run: not enough free memory (need 6932 MB; ${-337-i*7} MB spare = 4830 MB available - 2048 MB floor - 999 MB reserved, short by ${763+i*7} MB; load 6.99; 3 running, 0 ahead)`",seconds);
const working=(seconds:number)=>emit("`built unit ${i} of 9999`",seconds);
const launch=async(home:string,root:string,code:string,tree=policy)=>{
 const stop=new AbortController(),first=runReadyStageV1({home,root,environment:{},stage:stageOf("tools"),signal:stop.signal,jobs:1,onLine:()=>undefined,tree,code});
 await Bun.sleep(1200);stop.abort();expect((await first).detached).toBe(true);
};
const adopt=(home:string,root:string,code:string,tree=policy)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree,code});

test("wait lines are recognised in both languages and nothing else is",()=>{
 for(const line of["[capacity] waiting 3589s for run: not enough free memory (need 6932 MB)","[capacity] waiting 0s for run-many: an earlier request is still waiting (need 512 MB)","Process 91672 (x) holds the dependency store since 2026-11-14T16:34:21.027Z; waiting for it (4 s of at most 21600 s)"])expect(readyWaitLineV1(line),line).toBe(true);
 for(const line of["[capacity] admitted run-many: 1 job(s), 512 MB reserved","built unit 3 of 99","Planning the selected targets and what they need.","",`[capacity] waiting for nothing`])expect(readyWaitLineV1(line),line).toBe(false);
});

test("law: the stale-waiting verdict needs older code, a waiting last line and a quiet work clock of the full window, as an independent model says",()=>{
 const window=TREE.staleWaitingMilliseconds;
 expect(window).toBe(3*TREE.codeQuietMilliseconds);expect(window).toBe(900000);
 for(const older of[true,false])for(const waitingNow of[true,false])for(const quiet of[0,window-1,window,window*9]){
  expect(readyStaleWaitingV1({older,waiting:waitingNow,quietMs:quiet}),`${older} ${waitingNow} ${quiet}`).toBe(older&&waitingNow&&quiet>=window);
 }
 expect(readyScaledTreePolicyV1(2.5).staleWaitingMilliseconds).toBe(Math.ceil(window*2.5));expect(readyScaledTreePolicyV1(1).staleWaitingMilliseconds).toBe(window);
});

test("the refusal and the reason are declared in both languages and say it is no failure",()=>{
 const refusal=readyRefusalV1("stage-code-stale-waiting",{stage:"tools",minutes:15}),reason=readyReasonV1("code-stale-waiting","tools");
 expect(refusal.en).toContain("tools");expect(refusal.en).toContain("15");expect(refusal.en).toContain("not counted as a failure");expect(refusal.de).toContain("zählt nicht als Fehler");expect(refusal.de).toContain("15");
 expect(reason.en).toContain("tools");expect(reason.de).toContain("tools");expect(reason.en).not.toBe(reason.de);
});

posix("an adopted stage on older code that only waits in the capacity queue is stopped and handed over, not counted as a stall",async()=>{
 const root=temp("root"),home=fakeHome(waiting(60)),began=Date.now();
 await launch(home,root,"old-code");
 const result=await adopt(home,root,"new-code");
 expect(result.stalled).toBe(true);expect(result.codeStaleWaiting).toBe(true);expect(result.codeChanged).toBeDefined();expect(result.timedOut).toBe(false);
 expect(Date.now()-began).toBeGreaterThan(2500);expect(Date.now()-began).toBeLessThan(40000);
},60000);

posix("a stage on the current code is never stopped for waiting by this rule",async()=>{
 const root=temp("root"),home=fakeHome(waiting(7));
 await launch(home,root,"same-code");
 const result=await adopt(home,root,"same-code");
 expect(result.codeStaleWaiting).toBeUndefined();expect(result.stalled).toBe(false);expect(result.exit).toBe(0);
},60000);

posix("a stage on older code that makes real progress is never restarted",async()=>{
 const root=temp("root"),home=fakeHome(working(7));
 await launch(home,root,"old-code");
 const result=await adopt(home,root,"new-code");
 expect(result.codeStaleWaiting).toBeUndefined();expect(result.stalled).toBe(false);expect(result.exit).toBe(0);
},60000);

test("the age of a wait is read from its own line in both languages, nothing else gives one",()=>{
 expect(readyWaitedMillisecondsV1("[capacity] waiting 15549s for command: an earlier request is still waiting (need 2048 MB)")).toBe(15_549_000);
 expect(readyWaitedMillisecondsV1("[Kapazität] wartet 120s für command: x")).toBe(120_000);
 for(const line of["[capacity] waiting for nothing","built unit 3 of 99","","[capacity] admitted command: 1 job(s)"])expect(readyWaitedMillisecondsV1(line),line).toBeNull();
});
