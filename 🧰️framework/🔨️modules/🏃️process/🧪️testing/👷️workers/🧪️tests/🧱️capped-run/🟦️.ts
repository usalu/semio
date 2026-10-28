/** 🧱️ Tests for testing workers capped-run. */
import{expect,test}from"bun:test";
import{appendFileSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{runProcessTestSuiteV1}from"../../../🗂️suites/🏃️run/🟦️.ts";
import{createScriptProcessEnvelope,withScriptProcessEnvelope}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{observeTreeMegabytesV1}from"../../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import{testScratchRootV1}from"../../../🏖️sandbox/🟦️.ts";
import{readTestWorkerPolicyV1}from"../../🟦️.ts";
import corpus from"../🧫️fixtures/🔣️.json";

const FILES=6,HOLD_MS=1500,ALLOCATION_MB=120,policy=readTestWorkerPolicyV1({...corpus.policy,runners:corpus.policy.runners.map(row=>row.id==="bun-test"?{...row,baseMegabytes:64,workerMegabytes:300,wanted:1,maximum:6,oversubscription:1}:row)});
const plan=(jobs:number,cpu:number,memoryMegabytes:number)=>JSON.stringify({version:1,id:"k".repeat(22),jobs,cpu,memoryMegabytes,fixedMegabytes:1536,floorMegabytes:2048});
const fixtureSource=`const{appendFileSync,readFileSync}=require("node:fs");
test("holds memory while it runs",async()=>{
 const log=process.env.WORKERS_FIXTURE_LOG;
 appendFileSync(log,"start "+process.pid+" "+Date.now()+"\\n");
 for(let waited=0;waited<60000;waited+=20){
  const lines=readFileSync(log,"utf8").split("\\n");
  if(lines.filter(line=>line.startsWith("start")).length-lines.filter(line=>line.startsWith("end")).length>=Number(process.env.WORKERS_FIXTURE_BARRIER))break;
  await new Promise(done=>setTimeout(done,20));
 }
 const block=Buffer.alloc(Number(process.env.WORKERS_FIXTURE_MB)*1048576,1);
 await new Promise(done=>setTimeout(done,Number(process.env.WORKERS_FIXTURE_HOLD_MS)));
 appendFileSync(log,"end "+process.pid+" "+Date.now()+" "+block.length+"\\n");
});
`;

/** 🔢️ The most files that were running at the same moment, read from the start and end lines the fixture files wrote. */
const overlap=(log:string):number=>{
 const events=log.trim().split("\n").map(line=>line.split(" ")).map(([kind,pid,at])=>({kind,pid,at:Number(at)})).sort((left,right)=>left.at-right.at||(left.kind==="end"?-1:1));
 let running=0,most=0;for(const event of events){running+=event.kind==="start"?1:-1;most=Math.max(most,running);}
 return most;
};

/** 🏃️ Runs the fixture suite through the real suite runner under one admitted share and watches the tree; every file waits for `barrier` files to run at once (at most a minute), so a slow start of a process cannot hide the concurrency the runner allowed. */
const run=async(grant:string,barrier:number,extra:Record<string,string>={})=>{
 const root=mkdtempSync(join(testScratchRootV1("workers"),"capped-run-")),log=join(root,"events.log"),lines:string[]=[];
 const files=Array.from({length:FILES},(_,index)=>`file-${index}.ts`);
 for(const file of files)writeFileSync(join(root,file),fixtureSource);
 writeFileSync(log,"");mkdirSync(join(root,"out"),{recursive:true});
 const table={version:1 as const,suites:[{id:"capped",budgetMs:120000,typecheck:"none" as const,files,parallel:FILES}],codeSuites:[],helpers:[]};
 const environment={...process.env,SEMIO_HOST_CAPACITY_PLAN:grant,WORKERS_FIXTURE_LOG:log,WORKERS_FIXTURE_MB:String(ALLOCATION_MB),WORKERS_FIXTURE_HOLD_MS:String(HOLD_MS),WORKERS_FIXTURE_BARRIER:String(barrier),LANG:"en_US.UTF-8",...extra};
 let peak=0,baseline=0;
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"workers",maximumElapsedMilliseconds:290000},{owner:"workers"},Date.now()),async invocation=>{
   baseline=(await observeTreeMegabytesV1(process.pid))??0;
   const timer=setInterval(()=>{void observeTreeMegabytesV1(process.pid).then(megabytes=>{if(megabytes!==null)peak=Math.max(peak,megabytes);});},100);
   try{await runProcessTestSuiteV1({invocation,root,checkout:root,table,suite:"capped",owner:"workers",output:join(root,"out"),environment,workers:{policy,write:line=>lines.push(line)}});}finally{clearInterval(timer);}
  });
  return{concurrent:overlap(readFileSync(log,"utf8")),grown:peak-baseline,lines,started:readFileSync(log,"utf8").split("\n").filter(line=>line.startsWith("start")).length};
 }finally{rmSync(root,{recursive:true,force:true});}
};

test("End to end: a suite launched under a small grant never runs more files at once than the share allows and its process tree stays inside the share; a roomy grant and an override show the cap is what held it back",async()=>{
 const share=800,small=await run(plan(1,2,share),2);
 expect(small.started).toBe(FILES);
 expect(small.concurrent,"two workers fit the 800 MB share and two cores").toBe(2);
 expect(small.grown,"the tree grew by at most the share").toBeLessThanOrEqual(share);
 expect(small.grown,"the files really held memory").toBeGreaterThan(ALLOCATION_MB);
 expect(small.lines).toEqual(["[tests] bun test: 2 of 6 worker(s), bounded by the admitted memory (800 MB and 2 core(s) of the admitted grant, about 300 MB per worker)"]);
 const roomy=await run(plan(1,8,4096),FILES);
 expect(roomy.concurrent,"the same suite under a roomy grant runs all its files at once").toBe(FILES);
 expect(roomy.lines).toEqual([]);
 expect(roomy.grown).toBeGreaterThan(small.grown);
 const one=await run(plan(1,8,4096),1,{SEMIO_BUN_TEST_WORKERS:"1"});
 expect(one.concurrent).toBe(1);
 const forced=await run(plan(1,2,share),2,{SEMIO_BUN_TEST_WORKERS:"6"});
 expect(forced.concurrent,"an override never leaves the admitted cores").toBe(2);
},300000);
