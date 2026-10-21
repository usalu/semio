import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,acquireResourceLease,LeaseOrderErrorV1,leaseInheritanceEnvironmentV1,resourceLeasePathV1,withLeaseTaskV1,withResourceLeases} from "../../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import statusSchema from "../../../📋️status/🧬️schema/🔣️.json";
import ticketSchema from "../../../🧬️schema/📜️ticket.json";
import {runLeasesCommandV1} from "../../../📋️status/🟦️.ts";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"lo-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const worker=resolve(import.meta.dir,"../📥️worker/🟦️.ts");
const admissionResource=ticketSchema.definitions.RetirementPolicy.properties.admissionResource.const;

let counter=0;
function scenario(policy:Record<string,unknown>={}){
 const directory=join(root,`d${++counter}`),registry=join(root,`r${counter}`);mkdirSync(directory,{recursive:true});mkdirSync(registry,{recursive:true});
 const authored={...fixture.policy,...policy},file=join(root,`p${counter}.json`);writeFileSync(file,JSON.stringify(authored));
 return {directory,registry,env:{...process.env,SEMIO_LEASE_ORDER_POLICY:file,SEMIO_LEASE_REGISTRY_DIRECTORY:registry,SEMIO_LEASE_ORDER:"",SEMIO_LANGUAGE:"en"} as Record<string,string>};
}
const configure=(env:Record<string,string>):void=>{for(const [key,value] of Object.entries(env))if(key.startsWith("SEMIO_LEASE_")||key==="SEMIO_LANGUAGE")process.env[key]=value;};

type Child={pid:number;send(text:string):void;next(prefix:string,milliseconds?:number):Promise<string>;stop():void;done:Promise<number>};
function start(spec:Record<string,unknown>,env:Record<string,string>):Child{
 const process_=Bun.spawn([process.execPath,worker,JSON.stringify(spec)],{stdin:"pipe",stdout:"pipe",stderr:"pipe",env});
 const lines:string[]=[],waiting:{prefix:string;resolve:(line:string)=>void}[]=[];let buffer="";
 const push=(line:string)=>{lines.push(line);for(const entry of [...waiting])if(line.startsWith(entry.prefix)){waiting.splice(waiting.indexOf(entry),1);entry.resolve(line);}};
 (async()=>{const decoder=new TextDecoder();for await(const chunk of process_.stdout){buffer+=decoder.decode(chunk);for(let at=buffer.indexOf("\n");at>=0;at=buffer.indexOf("\n")){push(buffer.slice(0,at));buffer=buffer.slice(at+1);}}})();
 const errors:string[]=[];(async()=>{for await(const chunk of process_.stderr)errors.push(new TextDecoder().decode(chunk));})();
 return {pid:process_.pid,send:text=>{process_.stdin.write(text+"\n");process_.stdin.flush();},stop:()=>{try{process_.kill();}catch{}},done:process_.exited,
  next:(prefix,milliseconds=15000)=>{const found=lines.find(line=>line.startsWith(prefix));if(found)return Promise.resolve(found);return new Promise<string>((resolve,reject)=>{waiting.push({prefix,resolve});setTimeout(()=>reject(Error(`child ${process_.pid} never printed ${prefix}; stderr: ${errors.join("").slice(0,400)}`)),milliseconds).unref();});}};
}
const signal=()=>new AbortController().signal;
const tick=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("an out-of-order request fails fast with a localized diagnostic and leaves the held lease intact",async()=>{
 const {directory,env}=scenario();configure(env);
 const task=signal(),held=await acquireResourceLease({directory,resource:"inner:b",mode:"exclusive",signal:task});
 const first=Date.now();
 const error=await acquireResourceLease({directory,resource:"outer:a",mode:"exclusive",signal:task}).then(()=>undefined,error=>error);
 expect(Date.now()-first).toBeLessThan(1000);
 expect(error).toBeInstanceOf(LeaseOrderErrorV1);
 expect(error.violation).toMatchObject({kind:"out-of-order",held:"inner:b",requested:"outer:a",heldClass:"inner",requestedClass:"outer"});
 expect(error.message).toContain("Lock order violated");
 process.env.SEMIO_LANGUAGE="de";
 const german=await acquireResourceLease({directory,resource:"outer:a",mode:"exclusive",signal:task}).then(()=>undefined,error=>error);
 expect(german.message).toContain("Sperrreihenfolge verletzt");
 held.release();
 const again=await acquireResourceLease({directory,resource:"outer:a",mode:"exclusive",signal:signal()});again.release();
});

