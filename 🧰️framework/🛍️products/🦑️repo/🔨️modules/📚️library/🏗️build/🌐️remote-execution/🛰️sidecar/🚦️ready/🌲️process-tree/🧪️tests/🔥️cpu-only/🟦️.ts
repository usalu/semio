/** 🔥️ Contract: processor time alone keeps a Windows stage alive only for `cpuOnlyStallMilliseconds`; any other movement (a new line, a counter, a new process, a slice move) clears the streak. The m05 `tools` stage printed only heartbeats for 6900 s while its tree burnt processor time every sample, so the stall clock restarted for ever. The model replays the supervisor's two moves; the independent Python model computes the stop time.
 * @see ../../🟦️.ts `readyCpuOnlyStalledV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{READY_TREE_POLICY_V1 as policy,readyCpuOnlyStalledV1,readyStalledV1}from"../../🟦️.ts";

const replay=(events:readonly (readonly [number,"cpu"|"line"|"quiet"])[]):number|null=>{
 let stallAt=0,cpuSince:number|undefined;
 for(const [at,kind] of events){
  if(kind==="line"){stallAt=at;cpuSince=undefined;}
  if(kind==="cpu"){cpuSince??=at;stallAt=at;}
  if(readyStalledV1(at-stallAt)||readyCpuOnlyStalledV1(cpuSince,at))return at;
 }
 return null;
};
const tick=10_000,cpuOnly:(readonly [number,"cpu"])[]=Array.from({length:700},(_,index)=>[(index+1)*tick,"cpu"] as const);

test("a tree that only burns processor time is stopped after the cpu-only limit, not after hours",()=>{
 const stop=replay(cpuOnly);
 expect(stop).toBe(policy.cpuOnlyStallMilliseconds+tick);
 const run=spawnSync("python3",["-I","-c","import json,sys\nd=json.load(sys.stdin)\nt=d['tick']\nprint(next(i*t for i in range(1,10**6) if i*t-t>=d['limit']))"],{input:JSON.stringify({tick,limit:policy.cpuOnlyStallMilliseconds}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(Number(run.stdout)).toBe(stop!);
});

test("a new line in between clears the streak, so a compile that also prints is never cut",()=>{
 const events:(readonly [number,"cpu"|"line"|"quiet"])[]=cpuOnly.map(([at,kind])=>at%900_000===0?[at,"line"] as const:[at,kind] as const);
 expect(replay(events)).toBeNull();
 expect(readyCpuOnlyStalledV1(undefined,10**12)).toBe(false);
 expect(policy.cpuOnlyStallMilliseconds).toBeGreaterThan(policy.stallStopMilliseconds);
});
