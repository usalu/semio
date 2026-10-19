import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,readdirSync,rmSync} from "node:fs";
import {join,resolve,relative,sep} from "node:path";
import {getEventListeners} from "node:events";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {acquireResourceLease,acquireQueuedResourceLease,resourceLeasePathV1,type ResourceLeaseControl} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** ⏱️ Observes settlement within the neutral test ceiling and removes the observation timer. */
async function bounded<T>(pending:Promise<T>,milliseconds:number):Promise<T>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([pending,new Promise<never>((_,reject)=>{timer=setTimeout(()=>reject(Error("Lease wait failed to settle")),milliseconds);})]);}
 finally{if(timer!==undefined)clearTimeout(timer);}
}

const validate=new Ajv({strict:true}).compile(schema);
test("lease wait fixtures declare every original observer halt",()=>{
 expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:true})).toBe(false);
 expect(validate({...fixture,cases:[{...fixture.cases[0],halt:"renew"}]})).toBe(false);
});

for(const row of fixture.cases)test(`original resource observer ${row.id}`,async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!output)throw Error("Explicit ticket generated artifact directory required");
 const root=resolve(output);mkdirSync(root,{recursive:true});const directory=mkdtempSync(join(root,"lw-"));
 const controller=new AbortController(),reason=Error("original lease wait cancelled"),consumerError=Error("original progress consumer failed"),resource="original-wait";
 const holder=await acquireResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal});
 let enter!:()=>void,release!:()=>void,acquired=false,continuations=0;
 const entered=new Promise<void>(resolve=>{enter=resolve;}),held=new Promise<void>(resolve=>{release=resolve;});
 const stop=()=>{enter();if(row.halt==="reject")throw consumerError;return held;};
 const started=Date.now();
 const control:ResourceLeaseControl={signal:controller.signal,remainingMilliseconds:()=>row.halt==="deadline"?fixture.deadlineMilliseconds-(Date.now()-started):10000-(Date.now()-started),publish:()=>row.phase==="publish"?stop():undefined,yieldContinuation:()=>{continuations++;return row.phase==="yield"?stop():Promise.resolve();}};
 const options={directory,resource,mode:"shared" as const,signal:controller.signal,...(row.phase!=="onWait"?{control}:{}),...(row.halt==="timeout"?{timeoutMs:fixture.deadlineMilliseconds}:{}),onWait:row.phase==="onWait"?stop:()=>{}};
 const pending=(row.api==="queued"?acquireQueuedResourceLease({...options,owner:"observer-waiter"}):acquireResourceLease(options)).then(lease=>{acquired=true;lease.release();return{error:undefined};},error=>({error}));
 try{
  await bounded(entered,fixture.observationMilliseconds);
  if(row.halt==="cancel")controller.abort(reason);
  const result=await bounded(pending,fixture.observationMilliseconds);
  expect(acquired).toBe(row.waiterAcquired);
  if(row.halt==="cancel")expect(result.error).toBe(reason);
  else if(row.halt==="reject")expect(result.error).toBe(consumerError);
  else expect(String(result.error)).toMatch(row.halt==="deadline"?/Original compiler admission deadline exhausted/:/Resource lease timed out/);
  expect(getEventListeners(controller.signal,"abort")).toHaveLength(0);
  if(row.phase==="yield")expect(continuations).toBe(1);
 }finally{
  controller.abort(reason);release();await pending;holder.release();
  const followup=await acquireResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,timeoutMs:fixture.observationMilliseconds});followup.release();
  const db=new Database(resourceLeasePathV1(directory,resource),{readonly:true});try{expect(db.query<{resource:string},[]>("SELECT resource FROM semio_resource_lease WHERE id=1").get()?.resource).toBe(resource);}finally{db.close();}
  for(const name of readdirSync(directory).filter(name=>name.endsWith(".queue")))expect(readdirSync(join(directory,name))).toEqual([]);
  const owned=relative(root,directory);if(!owned.startsWith("lw-")||owned.includes(sep)||resolve(root,owned)!==directory)throw Error("Owned ticket lease fixture directory required");
  rmSync(directory,{recursive:true});
 }
 console.log(`[DEBUG] Original SQLite lease ${row.id} settled without observer completion, acquired=${acquired}, exact signal/rejection and cleanup preserved`);
});
