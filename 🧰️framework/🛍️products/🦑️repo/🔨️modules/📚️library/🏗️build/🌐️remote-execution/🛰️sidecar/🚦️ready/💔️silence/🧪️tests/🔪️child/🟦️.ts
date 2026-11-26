import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {readySilenceChildNoticeV1,readySilenceChildWatchV1,readySilencePolicyV1,readySilenceStatusFileV1} from "../../🟦️.ts";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";

/** 🔪️ Contract: the launcher ends a readiness child that is silent for `silentBeatsMaximum` beats, but only a child older than the grace and at most once per cooldown; a ready, absent or fresh record never ends a child. */
const P=readySilencePolicyV1,now=10_000_000,silent={ready:false,state:"running",updatedMs:now-10*P.statusHeartbeatMilliseconds};

test("a young child is never judged, an old one with a silent record is ended once per cooldown",()=>{
 const watch=readySilenceChildWatchV1();
 expect(watch.observe(silent,now,now-P.silenceChildGraceMilliseconds+1)).toBe("wait");
 expect(watch.observe(silent,now,now-P.silenceChildGraceMilliseconds)).toBe("restart");
 expect(watch.observe(silent,now+1000,now-P.silenceChildGraceMilliseconds)).toBe("wait");
 const later=now+P.silenceRestartCooldownMilliseconds;
 expect(watch.observe({...silent,updatedMs:later-10*P.statusHeartbeatMilliseconds},later,later-P.silenceChildGraceMilliseconds)).toBe("restart");
});

test("a fresh, ready, absent or malformed record never ends the child",()=>{
 const watch=readySilenceChildWatchV1(),old=now-24*3_600_000;
 for(const status of [{ready:false,state:"running",updatedMs:now-1000},{ready:true,state:"ready",updatedMs:now-86_400_000},null,{},"x",{ready:false,state:"observing",updatedMs:now-86_400_000}])expect(watch.observe(status,now,old),JSON.stringify(status)).toBe("wait");
});

test("the launcher decisions equal the independent Python oracle on seeded cases",()=>{
 let seed=20261126;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;},pick=<T>(rows:readonly T[]):T=>rows[Math.floor(next()*rows.length)]!;
 const base=1_800_000_000_000,cases=[] as {status:unknown;nowMs:number;childStartedMs:number}[];
 let clock=base;
 for(let index=0;index<500;index++){
  clock+=pick([1000,30_000,90_000,200_000,300_000,400_000]);
  const age=pick([0,29_999,30_000,89_999,90_000,600_000,86_400_000]),kind=pick(["work","ready","observing","none","broken","refreshing"]);
  const status=kind==="none"?null:kind==="broken"?{updatedMs:"x"}:{ready:kind==="ready"||kind==="refreshing",state:kind==="work"?pick(["running","waiting","backoff"]):kind==="refreshing"?"ready":kind,updatedMs:clock-age,...kind==="refreshing"?{refreshing:true}:{},...next()<0.4?{beatMilliseconds:pick([1,30_000,120_000,10_000_000,-1])}:{}};
  cases.push({status,nowMs:clock,childStartedMs:clock-pick([0,1000,P.silenceChildGraceMilliseconds-1,P.silenceChildGraceMilliseconds,86_400_000])});
 }
 const run=spawnSync(requireTestToolV1("python"),[join(import.meta.dir,"🐍️.py")],{input:cases.map(row=>JSON.stringify(row)).join("\n"),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=run.stdout.split("\n").filter(Boolean),watch=readySilenceChildWatchV1();
 expect(expected.length).toBe(cases.length);
 cases.forEach((row,index)=>expect(watch.observe(row.status,row.nowMs,row.childStartedMs),JSON.stringify(row)).toBe(expected[index]));
 expect(new Set(expected).size).toBe(2);
});

test("the notice is bilingual and names the silence, the status file comes from the environment",()=>{
 const text=readySilenceChildNoticeV1({missedBeats:5,ageMs:151_000});
 expect(text).toContain("151 s");expect(text).toContain("5 missed beats");expect(text).toContain("verpasste Schläge");expect(text).toContain(" / ");
 expect(readySilenceStatusFileV1({SEMIO_FLEET_STORE_ROOT:"/s",SEMIO_FLEET_ORIGIN:"g1-m05"})).toBe(join("/s","v1","origins","g1-m05","ready.json"));
 for(const environment of [{},{SEMIO_FLEET_STORE_ROOT:"/s"},{SEMIO_FLEET_STORE_ROOT:"",SEMIO_FLEET_ORIGIN:"g1-m05"},{SEMIO_FLEET_STORE_ROOT:"/s",SEMIO_FLEET_ORIGIN:"../x"}])expect(readySilenceStatusFileV1(environment)).toBeUndefined();
 expect(P.heartbeatNames).toContain("head.json");
});
