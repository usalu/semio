/** 🔁️ Contract: a slice worker that ends with the restart exit code (75: the code on this machine changed under it and the outermost command is to start again) is started again with the current code, a bounded number of times and with a progress line in English and German, never reported to the person as a failure of the plan; a worker that ends otherwise, a cancelled plan and a fourth restart in a row are not retried, and the last says in both languages what happened.
 * @see ../../../🧷️format-stamps/🟨️.mjs */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import settle from "./⏱️settle.json" with {type:"json"};

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const harness=(exits:(number|"done")[],options:{lives?:number[];step?:number;elapsed?:readonly number[];restartPolicy?:{backoffMilliseconds:number[];settleMilliseconds:number;progressMilliseconds:number;totalMaximum:number}}={})=>{
 const started:number[]=[],released:number[]=[],progress:any[]=[];let index=0,tick=0;
 const pool=createSlicePoolV1({...options.step===undefined&&options.elapsed===undefined?{}:{now:()=>tick},workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,...options.restartPolicy===undefined?{}:{restartPolicy:options.restartPolicy},hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reserve:async({megabytes}:{megabytes:number})=>{const id=started.length;return{megabytes,attach:()=>undefined,release:()=>{released.push(id);}};},
  createWorker:()=>{tick+=options.step??0;const outcome=exits[index++]!,listeners:Record<string,(value:unknown)=>void>={};started.push(index);const life=options.lives?.[index-1]??0,span=options.elapsed?.[index-1]??0,end=()=>{tick+=span;if(outcome==="done")listeners.message?.({type:"done",graph:{}});else listeners.exit?.(outcome);};if(life>0)setTimeout(end,life);else queueMicrotask(end);return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 return{pool,started,released,progress};
};

test("two restarts and then a finished load answer the plan, each restart is announced and every reservation is released",async()=>{
 const{pool,started,released,progress}=harness([75,75,"done"],{restartPolicy:{backoffMilliseconds:[1,2],settleMilliseconds:5000,progressMilliseconds:60,totalMaximum:6}});
 await pool.run({root:"/tmp/restart"},{onProgress:(_ms:number,detail:any)=>progress.push(detail)});await pool.close();
 expect(started.length).toBe(3);for(let wait=0;released.length<3&&wait<100;wait++)await new Promise(done=>setTimeout(done,20));expect(released.length).toBe(3);
 const restarts=progress.filter(detail=>detail?.decision?.key==="restarted").map(detail=>detail.decision);
 expect(restarts).toEqual([{key:"restarted",used:0,budget:5,kind:"75",limit:0},{key:"restarted",used:0,budget:5,kind:"75",limit:0}]);
 const text=renderTextV1("host-slice-owners-restarted",{seconds:7,phase:"reservation",done:0,total:0,owners:"",used:2,budget:900,limit:4,kind:"75"},["en","de"])!;
 for(const language of["en","de"] as const){expect(text[language]).toContain("75");expect(text[language]).toContain("2");expect(text[language]).toContain("900");expect(text[language]).not.toMatch(/\{[a-z]+\}/u);}
 expect(text.en).toContain("started again");expect(text.de).toContain("neu gestartet");
});

test("restarts under steady churn wait with growing pauses and converge: many quick restarts finish the plan inside the settle time, a replica that does not settle in time ends it with the typed error naming both languages",async()=>{
 const patient=harness([75,75,75,75,75,75,75,75,75,75,"done"],{restartPolicy:{backoffMilliseconds:[1,2,3],settleMilliseconds:60000,progressMilliseconds:60,totalMaximum:100}});
 await patient.pool.run({root:"/tmp/patient"});await patient.pool.close();
 expect(patient.started.length,"ten quick restarts do not fail the plan").toBe(11);
 const impatient=harness(Array.from({length:40},()=>75),{step:15,restartPolicy:{backoffMilliseconds:[1],settleMilliseconds:80,progressMilliseconds:60,totalMaximum:100}});
 const error:any=await impatient.pool.run({root:"/tmp/impatient"}).then(()=>undefined,failure=>failure);await impatient.pool.close();
 expect(error.code).toBe("slice-restarts");expect(error.exit).toBe(75);expect(error.restarts).toBe(impatient.started.length);
 expect(impatient.started.length,"ended by the settle time of the injected clock (15 per start, 80 to settle), not by a count of four").toBeGreaterThan(2);expect(impatient.started.length).toBeLessThan(10);
 expect(error.message).toContain("replica has settled");expect(error.message).toContain("Replikat");
 const pauses:number[]=[],paced=harness([75,75,75,75,75,"done"],{restartPolicy:{backoffMilliseconds:[2,4,8],settleMilliseconds:60000,progressMilliseconds:60,totalMaximum:100}});
 await paced.pool.run({root:"/tmp/paced"},{onProgress:(_ms:number,detail:any)=>{if(detail?.decision?.key==="restarted")pauses.push(detail.decision.limit);}});await paced.pool.close();
 expect(pauses.length).toBeGreaterThan(0);expect(pauses.every(value=>value===0)).toBe(true);
});

test("another exit code fails at once and a cancelled plan is not started again",async()=>{
 const other=harness([1,"done"]);
 const failure:any=await other.pool.run({root:"/tmp/other"}).then(()=>undefined,reason=>reason);await other.pool.close();
 expect(other.started.length).toBe(1);expect(failure.message).toBe("Slice worker exited with code 1");expect(failure.restart).toBe(false);
 const controller=new AbortController(),cancelled=harness([75,"done"]);
 controller.abort(new Error("stop"));
 const stopped:any=await cancelled.pool.run({root:"/tmp/cancel"},{signal:controller.signal}).then(()=>undefined,reason=>reason);await cancelled.pool.close();
 expect(stopped.message).toBe("stop");expect(cancelled.started.length).toBe(0);
});

test("a worker that lived past the progress time before it ended with the restart code made progress and starts the settle time again: many such restarts in a row finish the plan, and the total is bounded",async()=>{
 const policy={backoffMilliseconds:[1],settleMilliseconds:100,progressMilliseconds:60,totalMaximum:6};
 const slow=harness([75,75,75,75,75,"done"],{restartPolicy:policy,lives:[90,90,90,90,90,0]});
 await slow.pool.run({root:"/tmp/slow"});await slow.pool.close();
 expect(slow.started.length,"five restarts that lasted do not fail the plan").toBe(6);
 const endless=harness([75,75,75,75,75,75,75,75,"done"],{restartPolicy:policy,lives:[90,90,90,90,90,90,90,90,0]});
 const bounded:any=await endless.pool.run({root:"/tmp/endless"}).then(()=>undefined,reason=>reason);await endless.pool.close();
 expect(bounded.code).toBe("slice-restarts");expect(endless.started.length,"at most totalMaximum restarts, then the typed error").toBe(7);expect(bounded.restarts).toBe(7);
});

test("the declared restart policy backs off without shrinking, settles longer than a worker must last to count as progress, and the pauses follow an independent model",()=>{
 const policy=HOST_POLICY_V1 as unknown as{sliceRestartBackoffMilliseconds:number[];sliceRestartSettleMilliseconds:number;sliceRestartProgressMilliseconds:number};
 const pauses=policy.sliceRestartBackoffMilliseconds;
 expect(pauses.length).toBeGreaterThan(1);expect(pauses.every((value,at)=>Number.isInteger(value)&&value>0&&(at===0||value>=pauses[at-1]!))).toBe(true);
 expect(policy.sliceRestartSettleMilliseconds).toBeGreaterThan(policy.sliceRestartProgressMilliseconds);
 const model=(streak:number)=>pauses[Math.min(streak,pauses.length-1)]!;
 let spent=0;for(let streak=0;spent<policy.sliceRestartSettleMilliseconds;streak++)spent+=model(streak);
 expect(spent,"the declared settle time is reached by a bounded number of pauses").toBeGreaterThanOrEqual(policy.sliceRestartSettleMilliseconds);
});

for(const row of settle.cases)test(`the replica settles on time since transient restart, ${row.name}`,async()=>{
 const run=harness(row.exits as (number|"done")[],{elapsed:row.elapsed,restartPolicy:{backoffMilliseconds:[1],settleMilliseconds:settle.settleMilliseconds,progressMilliseconds:settle.progressMilliseconds,totalMaximum:100}});
 const waits:number[]=[];
 try{
  const failure=await run.pool.run({root:"/tmp/settle-clock"},{onProgress:(_ms:number,detail:{decision?:{key:string;used:number}})=>{if(detail.decision?.key==="restarted")waits.push(detail.decision.used);}}).then(()=>null,(error:Error & {code?:string})=>error.code);
  expect(failure).toBe(row.failure);
  expect(run.started.length).toBe(row.starts);
  expect(waits).toEqual(row.waitedSeconds);
 }finally{await run.pool.close();}
});
