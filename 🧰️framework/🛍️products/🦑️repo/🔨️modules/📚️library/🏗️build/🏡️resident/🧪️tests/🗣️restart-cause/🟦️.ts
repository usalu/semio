/** 🗣️ Contract: the real error text of a slice worker travels with the restart decision (so a worker that dies on a missing module is not reported as "the code changed under it"), and a worker that dies early again and again with the same text stops the plan with that text instead of looping. */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createSlicePoolV1}from"../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import{createProcessWorkerV1,workerCauseV1}from"../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
import{HOST_POLICY_V1,renderTextV1}from"../../📨️protocol/🎚️policy/🟨️.mjs";
import{failureRefusalV1}from"../../🏛️service/🧯️failure/🟨️.mjs";
import corpus from"./🔣️.json" with {type:"json"};

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const MISSING="error: Cannot find module '/w/🏗️ifc/🦀️render.ts' from '/w/🏗️ifc/📁️generate.ts'";
type Step=Readonly<{exit:number|"done";tail?:string}>;
const harness=(steps:readonly Step[],restartPolicy:{backoffMilliseconds:number[];settleMilliseconds:number;progressMilliseconds:number;totalMaximum:number;sameCauseMaximum?:number}={backoffMilliseconds:[1],settleMilliseconds:60000,progressMilliseconds:60,totalMaximum:100,sameCauseMaximum:3})=>{
 const started:number[]=[],progress:any[]=[];let index=0;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,restartPolicy,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:()=>{const step=steps[index++]!,listeners:Record<string,(value:unknown)=>void>={};started.push(index);const end=()=>step.exit==="done"?listeners.message?.({type:"done",graph:{}}):listeners.exit?.(step.exit);queueMicrotask(end);return{pid:undefined,stderrTail:()=>step.tail??"",on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 return{pool,started,progress,run:(root:string)=>pool.run({root},{onProgress:(_ms:number,detail:any)=>progress.push(detail)})};
};

test("the cause of a worker is the line of its last words that names the error, without colour codes, and nothing when it said nothing",()=>{
 expect(workerCauseV1("")).toBe("");
 expect(workerCauseV1("\n  \n")).toBe("");
 expect(workerCauseV1(`some output\n\u001b[31m${MISSING}\u001b[0m\n\nBun v1.4.2 (macOS arm64)\n`)).toBe(MISSING);
 expect(workerCauseV1("first\nlast words without a keyword\n")).toBe("last words without a keyword");
 expect(workerCauseV1("error: "+"x".repeat(1000)).length).toBeLessThanOrEqual(300);
});

for(const vector of corpus)test("an ordinary slice exit preserves worker evidence "+(vector.cause||"when silent"),async()=>{
 const directory=mkdtempSync(join(tmpdir(),"slice-exit-")),logs:string[]=[],{pool,run,started}=harness([{exit:vector.exit,tail:vector.stderr}]);
 try{
  const error=await run(directory).then(()=>undefined,reason=>reason);
  const refusal=failureRefusalV1(error,{log:(_event:string,text:string)=>logs.push(text),logPath:"/fixture/host.log",environment:{SEMIO_FLEET_STORE_ROOT:directory,SEMIO_FLEET_ORIGIN:"fixture"}});
  const block=readFileSync(join(directory,"v1","origins","fixture","service-failure.txt"),"utf8");
  expect(error.exit).toBe(vector.exit);expect(error.restart).toBe(false);expect(error.message).toBe("Slice worker exited with code "+vector.exit);
  expect(started).toHaveLength(1);expect(refusal.key).toBe("internal");
  if(vector.cause){
   expect(refusal.fields.message).toContain(vector.cause);
   expect(logs[0]).toContain("worker cause: "+vector.cause);
   expect(block).toContain("worker cause: "+vector.cause);
  }else{
   expect(block).not.toContain("worker cause:");
   expect(refusal.fields.message).toBe("Error: Slice worker exited with code "+vector.exit);
  }
 }finally{await pool.close();rmSync(directory,{recursive:true,force:true});}
});

test("a restart carries the worker's own text and the English and German announcement shows it",async()=>{
 const{run,progress,started,pool}=harness([{exit:75,tail:`noise\n${MISSING}\n`},{exit:"done"}]);
 await run("/tmp/cause");await pool.close();
 expect(started.length).toBe(2);
 const decisions=progress.filter(detail=>detail?.decision?.key==="restarted-cause").map(detail=>detail.decision);
 expect(decisions.length).toBe(1);expect(decisions[0].cause).toBe(MISSING);expect(decisions[0].kind).toBe("75");
 const text=renderTextV1("host-slice-owners-restarted-cause",{seconds:7,phase:"reservation",done:0,total:0,owners:"",used:1,budget:900,limit:1,kind:"75",cause:MISSING},["en","de"])!;
 for(const language of["en","de"] as const){expect(text[language]).toContain(MISSING);expect(text[language]).not.toMatch(/\{[a-z]+\}/u);expect(text[language]).not.toContain("changed under");}
});

test("a worker that dies early again with the same text stops the plan with that text; different texts keep restarting",async()=>{
 const same=harness([{exit:75,tail:MISSING},{exit:75,tail:MISSING},{exit:75,tail:MISSING},{exit:"done"}]);
 const failure:any=await same.run("/tmp/same").then(()=>undefined,reason=>reason);await same.pool.close();
 expect(failure.code).toBe("slice-worker-failing");expect(failure.cause).toBe(MISSING);expect(failure.count).toBe(3);expect(failure.exit).toBe(75);
 expect(same.started.length).toBe(3);expect(failure.message).toContain(MISSING);expect(failure.message).toContain("Ausschnitt-Worker");
 const varied=harness([{exit:75,tail:"error: a"},{exit:75,tail:"error: b"},{exit:75,tail:"error: a"},{exit:75,tail:"error: b"},{exit:"done"}]);
 await varied.run("/tmp/varied");await varied.pool.close();expect(varied.started.length).toBe(5);
 const silent=harness([{exit:75},{exit:75},{exit:75},{exit:75},{exit:"done"}]);
 await silent.run("/tmp/silent");await silent.pool.close();expect(silent.started.length).toBe(5);
});

test("a worker that ran long before it died on the same missing module again and again stops the plan too, because the churn clock restarts with every long run",async()=>{
 const long=harness([{exit:75,tail:MISSING},{exit:75,tail:MISSING},{exit:75,tail:MISSING},{exit:"done"}],{backoffMilliseconds:[1],settleMilliseconds:60000,progressMilliseconds:0,totalMaximum:100,sameCauseMaximum:3});
 const failure:any=await long.run("/tmp/long").then(()=>undefined,reason=>reason);await long.pool.close();
 expect(failure.code).toBe("slice-worker-failing");expect(failure.cause).toBe(MISSING);expect(failure.count).toBe(3);expect(long.started.length).toBe(3);
});

test("the declared policy bounds the same-cause restarts at a small number",()=>{
 const maximum=(HOST_POLICY_V1 as unknown as{sliceRestartSameCauseMaximum:number}).sliceRestartSameCauseMaximum;
 expect(Number.isInteger(maximum)&&maximum>=2&&maximum<=5).toBe(true);
});

for(const exit of [1,75])test(`a real worker process with exit ${exit} leaves its message in the drained stderr tail`,async()=>{
 const directory=mkdtempSync(join(tmpdir(),"slice-cause-")),script=join(directory,"worker.mjs");
 try{
  writeFileSync(script,`process.stderr.write(${JSON.stringify(MISSING+"\n")});process.exit(${exit});`);
  const worker:any=createProcessWorkerV1(script,{},{...process.env});
  const code=await new Promise<number>(done=>worker.on("exit",done));
  expect(code).toBe(exit);expect(workerCauseV1(worker.stderrTail())).toBe(MISSING);
  await worker.terminate();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
