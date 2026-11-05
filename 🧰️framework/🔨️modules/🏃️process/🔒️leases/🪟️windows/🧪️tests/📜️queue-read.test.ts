import{test,expect}from"bun:test";
import{readFileSync,existsSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import Ajv from"ajv";
import*as api from"../🟦️.ts";
import corpus from"./📜️queue-read.json";
import schema from"../🧬️schema/📜️queue-read.json";
import{testPythonV1}from"../../../🧪️testing/🧰️toolchain/🟦️.ts";
const reader=api.readLeaseQueueTicketV1;
test("queue-owned read preserves predecessor custody through bounded sharing races",async()=>{
 expect(new Ajv().validate(schema,corpus)).toBe(true);
 for(const row of corpus.vectors){
  const errors=row.errors.map(code=>Object.assign(new Error(code),{code})),pauses:number[]=[];let calls=0,checks=0;
  const run=()=>reader("canonical-ticket",new AbortController().signal,()=>{checks++;},{platform:row.platform,read:(path:string)=>{expect(path).toBe("canonical-ticket");const error=errors[calls++];if(error)throw error;return corpus.value;},sleep:async(ms:number)=>{pauses.push(ms);}});
  if(row.expected==="error")await expect(run()).rejects.toBe(errors.at(-1));else expect(await run()).toBe(row.expected==="missing"?undefined:corpus.value);
  expect(calls).toBe(row.attempts);expect(pauses).toEqual(row.pauses);expect(checks).toBeGreaterThanOrEqual(calls);
 }
});
test("queue read never classifies control refusal or cancellation as a physical sharing race",async()=>{
 for(const row of corpus.controlRefusal.stages){
  const reason=Object.assign(new Error("original refusal"),{code:corpus.controlRefusal.code}),controller=new AbortController();let reads=0,sleeps=0,checks=0;
  await expect(reader("canonical-ticket",controller.signal,()=>{checks++;if(row.stage==="before-read"||(row.stage==="after-failure"||row.stage==="after-success")&&checks===2)throw reason;},{platform:"win32",read:()=>{reads++;if(row.stage==="after-success")return"owner";throw Object.assign(new Error("sharing"),{code:"EPERM"});},sleep:async(_ms:number,signal?:AbortSignal)=>{expect(signal).toBe(controller.signal);sleeps++;controller.abort(reason);}})).rejects.toBe(reason);
  expect(reads).toBe(row.reads);expect(sleeps).toBe(row.sleeps);
 }
});
test("queue reader agrees with independent Python filesystem and JSON observation",async()=>{
 const python=testPythonV1()??Bun.which("python3")??"python3",file=join(import.meta.dir,"📜️queue-read.json"),oracle=join(import.meta.dir,"🐍️oracle","📜️queue.py");
 for(const path of[file,file+".released"]){
  const independent=spawnSync(python,["-I",oracle,path],{encoding:"utf8"});expect(independent.status).toBe(0);const expected=JSON.parse(independent.stdout);let attempts=0;
  const actual=await reader(path,new AbortController().signal,()=>{},{platform:"win32",read:(name:string)=>{if(attempts++===0)throw Object.assign(new Error("sharing"),{code:"EPERM"});return readFileSync(name,"utf8");},sleep:async()=>{}});
  expect(actual===undefined?{exists:false}:{exists:true,value:JSON.parse(actual)}).toEqual(expected);expect(existsSync(file)).toBe(true);
 }
});
test("queue read preserves unusual physical refusal values without replacing them",async()=>{
 for(const reason of corpus.unknownFailures){
  let reads=0,sleeps=0;
  await expect(reader("canonical-ticket",new AbortController().signal,()=>{},{platform:"win32",read:()=>{reads++;throw reason;},sleep:async()=>{sleeps++;}})).rejects.toBe(reason);
  expect(reads).toBe(1);expect(sleeps).toBe(0);
 }
});
