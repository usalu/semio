import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,readdirSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {acquireQueuedResourceLease,acquireResourceLease,resourceLeasePathV1,resourceQueueClaimV1,resourceQueueTicketV1} from "../../🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../🏗️build/🗣️language/🚨️error/🟦️.ts";
import policySchema from "../../⏳️wait/🧬️schema/📜️policy.json";

/** 🪦️ Laws of the directory admission under abandoned queue tickets: a waiter reclaims thousands of tickets of dead requesters in bounded slices, a timeout of the admission is a typed bilingual refusal, a dead admission holder is released by the kernel at once and a live slow holder is waited for and named. Independent oracle: a Python `sqlite3` process proves the claim locks. */
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output)throw Error("Explicit ticket generated artifact directory required");
const admission="resource-directory-admission:v1",fresh=():string=>{const root=resolve(output);mkdirSync(root,{recursive:true});return mkdtempSync(join(root,"sq-"));};
const forge=async(directory:string,resource:string,count:number,deadPid:number):Promise<string>=>{
 const queue=join(directory,createHash("sha256").update(resource).digest("hex")+".queue"),signal=new AbortController().signal,base=Date.now()-86400000;mkdirSync(queue,{recursive:true});
 for(let at=0;at<count;at++){const ticket=resourceQueueTicketV1({arrivedAtMs:base+at,pid:deadPid,owner:"dead"+at});
  const claim=await acquireResourceLease({...resourceQueueClaimV1(directory,resource,ticket),mode:"shared",signal,timeoutMs:1000,maximumDatabaseBytes:16384});claim.release();
  writeFileSync(join(queue,ticket),JSON.stringify({version:1,pid:deadPid,mode:"exclusive",sharedCohort:false}));}
 return queue;
};
const deadPid=():number=>{const child=Bun.spawnSync([process.execPath,"-e","0"]);return child.pid??0;};
const child=(code:string,environment:Record<string,string>)=>Bun.spawn([process.execPath,"-e",code],{stdout:"pipe",stderr:"pipe",env:{...process.env,...environment}});
const ready=async(process_:ReturnType<typeof child>):Promise<void>=>{const reader=process_.stdout.getReader();const {value}=await reader.read();const text=new TextDecoder().decode(value);if(!text.includes("held"))throw Error("holder failed: "+await new Response(process_.stderr).text());reader.releaseLock();};
const holder=(directory:string)=>`const {Database}=await import("bun:sqlite");const api=await import(${JSON.stringify(process.env.SEMIO_TEST_LEASES_MODULE??import.meta.resolve("../../🟦️.ts"))});const control=new AbortController();await api.acquireResourceLease({directory:${JSON.stringify(directory)},resource:"r:registered",mode:"shared",signal:control.signal});const db=new Database(${JSON.stringify(resourceLeasePathV1(directory,admission))});db.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");console.log("held");await new Promise(()=>{});`;

test("the wait policy declares the reap slice and validates against its schema",()=>{
 const validate=new Ajv({strict:true}).compile({$schema:policySchema.$schema,definitions:policySchema.definitions,$ref:"#/definitions/WaitPolicy"});
 expect(validate(policySchema.definitions.Policy.const)).toBe(true);expect(policySchema.definitions.Policy.const.reapSliceTickets).toBe(256);
 expect(validate({...policySchema.definitions.Policy.const,reapSliceTickets:0})).toBe(false);
});

test("twenty thousand abandoned tickets of dead requesters never delay the waiter and are reclaimed in bounded slices, one per acquisition",async()=>{
 const directory=fresh(),resource="r:stale",count=20000,queue=await forge(directory,resource,count,deadPid());
 expect(readdirSync(queue).length).toBe(count);
 const signal=new AbortController().signal;
 const began=Date.now(),lease=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"waiter",signal,timeoutMs:20000}),elapsed=Date.now()-began;lease.release();
 expect(elapsed).toBeLessThan(5000);
 const afterFirst=readdirSync(queue).length;expect(afterFirst).toBeLessThan(count);
 let slowest=0,stop=false,previous=afterFirst,slowestCycle=0;
 const probe=(async()=>{while(!stop){const at=Date.now(),other=await acquireResourceLease({directory,resource:"r:other",mode:"shared",signal,timeoutMs:20000});other.release();slowest=Math.max(slowest,Date.now()-at);await new Promise(done=>setTimeout(done,20));}})();
 for(let cycle=0;cycle<15;cycle++){const at=Date.now(),next=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"cycle"+cycle,signal,timeoutMs:20000});next.release();slowestCycle=Math.max(slowestCycle,Date.now()-at);const left=readdirSync(queue).length;expect(left).toBeLessThan(previous);previous=left;}
 stop=true;await probe;
 expect(previous).toBeLessThanOrEqual(afterFirst-15*30);expect(slowest).toBeLessThan(2000);expect(slowestCycle).toBeLessThan(5000);
 const claims=readdirSync(join(directory,"claims")).filter(name=>name.endsWith(".sqlite")).slice(0,40).map(name=>join(directory,"claims",name));
 const python=Bun.spawnSync(["python3","-I","-c","import sqlite3,sys\nfor p in sys.argv[1:]:\n c=sqlite3.connect(p,timeout=0,isolation_level=None)\n c.execute('BEGIN EXCLUSIVE')\n c.execute('ROLLBACK')\n c.close()\nprint('free')",...claims]);
 expect(new TextDecoder().decode(python.stdout).trim()).toBe("free");
},178000);