test("observe mode records the violation and still grants the lease; off records nothing",async()=>{
 const {directory,env,registry}=scenario({rules:{outOfOrder:"observe",holdAndWait:"enforce",selfDeadlock:"enforce",cycle:"enforce"}});configure(env);
 const task=signal(),held=await acquireResourceLease({directory,resource:"inner:b",mode:"shared",signal:task});
 const granted=await acquireResourceLease({directory,resource:"outer:a",mode:"shared",signal:task});granted.release();held.release();
 const folder=readdirSync(registry).find(name=>readdirSync(join(registry,name)).includes("violations.jsonl"))!;
 const rows=readFileSync(join(registry,folder,"violations.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(rows).toHaveLength(1);expect(rows[0]).toMatchObject({kind:"out-of-order",enforced:false,requested:"outer:a"});
 process.env.SEMIO_LEASE_ORDER="off";
 const quiet=await acquireResourceLease({directory,resource:"inner:b2",mode:"shared",signal:task}),quiet2=await acquireResourceLease({directory,resource:"outer:a2",mode:"shared",signal:task});quiet2.release();quiet.release();
 expect(readFileSync(join(registry,folder,"violations.jsonl"),"utf8").trim().split("\n")).toHaveLength(1);
});

test("a task that holds a lease exclusively and waits for a busy one is refused after the grace period, names the blocker, and never blocks unrelated leases",async()=>{
 const {directory,env}=scenario();configure(env);
 const holder=start({mode:"hold",directory,resource:"inner:busy",access:"exclusive"},env);await holder.next("HELD");
 const task=signal(),mine=await acquireResourceLease({directory,resource:"outer:a",mode:"exclusive",signal:task});
 const started=Date.now(),waiting=acquireResourceLease({directory,resource:"inner:busy",mode:"exclusive",signal:task}).then(()=>undefined,error=>error);
 await tick(300);
 const unrelated=Date.now(),other=await acquireResourceLease({directory,resource:"free:unrelated",mode:"exclusive",signal:signal()});other.release();
 expect(Date.now()-unrelated).toBeLessThan(500);
 const error=await waiting,elapsed=Date.now()-started;
 expect(error).toBeInstanceOf(LeaseOrderErrorV1);
 expect(error.violation).toMatchObject({kind:"hold-and-wait",held:"outer:a",requested:"inner:busy",blockerPid:holder.pid});
 expect(error.message).toContain(`process ${holder.pid}`);
 expect(elapsed).toBeGreaterThanOrEqual(900);expect(elapsed).toBeLessThan(5000);
 mine.release();holder.send("release");await holder.next("DONE");
});

test("waiting while holding only shared leases, or holding nothing, is never refused",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:200,pollMilliseconds:50}});configure(env);
 const holder=start({mode:"hold",directory,resource:"inner:busy",access:"exclusive"},env);await holder.next("HELD");
 const task=signal(),shared=await acquireResourceLease({directory,resource:"outer:a",mode:"shared",signal:task});
 const waiting=acquireResourceLease({directory,resource:"inner:busy",mode:"shared",signal:task});
 await tick(700);holder.send("release");const lease=await waiting;lease.release();shared.release();await holder.next("DONE");
});

