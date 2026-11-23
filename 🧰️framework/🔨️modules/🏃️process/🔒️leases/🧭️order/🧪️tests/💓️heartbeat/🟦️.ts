import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {leaseRegistryV1,readLeaseRegistryEntriesV1} from "../../🗂️registry/🟦️.ts";

/** 💓️ A publish that fails (a full disk, a locked folder) never silences a live holder for good: the next heartbeat publishes its standing again, so a waiter can judge it stuck and ask it to yield instead of queueing behind an unreadable holder forever. */
test("a failed registry publish is retried by the heartbeat",()=>{
 const base=mkdtempSync(join(tmpdir(),"lease-heartbeat-")),blocked=join(base,"blocked"),good=join(base,"good"),directory=join(base,"leases"),previous=process.env.SEMIO_LEASE_REGISTRY_DIRECTORY,prune=process.env.SEMIO_LEASE_REGISTRY_PRUNE;
 process.env.SEMIO_LEASE_REGISTRY_PRUNE="off";
 try{
  writeFileSync(blocked,"");
  const snapshot={holds:[{directory,resource:"nx-tooling-provision",mode:"exclusive" as const,atMs:1,task:1}],waits:[]};
  leaseRegistryV1.attach({snapshot:()=>snapshot,cycle:()=>undefined});
  process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(blocked,"root");
  leaseRegistryV1.changed(directory,{registry:{flushMilliseconds:0}} as never,true);
  process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=good;
  expect(readLeaseRegistryEntriesV1(good,directory,Date.now(),false)).toHaveLength(0);
  leaseRegistryV1.heartbeat();
  const [entry]=readLeaseRegistryEntriesV1(good,directory,Date.now(),false);
  expect(entry?.pid).toBe(process.pid);
  expect(entry?.holds[0]?.resource).toBe("nx-tooling-provision");
  expect(entry?.standing.progressAtMs).toBeGreaterThan(0);
 }finally{
  if(previous===undefined)delete process.env.SEMIO_LEASE_REGISTRY_DIRECTORY;else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=previous;
  if(prune===undefined)delete process.env.SEMIO_LEASE_REGISTRY_PRUNE;else process.env.SEMIO_LEASE_REGISTRY_PRUNE=prune;
  snapshotEmpty();rmSync(base,{recursive:true,force:true});
 }
});
function snapshotEmpty():void{leaseRegistryV1.attach({snapshot:()=>({holds:[],waits:[]}),cycle:()=>undefined});}
