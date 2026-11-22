import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {readySidecarSilenceV1,readySilenceNoticeV1,readySilenceWatchV1,readyWorkerLivenessV1} from "../🟦️.ts";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 💔️ Contract: a sidecar at work that misses `silentBeatsMaximum` beats is silent and restarted once per cooldown; a ready, absent or stopped one never is. */
const P=schema.definitions.Policy.const,beat=P.statusHeartbeatMilliseconds,row=(state:string,age:number,ready=false)=>({ready,state,updatedMs:1_000_000-age});

test("verdict by missed beats",()=>{
 expect(readySidecarSilenceV1({status:row("running",beat-1),nowMs:1_000_000}).state).toBe("fresh");
 expect(readySidecarSilenceV1({status:row("running",beat*(P.silentBeatsMaximum-1)),nowMs:1_000_000}).state).toBe("fresh");
 const silent=readySidecarSilenceV1({status:row("backoff",beat*P.silentBeatsMaximum),nowMs:1_000_000});
 expect(silent).toEqual({state:"silent",missedBeats:P.silentBeatsMaximum,ageMs:beat*P.silentBeatsMaximum});
 expect(readySidecarSilenceV1({status:row("waiting",86_400_000),nowMs:1_000_000}).state).toBe("silent");
 expect(readySidecarSilenceV1({status:row("running",86_400_000,true),nowMs:1_000_000}).state).toBe("idle");
 expect(readySidecarSilenceV1({status:row("stopped",86_400_000),nowMs:1_000_000}).state).toBe("idle");
 for(const bad of [null,undefined,"x",{},{updatedMs:"1"}])expect(readySidecarSilenceV1({status:bad,nowMs:1}).state).toBe("absent");
});

test("one restart per silence and cooldown",()=>{
 const watch=readySilenceWatchV1(),silent=(now:number)=>({ready:false,state:"running",updatedMs:now-beat*10});
 expect(watch.observe(row("running",0),1_000_000)).toBe("wait");
 expect(watch.observe(silent(1_000_000),1_000_000)).toBe("restart");
 expect(watch.observe(silent(1_000_100),1_000_100)).toBe("wait");
 const later=1_000_000+P.silenceRestartCooldownMilliseconds;
 expect(watch.observe(silent(later),later)).toBe("restart");
});

test("the notice is bilingual and names the silence",()=>{
 const text=readySilenceNoticeV1({state:"silent",missedBeats:4,ageMs:125000});
 expect(text).toContain("125 s");expect(text).toContain(" / ");expect(text).toContain("verpasste");
});

test("the threshold follows the beat the worker declares in its record, bounded by the offline limit",()=>{
 const declared=(state:string,age:number,beatMilliseconds:number)=>({ready:false,state,updatedMs:1_000_000-age,beatMilliseconds});
 expect(readySidecarSilenceV1({status:declared("waiting",200_000,120_000),nowMs:1_000_000}).state,"a waiting worker that declared 120 s is fresh at 200 s").toBe("fresh");
 expect(readySidecarSilenceV1({status:declared("waiting",360_000,120_000),nowMs:1_000_000}).state).toBe("silent");
 expect(readySidecarSilenceV1({status:declared("running",600_000,1e12),nowMs:1_000_000}).state,"a huge declared beat is bounded").toBe("silent");
 expect(readySidecarSilenceV1({status:declared("running",90_000,-5),nowMs:1_000_000}).state,"an invalid beat falls back to the policy").toBe("silent");
});

test("worker liveness: a record is never trusted alone, the heartbeats of the machine decide offline first",()=>{
 const now=10_000_000,at=(age:number|null)=>age===null?null:now-age,work=(age:number,state="running")=>({ready:false,state,updatedMs:now-age});
 const live=(status:unknown,heartbeat:number|null)=>readyWorkerLivenessV1({status,heartbeatMs:at(heartbeat),nowMs:now});
 expect(live(work(5_000),1_000).liveness).toBe("working");
 expect(live(work(beat*P.silentBeatsMaximum),1_000).liveness).toBe("silent");
 expect(live(work(P.offlineMilliseconds),P.offlineMilliseconds).liveness).toBe("offline");
 expect(live(work(P.offlineMilliseconds-1),null).liveness).toBe("silent");
 expect(live(work(24*3_600_000),null).liveness).toBe("offline");
 expect(live(work(24*3_600_000),2_000).liveness).toBe("silent");
 expect(live({ready:true,state:"ready",updatedMs:now-24*3_600_000},2_000).liveness).toBe("ready");
 expect(live({ready:true,state:"ready",updatedMs:now-24*3_600_000},null).liveness).toBe("offline");
 expect(live({ready:false,state:"observing",updatedMs:now-5_000},500).liveness).toBe("idle");
 expect(live(null,500)).toEqual({liveness:"absent",ageMs:null,heartbeatAgeMs:500,missedBeats:0});
 expect(live(undefined,null).liveness).toBe("absent");
});

test("worker liveness equals the independent Python oracle on seeded cases",()=>{
 let seed=20261122;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;},pick=<T>(rows:readonly T[]):T=>rows[Math.floor(next()*rows.length)]!;
 const ages=[0,1,29_999,30_000,89_999,90_000,90_001,599_999,600_000,600_001,3_600_000,86_400_000,null],now=1_800_000_000_000,cases=[] as {status:unknown;heartbeatMs:number|null;nowMs:number}[];
 for(let index=0;index<400;index++){
  const age=pick(ages),beatAge=pick(ages),kind=pick(["work","ready","observing","server","none","broken"]);
  const status=kind==="none"||age===null?null:kind==="broken"?{updatedMs:"x"}:{ready:kind==="ready",state:kind==="work"?pick(["running","waiting","backoff"]):kind,updatedMs:now-age,...next()<0.4?{beatMilliseconds:pick([1,30_000,45_000,120_000,10_000_000,-1,0])}:{}};
  cases.push({status,heartbeatMs:beatAge===null?null:now-beatAge,nowMs:now});
 }
 const run=spawnSync(requireTestToolV1("python"),[join(import.meta.dir,"🐍️.py")],{input:cases.map(row=>JSON.stringify(row)).join("\n"),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=run.stdout.split("\n").filter(Boolean).map(line=>JSON.parse(line));
 expect(expected.length).toBe(cases.length);
 cases.forEach((row,index)=>expect(readyWorkerLivenessV1({status:row.status,heartbeatMs:row.heartbeatMs,nowMs:row.nowMs}),JSON.stringify(row)).toEqual(expected[index]));
 expect(new Set(expected.map(row=>row.liveness)).size).toBe(6);
});