test("two processes that each hold one lease and wait for the other's are detected as a deadlock; exactly one fails fast and the other proceeds",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});
 const left=start({mode:"chain",directory,first:{resource:"free:A",access:"exclusive"},second:{resource:"free:B",access:"exclusive"}},env),right=start({mode:"chain",directory,first:{resource:"free:B",access:"exclusive"},second:{resource:"free:A",access:"exclusive"}},env);
 await left.next("HELD");await right.next("HELD");
 left.send("go");await tick(120);right.send("go");
 const results=await Promise.all([left.next("RESULT"),right.next("RESULT")]).then(rows=>rows.map(row=>JSON.parse(row.slice(7))));
 const failed=results.filter(row=>!row.ok),granted=results.filter(row=>row.ok);
 expect(failed).toHaveLength(1);expect(granted).toHaveLength(1);
 expect(failed[0]).toMatchObject({code:"SEMIO_LEASE_ORDER",kind:"cycle"});expect(failed[0].message).toContain("Deadlock detected");expect(failed[0].elapsedMs).toBeLessThan(5000);
 expect(failed[0].message).toContain("→");
 left.send("release");right.send("release");await left.done;await right.done;
});

test("a deadlock message is German when the language says so",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});env.SEMIO_LANGUAGE="de";
 const left=start({mode:"chain",directory,first:{resource:"free:A",access:"exclusive"},second:{resource:"free:B",access:"exclusive"}},env),right=start({mode:"chain",directory,first:{resource:"free:B",access:"exclusive"},second:{resource:"free:A",access:"exclusive"}},env);
 await left.next("HELD");await right.next("HELD");left.send("go");await tick(120);right.send("go");
 const results=await Promise.all([left.next("RESULT"),right.next("RESULT")]).then(rows=>rows.map(row=>JSON.parse(row.slice(7))));
 expect(results.find(row=>!row.ok)?.message).toContain("Verklemmung erkannt");
 left.send("release");right.send("release");await left.done;await right.done;
});

test("queued requests that close a cycle are detected too",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});
 const left=start({mode:"chain",queued:true,directory,first:{resource:"free:A",access:"exclusive"},second:{resource:"free:B",access:"exclusive"}},env),right=start({mode:"chain",queued:true,directory,first:{resource:"free:B",access:"exclusive"},second:{resource:"free:A",access:"exclusive"}},env);
 await left.next("HELD");await right.next("HELD");left.send("go");await tick(150);right.send("go");
 const results=await Promise.all([left.next("RESULT",20000),right.next("RESULT",20000)]).then(rows=>rows.map(row=>JSON.parse(row.slice(7))));
 expect(results.filter(row=>!row.ok)).toHaveLength(1);expect(results.filter(row=>row.ok)).toHaveLength(1);
 left.send("release");right.send("release");await left.done;await right.done;
});

