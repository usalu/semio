import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {getEventListeners} from "node:events";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {observedFs,settleObservedComputationV1} from "../../🟨️.mjs";
import fixture from "./🔣️.json" with {type:"json"};

for(const row of fixture.cases)test("a real settlement backoff releases its abort listener on "+row.mode,async()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"backoff-")),path=join(directory,"source.txt");
 const controller=new AbortController(),reason=Object.freeze({case:row.mode}),first=new Error("fresh source"),db=new Database(":memory:");
 const keep=()=>{};
 controller.signal.addEventListener("abort",keep);
 const listeners=()=>getEventListeners(controller.signal,"abort");
 const baseline=listeners();
 const roundListeners:number[]=[];
 let computations=0,rounds=0;
 try{
  writeFileSync(path,fixture.file);
  if(row.mode==="pre-abort")controller.abort(reason);
  const result=settleObservedComputationV1(()=>{
   computations++;
   const text=observedFs.readFileSync(path,"utf8");
   if(computations===1)throw first;
   return text;
  },{signal:controller.signal,onRound(){rounds++;roundListeners.push(listeners().length);}}).then(value=>({value}),error=>({error}));
  if(row.mode==="abort"){
   const until=performance.now()+1000;
   while(listeners().length===baseline.length&&performance.now()<until)await Bun.sleep(1);
   expect(listeners().length).toBe(baseline.length+1);
   controller.abort(reason);
  }
  const settled=await result;
  const actual={computations,rounds,aborted:"error" in settled};
  if("error" in settled)expect(settled.error).toBe(reason);
  else expect(settled.value).toBe(fixture.file);
  expect(listeners()).toEqual(baseline);
  expect(roundListeners).toEqual(row.mode==="complete"?[baseline.length]:[]);
  expect(actual).toEqual(row.expected);
  const oracle=db.query<{computations:number;rounds:number;aborted:number},[string]>("SELECT CASE ?1 WHEN 'complete' THEN 2 WHEN 'abort' THEN 1 ELSE 0 END computations, ?1='complete' rounds, ?1<>'complete' aborted").get(row.mode)!;
  expect(actual).toEqual({...oracle,rounds:Number(oracle.rounds),aborted:Boolean(oracle.aborted)});
 }finally{db.close();rmSync(directory,{recursive:true,force:true});}
},7500);
