import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {SCHEDULER_POLICY_V1,type SchedulerPolicyV1} from "../../🟦️.ts";
import {createFileSchedulerStoreV1} from "../../💾️store/🟦️.ts";

/** 🏋️ Real-process soak: 16 client processes share one host store and run, dedupe and cancel concurrently. Checks that no ticket is lost, deduped tickets never execute, equal workloads finish within a bounded spread, and the log and state stay bounded and drained.
 * @see ./👷️client.ts the client process */
const clients=16,unique=4,chaos=8,client=join(import.meta.dir,"👷️client.ts");
const rows=(directory:string,name:string):Record<string,any>[]=>readdirSync(directory).filter(file=>file.startsWith(`${name}.`)).flatMap(file=>readFileSync(join(directory,file),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)));

test("16 real client processes: no ticket lost, dedupe executes once, fair finish times, bounded log, drained queue",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"scheduler-soak-")),startAt=Date.now()+1500;
 try{
  const codes=await Promise.all(Array.from({length:clients},(_,index)=>new Promise<number>(resolve=>{
   const child=spawn(process.execPath,[client,directory,`agent${String(index).padStart(2,"0")}`,String(startAt),String(unique),String(chaos)],{stdio:["ignore","ignore","inherit"]});
   child.on("close",code=>resolve(code??1));
  })));
  expect(codes).toEqual(Array(clients).fill(0));
  const submitted=rows(directory,"submitted"),results=rows(directory,"results"),executed=rows(directory,"executed");
  expect(submitted.length).toBe(clients*(unique+chaos));
  expect(results.length).toBe(submitted.length);
  expect(results.filter(row=>row.error!==undefined)).toEqual([]);
  expect(new Set(results.map(row=>row.ticket)).size).toBe(submitted.length);
  const ran=new Set(executed.map(row=>row.ticket)),byTicket=new Map(results.map(row=>[row.ticket,row]));
  expect(ran.size).toBe(executed.length);
  for(const ticket of ran)expect(byTicket.get(ticket)!.shared,ticket).toBe(false);
  const sharedTickets=results.filter(row=>row.shared);
  expect(sharedTickets.length).toBeGreaterThan(0);
  expect(executed.length).toBeLessThan(submitted.length);
  for(const row of sharedTickets)expect(ran.has(row.ticket)).toBe(false);
  for(const row of results.filter(row=>row.key.startsWith("unique:")))expect(row.exit,row.ticket).toBe("ok");
  const times=rows(directory,"timing").map(row=>row.elapsedMs as number);
  expect(times.length).toBe(clients);
  expect(Math.max(...times)/Math.min(...times)).toBeLessThanOrEqual(1.5);
  const policy={...SCHEDULER_POLICY_V1,maxConcurrent:3,compactEveryEvents:64,maxFinished:32} as SchedulerPolicyV1,store=createFileSchedulerStoreV1(directory,policy,{host:()=>({budgetMegabytes:100000})}),snapshot=await store.snapshot();
  expect(snapshot.running).toEqual([]);expect(snapshot.queued).toEqual([]);
  expect(snapshot.finished.length).toBeLessThanOrEqual(16);
  const log=readFileSync(join(directory,"log.ndjson"),"utf8").split("\n").filter(Boolean).length;
  expect(log).toBeLessThan(64+40);
  expect(readFileSync(join(directory,"state.json")).length).toBeLessThan(200_000);
  expect(existsSync(join(directory,"lock"))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120_000);