test("a waiter does not hold the directory admission gate while it waits, so openers and retirement are never blocked behind it; the old behaviour is the negative control",async()=>{
 const probe=async(releaseWhileWaiting:boolean):Promise<number>=>{
  const {directory,env}=scenario({gate:{releaseWhileWaiting,pollInitialMilliseconds:20,pollMaximumMilliseconds:40}});configure(env);
  const holder=start({mode:"hold",directory,resource:"free:R",access:"exclusive"},env);await holder.next("HELD");
  const waiting=acquireResourceLease({directory,resource:"free:R",mode:"exclusive",signal:signal(),timeoutMs:2500}).then(()=>"acquired",error=>String(error));
  await tick(300);
  let granted=0;const gate=resourceLeasePathV1(directory,admissionResource);
  for(let round=0;round<60;round++){
   const database=new Database(gate);
   try{database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");database.exec("ROLLBACK");granted++;}catch{}finally{database.close();}
   await tick(9);
  }
  expect(await waiting).toContain("timed out");holder.send("release");await holder.next("DONE");return granted;
 };
 expect(await probe(true)).toBeGreaterThan(10);
 expect(await probe(false)).toBe(0);
});

test("fair readers: shared requests through withResourceLeases no longer starve an exclusive request; raw polling is the negative control",async()=>{
 const run=async(queued:boolean):Promise<{acquired:boolean;elapsed:number}>=>{
  const {directory,env}=scenario({fairness:{queuedWithResourceLeases:queued}});configure(env);
  const readers=[0,50,100].map(offsetMs=>start({mode:"readers",directory,resource:"free:W",holdMs:150,durationMs:6000,offsetMs},env));
  for(const reader of readers)await reader.next("READY");
  await tick(300);
  const started=Date.now();
  const outcome=await withResourceLeases({directory,signal:signal(),timeoutMs:3500,resources:[{resource:"free:W",mode:"exclusive"}]},async()=>true).then(()=>true,()=>false);
  const elapsed=Date.now()-started;
  for(const reader of readers)reader.stop();
  await Promise.all(readers.map(reader=>reader.done));
  return {acquired:outcome,elapsed};
 };
 const fair=await run(true);expect(fair.acquired).toBe(true);expect(fair.elapsed).toBeLessThan(2500);
 const raw=await run(false);expect(raw.acquired).toBe(false);
});

test("a task re-entering a shared lease it already holds bypasses the queue, so a waiting writer cannot deadlock it",async()=>{
 const {directory,env}=scenario();configure(env);
 const outerSignal=signal(),outer=await withResourceLeases({directory,signal:outerSignal,resources:[{resource:"free:Y",mode:"shared"}]},async()=>{
  const writer=withResourceLeases({directory,signal:signal(),timeoutMs:8000,resources:[{resource:"free:Y",mode:"exclusive"}]},async()=>"written");
  await tick(300);
  const nested=await withResourceLeases({directory,signal:outerSignal,timeoutMs:2000,resources:[{resource:"free:Y",mode:"shared"}]},async()=>"nested");
  return {nested,writer};
 });
 expect(outer.nested).toBe("nested");expect(await outer.writer).toBe("written");
});

test("a lease taken exclusively and requested again shared by the same task is refused after the grace period instead of waiting for itself forever",async()=>{
 const {directory,env}=scenario();configure(env);
 const own=signal(),held=await acquireResourceLease({directory,resource:"free:S",mode:"exclusive",signal:own});
 const started=Date.now(),error=await acquireResourceLease({directory,resource:"free:S",mode:"shared",signal:own}).then(()=>undefined,error=>error);
 expect(error).toBeInstanceOf(LeaseOrderErrorV1);expect(error.violation.kind).toBe("self-deadlock");expect(Date.now()-started).toBeGreaterThanOrEqual(900);
 held.release();
});

test("concurrent sibling tasks of one process wait for each other's lease like separate processes do: no self-deadlock, even beyond the grace period; without the task scopes the same shape is one task",async()=>{
 const {directory,env}=scenario();configure(env);
 const shared=signal();
 const sibling=async(milliseconds:number):Promise<number>=>withLeaseTaskV1(async()=>{
  const started=Date.now(),lease=await acquireQueuedResourceLease({directory,resource:"free:G",mode:"exclusive",owner:"t"+Math.random().toString(36).slice(2,8),signal:shared});
  const waited=Date.now()-started;await tick(milliseconds);lease.release();return waited;
 });
 const [first,second]=await Promise.all([sibling(1500),(async()=>{await tick(100);return sibling(100);})()]);
 expect(first).toBeLessThan(500);expect(second).toBeGreaterThanOrEqual(1200);
 const holder=await acquireQueuedResourceLease({directory,resource:"free:G",mode:"exclusive",owner:"unscoped",signal:shared});
 const refused=await acquireQueuedResourceLease({directory,resource:"free:G",mode:"exclusive",owner:"unscoped2",signal:shared}).then(()=>undefined,error=>error);
 expect(refused?.violation?.kind).toBe("self-deadlock");holder.release();
});

test("a nested task sees what its ancestors hold but not what a sibling holds",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:300,pollMilliseconds:50}});configure(env);
 const signals=signal();
 await withLeaseTaskV1(async()=>{
  const parent=await acquireResourceLease({directory,resource:"free:P",mode:"exclusive",signal:signals});
  const child=await withLeaseTaskV1(()=>acquireResourceLease({directory,resource:"free:P",mode:"exclusive",signal:signals}).then(()=>undefined,error=>error));
  expect(child?.violation?.kind).toBe("self-deadlock");
  parent.release();
 });
});

