/** 🥇️ Contract: one service generation reads the graph at a time. A superseded or draining generation hands the loads whose worker has not started over (their clients get the typed `handover`) and starts no further worker, a restart included, while a running worker finishes; a younger generation waits for an older one whose descriptor says it still reads slices, and starts its worker once that one is gone.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../🏛️service/🌐️host/🟨️.mjs */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {olderBusyServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import fixture from "./🔣️.json";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as Record<string,number>,guard=HOST_POLICY_V1.slicePlanGuard as Record<string,number>,MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,value?:unknown,extra?:unknown)=>void};

const harness=(predecessor?:()=>unknown,bound?:number)=>{
 const workers:Fake[]=[],decisions:{key:string;used:number}[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:20,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  pressure:()=>({sample:()=>({tripped:false}),stop:()=>undefined}),headroom:()=>({limitBytes:Infinity,currentBytes:100*MiB,availableBytes:8000*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),observe:async()=>1000,
  ...predecessor===undefined?{}:{predecessor},...bound===undefined?{}:{predecessorBoundMilliseconds:bound},
  createWorker:()=>{const listeners:Record<string,(value?:unknown,extra?:unknown)=>void>={},fake:Fake={emit:(event,value,extra)=>listeners[event]?.(value,extra)};workers.push(fake);return{pid:4242,on:(event:string,listener:(value?:unknown,extra?:unknown)=>void)=>{listeners[event]=listener;},send:()=>true,terminate:async()=>0};}} as never);
 const start=(name:string)=>pool.run({root:"/tmp/"+name},{onProgress:(_ms:number,detail:any)=>{if(detail?.decision)decisions.push(detail.decision);}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1];};
 return{pool,workers,decisions,start,next};
};
const identity={root:"/r",state:""},handover=Object.assign(new Error("handover"),{code:"handover",reason:"superseded"});

test("the older busy generation is the live earlier descriptor with running reads, whatever its siblings say (oracle: sql)",()=>{
 const db=new Database(":memory:");
 try{
  db.run("create table service(name text, startedAt integer, busy integer, pid integer, alive integer)");
  for(const row of fixture.descriptors)db.query("insert into service values (?,?,?,?,?)").run(row.name,row.startedAt,row.busy,row.pid,Number(row.alive));
  const oracle=db.query("select pid from service where name glob 'host-[0-9a-f]*.json' and name <> ?1 and startedAt < ?2 and busy > 0 and alive = 1 order by name limit 1").get("host-"+fixture.tag+".json",fixture.since) as {pid:number};
  const found=olderBusyServiceV1({directory:"/unused",tag:fixture.tag,since:fixture.since,identity,list:()=>fixture.descriptors.map(row=>row.name),read:name=>({...fixture.descriptors.find(row=>row.name===name)!,identity}),alive:pid=>fixture.descriptors.find(row=>row.pid===pid)!.alive});
  expect(oracle.pid).toBe(fixture.expectedPid);
  expect(found?.pid).toBe(fixture.expectedPid);
  expect(olderBusyServiceV1({directory:"/unused",tag:fixture.tag,since:0,identity,list:()=>fixture.descriptors.map(row=>row.name),read:name=>({...fixture.descriptors.find(row=>row.name===name)!,identity}),alive:()=>true})).toBeUndefined();
 }finally{db.close();}
});

test("a handover refuses the queued load with the typed error and leaves the running worker alone, and a restart after it is handed over too",async()=>{
 const{pool,start,next}=harness(),running=start("running"),worker=(await next(1))!;
 const queued=start("queued").then(()=>undefined,error=>error);
 await wait(50);pool.handover(handover);
 expect(await queued).toBe(handover);
 worker.emit("exit",null,"SIGKILL");
 expect(await running.then(()=>undefined,error=>error)).toBe(handover);
 await pool.close();
});

test("an older generation whose pid was reused by another process is not alive: the identity is pid and start time",()=>{
 const descriptors=[{name:"host-aaaaaaaaaaaa.json",pid:41,startMs:5000,startedAt:1,busy:2,identity}],seen:[number,number|undefined][]=[];
 const list=()=>descriptors.map(row=>row.name),read=(name:string)=>descriptors.find(row=>row.name===name)!;
 const found=olderBusyServiceV1({directory:"/unused",tag:"bbbbbbbbbbbb",since:10,identity,list,read,alive:(pid,startMs)=>{seen.push([pid,startMs]);return startMs===5000&&pid===41;}});
 expect(found?.pid).toBe(41);expect(seen).toEqual([[41,5000]]);
 expect(olderBusyServiceV1({directory:"/unused",tag:"bbbbbbbbbbbb",since:10,identity,list,read,alive:(_pid,startMs)=>startMs!==5000})).toBeUndefined();
 expect(olderBusyServiceV1({directory:"/unused",tag:"bbbbbbbbbbbb",since:10,identity:{root:"/other",state:""},list,read,alive:()=>true}),"a busy service of another checkout is not waited for").toBeUndefined();
});

test("a younger generation stops waiting for an older busy one after the declared bound and starts its worker (a held old generation never blocks it)",async()=>{
 const older={pid:77,busy:1},{pool,workers,decisions,start,next}=harness(()=>older,150),run=start("bounded");
 await wait(60);expect(workers.length).toBe(0);
 (await next(1))!.emit("message",{type:"done",graph:{ok:3}});
 expect(await run).toEqual({graph:{ok:3}});expect(decisions.some(row=>row.key==="predecessor"&&row.used===77)).toBe(true);await pool.close();
});

test("a younger generation waits for the older busy one before it starts a worker and says so",async()=>{
 let older:{pid:number;busy:number}|undefined={pid:77,busy:1};
 const{pool,workers,decisions,start,next}=harness(()=>older),run=start("waits");
 await wait(120);expect(workers.length).toBe(0);expect(decisions.some(row=>row.key==="predecessor"&&row.used===77)).toBe(true);
 older=undefined;(await next(1))!.emit("message",{type:"done",graph:{ok:1}});
 expect(await run).toEqual({graph:{ok:1}});await pool.close();
});
