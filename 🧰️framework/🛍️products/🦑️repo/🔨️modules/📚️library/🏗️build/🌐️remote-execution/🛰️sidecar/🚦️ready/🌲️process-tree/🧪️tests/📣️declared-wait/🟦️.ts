/** 📣️ Law: a declared wait and the stall detector agree. The heartbeat of a bounded tool hook is a step line the supervisor reads in both languages (tool, step in both languages, seconds run, declared bound); every beat is new for the stall tracker so a legitimate wait is never stopped as silent before its own bound, the step is overdue only after its bound plus the silent window (always after the hook's own typed timeout), and a heartbeat that only counts seconds with a unit stays no progress. */
import{expect,test}from"bun:test";
import{readyNormaliseLineV1,readyStepOfV1,readyStallTrackerV1,readyStalledV1,readyStepOverdueSecondsV1,readyStepWatchV1,READY_TREE_POLICY_V1 as P}from"../../🟦️.ts";
import{TOOL_HOOK_POLICY_V1,runToolHookBoundedV1}from"../../../../../../🧰️tool-setup/⏱️hook-bound/🟦️.ts";

const beats=async(count:number):Promise<readonly string[]>=>{
 const lines:string[]=[];let clock=0;
 await runToolHookBoundedV1({tool:"tectonic",step:"verify",script:"C:\\semio\\print\\toolchain\\script.ts",signal:new AbortController().signal},()=>new Promise<void>(done=>{const poll=setInterval(()=>{if(lines.length>=count*2){clearInterval(poll);done();}},2);}),{say:line=>lines.push(line),now:()=>{const value=clock;clock+=TOOL_HOOK_POLICY_V1.heartbeatMilliseconds;return value;},heartbeatMilliseconds:3,boundMilliseconds:TOOL_HOOK_POLICY_V1.steps.verify.boundMilliseconds});
 return lines;
};
const bound=TOOL_HOOK_POLICY_V1.steps.verify.boundMilliseconds/1000,beat=TOOL_HOOK_POLICY_V1.heartbeatMilliseconds/1000;

test("the supervisor reads every heartbeat of the verify step in both languages as the step it declares, with the seconds run and the declared bound",async()=>{
 const lines=await beats(8);
 for(const [index,line] of lines.entries()){
  const step=readyStepOfV1([line]);
  expect(step,line).toBeDefined();expect(step!.bound).toBe(bound);expect(step!.seconds).toBe((Math.floor(index/2)+1)*beat);
  expect(step!.text).toBe(TOOL_HOOK_POLICY_V1.steps.verify.en+" / "+TOOL_HOOK_POLICY_V1.steps.verify.de);
 }
 expect(lines[0]).toContain("tectonic");expect(lines[0]).toContain("script.ts");expect(lines[1]).toContain("von");
});

test("every beat of a wait that lasts the whole bound is new for the stall tracker, so the longest silent stretch is one beat and the stage is never stopped as silent before its own bound",async()=>{
 const lines=await beats(bound/beat),tracker=readyStallTrackerV1();let freshAt=0,longest=0,at=0;
 for(let index=0;index<lines.length;index+=2){
  at+=TOOL_HOOK_POLICY_V1.heartbeatMilliseconds;
  if(tracker.observe([lines[index]!,lines[index+1]!,"[repository:command] still running after 1 s","[repo] still working after 1 s; last phase started: source.served"])){longest=Math.max(longest,at-freshAt);freshAt=at;}
 }
 longest=Math.max(longest,at-freshAt);
 expect(lines.length).toBe(2*bound/beat);expect(longest).toBeLessThanOrEqual(TOOL_HOOK_POLICY_V1.heartbeatMilliseconds);
 expect(readyStalledV1(longest,P)).toBe(false);expect(bound*1000).toBeGreaterThan(P.stallStopMilliseconds);
});

test("a step is overdue only after its bound plus the silent window, which is after the hook stopped itself with its typed timeout, and never while the beats come",async()=>{
 const lines=await beats(bound/beat),watch=readyStepWatchV1();let now=0,overdue=false;
 for(let index=0;index<lines.length;index+=2){now+=TOOL_HOOK_POLICY_V1.heartbeatMilliseconds;overdue=overdue||watch.observe([lines[index]!,lines[index+1]!],now).overdue;}
 expect(overdue).toBe(false);
 expect(readyStepOverdueSecondsV1(bound,P)).toBeGreaterThan(bound);
 const stuck=watch.observe([lines.at(-2)!.replace(/: \d+ of/u,": "+readyStepOverdueSecondsV1(bound,P)+" of")],now+P.stallStopMilliseconds);
 expect(stuck.overdue).toBe(true);
});

test("a heartbeat that only counts seconds with a unit is no progress, the form of the declared step is the way to be a legitimate long wait",()=>{
 const tracker=readyStallTrackerV1(),old=(seconds:number)=>`[tools] tectonic: verifying the pinned tool (script.ts) has run ${seconds} s of at most 1800 s`;
 expect(tracker.observe([old(30)])).toBe(true);expect(tracker.observe([old(60)])).toBe(false);
 expect(readyStepOfV1([old(60)])).toBeUndefined();
 expect(readyNormaliseLineV1(old(60))).toBe(readyNormaliseLineV1(old(90)));
});