test("sibling tasks of one process that hold one lease each and wait for the other's are a wait-for cycle: exactly one fails, the other proceeds",async()=>{
 const {directory,env}=scenario({holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});configure(env);
 const shared=signal(),gate=signal();let ready=0;
 const flow=(first:string,second:string)=>withLeaseTaskV1(async()=>{
  const mine=await acquireResourceLease({directory,resource:first,mode:"exclusive",signal:shared});
  ready++;while(ready<2)await tick(10);await tick(first==="free:A"?0:150);
  try{const other=await acquireResourceLease({directory,resource:second,mode:"exclusive",signal:shared});other.release();return "ok";}
  catch(error){return error instanceof LeaseOrderErrorV1?error.violation.kind:String(error);}
  finally{mine.release();}
 });
 const outcomes=await Promise.all([flow("free:A","free:B"),flow("free:B","free:A")]);
 void gate;
 expect(outcomes.sort()).toEqual(["cycle","ok"]);
});

const wedge=async(reentrantShared:boolean)=>{
 const {directory,env}=scenario({fairness:{queuedWithResourceLeases:true,reentrantShared},holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});configure(env);
 const shared=(owner:string)=>acquireQueuedResourceLease({directory,resource:"free:g",mode:"shared",sharedCohort:true,owner,signal:signal()});
 const first=await withLeaseTaskV1(()=>shared("held"));
 const writers=[0,1,2].map(()=>start({mode:"wait",queued:true,directory,resource:"free:g",access:"exclusive",timeoutMs:40000},env));
 await tick(900);
 const started=Date.now(),second=await withLeaseTaskV1(()=>shared("second")).then(lease=>lease,error=>error);
 return {first,second,writers,elapsed:Date.now()-started};
};

test("re-entrant shared: a process that holds a shared lease gets a second shared request at once, even with exclusive requests queued on it",async()=>{
 const {first,second,writers,elapsed}=await wedge(true);
 expect(second).not.toBeInstanceOf(Error);expect(elapsed).toBeLessThan(800);
 second.release();first.release();
 for(const writer of writers)await writer.next("ACQUIRED");
});

test("the exact wedge shape without re-entrancy is detected as a deadlock cycle through the queued-ahead writers, and only the stuck second request fails",async()=>{
 const {first,second,writers,elapsed}=await wedge(false);
 expect(second).toBeInstanceOf(LeaseOrderErrorV1);expect(second.violation.kind).toBe("cycle");expect(elapsed).toBeLessThan(8000);
 expect(second.violation.cycle.length).toBeGreaterThanOrEqual(3);
 first.release();
 for(const writer of writers)await writer.next("ACQUIRED");
});

test("a child started with the inherited environment starts from its parent's position in the lock order",async()=>{
 const {directory,env}=scenario();configure(env);
 const held=await acquireResourceLease({directory,resource:"inner:b",mode:"exclusive",signal:signal()});
 const inherited=leaseInheritanceEnvironmentV1(env as never) as Record<string,string>;
 const refused=start({mode:"probe",directory,resource:"outer:a",access:"exclusive",timeoutMs:2000},inherited);
 expect(JSON.parse((await refused.next("RESULT")).slice(7))).toMatchObject({ok:false,kind:"out-of-order",code:"SEMIO_LEASE_ORDER"});
 const plain=start({mode:"probe",directory,resource:"outer:a",access:"exclusive",timeoutMs:2000},env);
 expect(JSON.parse((await plain.next("RESULT")).slice(7))).toEqual({ok:true});
 const free=start({mode:"probe",directory,resource:"free:z",access:"exclusive",timeoutMs:2000},inherited);
 expect(JSON.parse((await free.next("RESULT")).slice(7))).toEqual({ok:true});
 held.release();
});

