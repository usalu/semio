/** 🧱️ End to end: the real suite runner launches fixture files that hold memory; a file above its ceiling is stopped by name, and the gate keeps the megabytes of the running files inside the share whatever the worker count asks for. */
import{expect,test}from"bun:test";
import{appendFileSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{join,relative}from"node:path";
import{runProcessTestSuiteV1}from"../../../../🗂️suites/🏃️run/🟦️.ts";
import{createScriptProcessEnvelope,withScriptProcessEnvelope}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{testScratchRootV1}from"../../../../🏖️sandbox/🟦️.ts";
import{readTestWorkerPolicyV1}from"../../../🟦️.ts";
import corpus from"../../../🧪️tests/🧫️fixtures/🔣️.json";
import{readTestFileBudgetsV1}from"../../🟦️.ts";

const FILES=4,HOLD_MS=2500,policy=readTestWorkerPolicyV1({...corpus.policy,runners:corpus.policy.runners.map(row=>row.id==="bun-test"?{...row,baseMegabytes:64,workerMegabytes:100,wanted:1,maximum:6,oversubscription:1}:row)});
const plan=(memoryMegabytes:number)=>JSON.stringify({version:1,id:"k".repeat(22),jobs:1,cpu:8,memoryMegabytes,fixedMegabytes:1536,floorMegabytes:2048});
const fixtureSource=`const{appendFileSync,readFileSync}=require("node:fs");
test("holds memory while it runs",async()=>{
 const log=process.env.BUDGETS_FIXTURE_LOG,big=process.argv.join(" ").includes("file-0.ts");
 appendFileSync(log,"start "+process.pid+" "+Date.now()+"\\n");
 for(let waited=0;waited<60000;waited+=20){
  const lines=readFileSync(log,"utf8").split("\\n");
  if(lines.filter(line=>line.startsWith("start")).length-lines.filter(line=>line.startsWith("end")).length>=Number(process.env.BUDGETS_FIXTURE_BARRIER))break;
  await new Promise(done=>setTimeout(done,20));
 }
 const block=Buffer.alloc((big?Number(process.env.BUDGETS_FIXTURE_BIG_MB):40)*1048576,1);
 await new Promise(done=>setTimeout(done,Number(process.env.BUDGETS_FIXTURE_HOLD_MS)));
 appendFileSync(log,"end "+process.pid+" "+Date.now()+" "+block.length+"\\n");
});
`;
const overlap=(log:string):number=>{
 const events=log.trim().split("\n").filter(Boolean).map(line=>line.split(" ")).map(([kind,pid,at])=>({kind,pid,at:Number(at)})).sort((left,right)=>left.at-right.at||(left.kind==="end"?-1:1));
 let running=0,most=0;for(const event of events){running+=event.kind==="start"?1:-1;most=Math.max(most,running);}
 return most;
};

/** 🏃️ Runs the fixture suite through the real runner under one share (`budgets` are the declared files, keyed by the file names of the fixture); `barrier` files wait until that many run at once. */
const run=async(share:number,barrier:number,declared:Readonly<Record<string,number>>,extra:Record<string,string>={})=>{
 const root=mkdtempSync(join(testScratchRootV1("workers"),"capped-files-")),log=join(root,"events.log"),lines:string[]=[],errors:string[]=[];
 const files=Array.from({length:FILES},(_,index)=>`file-${index}.ts`);
 for(const file of files)writeFileSync(join(root,file),fixtureSource);
 writeFileSync(log,"");mkdirSync(join(root,"out"),{recursive:true});
 const budgets=readTestFileBudgetsV1({version:1,ceilingBasisPoints:15000,unlistedCeilingMegabytes:3072,files:Object.entries(declared).sort(([left],[right])=>left<right?-1:1).map(([path,megabytes])=>({path:relative(root,join(root,path)),megabytes,reason:"fixture"}))});
 const table={version:1 as const,suites:[{id:"budgeted",budgetMs:120000,typecheck:"none" as const,files,parallel:FILES}],codeSuites:[],helpers:[]};
 const environment={...process.env,SEMIO_HOST_CAPACITY_PLAN:plan(share),BUDGETS_FIXTURE_LOG:log,BUDGETS_FIXTURE_HOLD_MS:String(HOLD_MS),BUDGETS_FIXTURE_BARRIER:String(barrier),BUDGETS_FIXTURE_BIG_MB:"300",LANG:"en_US.UTF-8",...extra};
 const write=process.stderr.write.bind(process.stderr);let refusal:unknown;
 process.stderr.write=((chunk:string|Uint8Array,...rest:unknown[])=>{errors.push(String(chunk));return(write as (chunk:unknown,...rest:unknown[])=>boolean)(chunk,...rest);}) as typeof process.stderr.write;
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"workers",maximumElapsedMilliseconds:290000},{owner:"workers"},Date.now()),async invocation=>{
   try{await runProcessTestSuiteV1({invocation,root,checkout:root,table,suite:"budgeted",owner:"workers",output:join(root,"out"),environment,workers:{policy,budgets,intervalMilliseconds:100,write:line=>lines.push(line)}});}catch(error){refusal=error;}
  });
  return{concurrent:overlap(readFileSync(log,"utf8")),lines,errors:errors.join(""),refusal:refusal===undefined?undefined:String((refusal as Error).message),started:readFileSync(log,"utf8").split("\n").filter(line=>line.startsWith("start")).length};
 }finally{process.stderr.write=write;rmSync(root,{recursive:true,force:true});}
};

test("A file above its ceiling is stopped by name while its siblings finish, and the suite is refused naming that file",async()=>{
 const result=await run(4096,1,{"file-0.ts":100},{SEMIO_BUN_TEST_WORKERS:"1"});
 expect(result.refusal,"the suite is refused").toContain("file-0.ts");expect(result.refusal).not.toContain("file-1.ts");
 expect(result.errors).toMatch(/\[tests\] .*file-0\.ts held \d+ MB and was stopped at its ceiling of 150 MB \(100 MB declared in the test file budgets\)/u);
 expect(result.errors.match(/was stopped at its ceiling/gu)?.length,"exactly one file was stopped").toBe(1);
 expect(result.started,"the siblings of the stopped file still ran").toBe(FILES);
 const german=await run(4096,1,{"file-0.ts":100},{SEMIO_BUN_TEST_WORKERS:"1",LANG:"de_DE.UTF-8"});
 expect(german.errors).toMatch(/belegte \d+ MB und wurde an seiner Obergrenze von 150 MB angehalten/u);
 const within=await run(4096,1,{"file-0.ts":400},{SEMIO_BUN_TEST_WORKERS:"1"});
 expect(within.refusal,"the same file within its declared budget passes").toBeUndefined();expect(within.errors).not.toContain("stopped at its ceiling");
},300000);

test("The gate keeps the running files inside the share: four declared files of 400 MB under a 900 MB share run two at once although four workers are forced, and a roomy share runs all four",async()=>{
 const declared=Object.fromEntries(Array.from({length:FILES},(_,index)=>[`file-${index}.ts`,400])),two=await run(900,2,declared,{SEMIO_BUN_TEST_WORKERS:"4"});
 expect(two.refusal).toBeUndefined();expect(two.started).toBe(FILES);expect(two.concurrent,"900 MB carry two files of 400 MB").toBe(2);
 const all=await run(4096,FILES,declared,{SEMIO_BUN_TEST_WORKERS:"4"});
 expect(all.concurrent,"the cap was the gate, not the fixture").toBe(FILES);
},300000);
