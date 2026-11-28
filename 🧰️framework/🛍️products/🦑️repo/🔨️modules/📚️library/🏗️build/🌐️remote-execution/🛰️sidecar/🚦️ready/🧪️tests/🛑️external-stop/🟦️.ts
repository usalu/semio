import{expect,test}from"bun:test";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,readyExternallyStoppedV1,readySourceTransientV1,readyRefreshV1}from"../../🟦️.ts";

/** 🛑️ Contract: a stage that another process stopped (a stop signal, its exit status, or evidence lines saying a stop signal reached the run) is `interrupted`, never real evidence `stage-failed`, so a Ready worker that refreshes stays Ready; a graph read whose service was replaced by newer code is `source-transient`.
 * @see ../../🟦️.ts */
const oracle=(payload:unknown)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),"stopped",JSON.stringify(payload)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const cases:{result:{exit:number|null;signal?:string|null;tail:string[];failure?:string[]};want:boolean}[]=[
 {result:{exit:null,signal:"SIGTERM",tail:["the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet"]},want:true},
 {result:{exit:143,signal:null,tail:["repository:command failed"]},want:true},
 {result:{exit:1,signal:null,tail:["[nx] SIGTERM reached this run while Nx was still running; exit status 143; the sender is unknown"]},want:true},
 {result:{exit:1,signal:null,tail:["Cancelled (SIGTERM); leases released."]},want:true},
 {result:{exit:1,signal:null,tail:["SIGTERM received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest."]},want:true},
 {result:{exit:1,signal:null,tail:["x"],failure:["\u001b[31mthe stage process was ended by signal SIGHUP\u001b[0m"]},want:true},
 {result:{exit:1,signal:null,tail:["error[E0308]: mismatched types"]},want:false},
 {result:{exit:1,signal:null,tail:["a user script printed SIGTERM somewhere"]},want:false},
 {result:{exit:null,signal:"SIGKILL",tail:[]},want:false},
 {result:{exit:0,signal:null,tail:[]},want:false},
];
test("the external stop classification equals an independent Python implementation",()=>{
 const expected=oracle(cases.map(row=>({result:row.result,signals:P.externalStopSignals,exits:P.externalStopExits,patterns:P.externalStopPatterns})));
 cases.forEach((row,index)=>{expect(readyExternallyStoppedV1(row.result),JSON.stringify(row)).toBe(row.want);expect(expected[index]).toBe(row.want);});
});
test("a graph read whose service was replaced is source-transient in both languages and the interrupted class keeps a refreshing worker Ready",()=>{
 const root="/Users/ueli/Documents/semio";
 expect(readySourceTransientV1(["The repo graph service ends (superseded) before this graph read finished; the read is asked again of the service that follows it and continues from what was kept."],root)).toBe(true);
 expect(readySourceTransientV1(["Der Repo-Graphdienst endet (superseded), bevor dieses Lesen des Graphen fertig war; das Lesen wird beim nachfolgenden Dienst erneut angefragt."],root)).toBe(true);
 expect(readySourceTransientV1(["The repo graph service is slow"],root)).toBe(false);
 const stages=[{name:"workspace",digest:"2".repeat(64)}],now=1e12,input={stages,done:{workspace:"1".repeat(64)},doneMs:{workspace:now-1000},now,marker:true,repo:true,readyMs:now-5000};
 expect(readyRefreshV1({...input,failure:{code:"interrupted"}})).toEqual(["workspace"]);
 expect(readyRefreshV1({...input,failure:{code:"stage-failed"}})).toBeNull();
});