const validateReport=new Ajv({strict:true,allowUnionTypes:true}).compile({definitions:statusSchema.definitions,$ref:"#/definitions/Report"} as never);

test("leases status shows holders, waiters with position and blockers, hold durations and the wait-for edge, in English and German, and the JSON matches its schema",async()=>{
 const {directory,env,registry}=scenario();configure(env);
 const holder=start({mode:"hold",queued:true,directory,resource:"outer:status",access:"exclusive"},env);await holder.next("HELD");
 const waiter=start({mode:"wait",queued:true,directory,resource:"outer:status",access:"shared",timeoutMs:20000},env);
 await tick(1500);
 const lines:string[]=[];
 expect(await runLeasesCommandV1(["status","--json",`--lease-directory=${directory}`],{environment:{...env,SEMIO_LANGUAGE:"en"} as never,write:line=>lines.push(line)})).toBe(0);
 const report=JSON.parse(lines.join("\n"));
 expect(validateReport(report)).toBe(true);
 const held=report.holders.find((row:{pid:number})=>row.pid===holder.pid);
 expect(held).toMatchObject({resource:"outer:status",mode:"exclusive",class:"outer",rank:1,source:"registry"});expect(held.heldSeconds).toBeGreaterThanOrEqual(1);
 const waits=report.waiters.find((row:{pid:number})=>row.pid===waiter.pid);
 expect(waits).toMatchObject({resource:"outer:status",mode:"shared",position:2,queued:2});expect(waits.waitedSeconds).toBeGreaterThanOrEqual(1);
 expect(waits.blockers).toContain(`${holder.pid}/${held.task}`);
 expect(report.waitFor.some((edge:{from:string;to:string})=>edge.from.startsWith(`${waiter.pid}/`)&&edge.to.startsWith(`${holder.pid}/`))).toBe(true);
 expect(report.cycles).toEqual([]);
 const english:string[]=[],german:string[]=[];
 await runLeasesCommandV1(["status",`--lease-directory=${directory}`],{environment:{...env,SEMIO_LANGUAGE:"en"} as never,write:line=>english.push(line)});
 await runLeasesCommandV1(["status",`--lease-directory=${directory}`],{environment:{...env,SEMIO_LANGUAGE:"de"} as never,write:line=>german.push(line)});
 expect(english.join("\n")).toContain("HELD");expect(english.join("\n")).toContain("WAITING");expect(english.join("\n")).toContain("position 2 of 2");
 expect(german.join("\n")).toContain("GEHALTEN");expect(german.join("\n")).toContain("WARTET");expect(german.join("\n")).toContain("Position 2 von 2");
 holder.send("release");await holder.next("DONE");await waiter.next("ACQUIRED");await waiter.done;
 await tick(300);
 const after:string[]=[];await runLeasesCommandV1(["status","--json",`--lease-directory=${directory}`],{environment:env as never,write:line=>after.push(line)});
 expect(JSON.parse(after.join("\n")).holders).toEqual([]);
 expect(readdirSync(registry).every(name=>!readdirSync(join(registry,name)).some(file=>/^\d+-\d+\.json$/u.test(file)))).toBe(true);
});

test("a registry entry of a killed process is pruned and never reported",async()=>{
 const {directory,env}=scenario();configure(env);
 const holder=start({mode:"hold",directory,resource:"outer:killed",access:"exclusive"},env);await holder.next("HELD");
 const lines:string[]=[];await runLeasesCommandV1(["status","--json",`--lease-directory=${directory}`],{environment:env as never,write:line=>lines.push(line)});
 expect(JSON.parse(lines.join("\n")).holders.some((row:{pid:number})=>row.pid===holder.pid)).toBe(true);
 holder.stop();await holder.done;
 const after:string[]=[];await runLeasesCommandV1(["status","--json",`--lease-directory=${directory}`],{environment:env as never,write:line=>after.push(line)});
 expect(JSON.parse(after.join("\n")).holders.some((row:{pid:number})=>row.pid===holder.pid)).toBe(false);
});