test("a budget that ends inside the reclaim is the timeout of the real resource or the typed refusal, never an untyped admission error",async()=>{
 const directory=fresh(),resource="r:budget",queue=await forge(directory,resource,3000,deadPid()),signal=new AbortController().signal;
 const blocker=await acquireResourceLease({directory,resource,mode:"exclusive",signal,timeoutMs:1000});
 try{await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"short",signal,timeoutMs:1500});throw Error("acquired behind a holder");}
 catch(error){
  if(isRepoErrorV1(error)){expect(error.code).toBe("timeout");expect(error.key).toBe("process.leases.admission-timeout");}
  else{expect(error).toBeInstanceOf(Error);expect((error as Error).message).toBe("Resource lease timed out: "+resource);}
 }finally{blocker.release();}
 expect(readdirSync(queue).length).toBeLessThan(3000);
},60000);

test("a dead holder of the directory admission is released by the kernel and the waiter acquires at once",async()=>{
 const directory=fresh(),signal=new AbortController().signal;
 (await acquireResourceLease({directory,resource:"r:seed",mode:"shared",signal})).release();
 const holding=child(holder(directory),{SEMIO_LEASE_REGISTRY_DIRECTORY:process.env.SEMIO_LEASE_REGISTRY_DIRECTORY!});await ready(holding);
 setTimeout(()=>holding.kill("SIGKILL"),300);const began=Date.now();
 const lease=await acquireQueuedResourceLease({directory,resource:"r:after",mode:"exclusive",owner:"after",signal,timeoutMs:20000});lease.release();
 expect(Date.now()-began).toBeLessThan(8000);
},60000);

test("a live slow holder of the directory admission is waited for, then refused with a typed bilingual refusal naming the directory, its size and the holder",async()=>{
 const directory=fresh(),signal=new AbortController().signal;
 (await acquireResourceLease({directory,resource:"r:seed",mode:"shared",signal})).release();
 const holding=child(holder(directory),{SEMIO_LEASE_REGISTRY_DIRECTORY:process.env.SEMIO_LEASE_REGISTRY_DIRECTORY!});await ready(holding);
 try{
  const began=Date.now();let failure:unknown;
  try{await acquireQueuedResourceLease({directory,resource:"r:blocked",mode:"exclusive",owner:"blocked",signal,timeoutMs:1500});}catch(error){failure=error;}
  expect(Date.now()-began).toBeGreaterThanOrEqual(1400);
  expect(isRepoErrorV1(failure)).toBe(true);
  const refusal=failure as import("../../../🏗️build/🗣️language/🚨️error/🟦️.ts").RepoErrorV1;
  expect(refusal.code).toBe("timeout");expect(refusal.key).toBe("process.leases.admission-timeout");
  expect(refusal.message.startsWith("Resource lease timed out: "+admission)).toBe(true);
  expect(refusal.values.directory).toBe(resolve(directory));expect(Number(refusal.values.entries)).toBeGreaterThan(0);expect(String(refusal.values.holders)).toContain(String(holding.pid));
  for(const language of ["en","de"]){const text=renderRepoErrorV1(refusal,language);expect(text).toContain(resolve(directory));expect(text).toContain(String(holding.pid));expect(text).not.toMatch(/\{[a-z]+\}/u);}
  expect(renderRepoErrorV1(refusal,"de")).toContain("Verzeichniszulassung");
 }finally{holding.kill("SIGKILL");}
},60000);
