import {test,expect} from "bun:test";
import {linkSync,mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as pause} from "node:timers/promises";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import * as leases from "../../🟦️.ts";
import {withResourceLeaseOpenPortV1} from "../../♻️retirement/🟦️.ts";
import {leaseDatabaseReplacedV1,leaseUnopenablePausesV1} from "../../🪟️windows/🟦️.ts";
import {SqliteDatabaseV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🗃️sqlite/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"lease-replaced-file",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
/** 🔁️ Pins that a waiter whose lease database was unlinked and recreated under it reopens the path and acquires instead of failing with a vnode I/O error. */
test("a waiter survives its lease database being replaced while it waits",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"lease-replaced-")),control=createIssuedResourceControlV1(original,"lease:replaced",0),resource="fixture:replaced",path=leases.resourceLeasePathV1(directory,resource);
 try{const holder=await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:control.signal,control});const waiter=leases.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control,timeoutMs:20000});
  await pause(200);rmSync(path,{force:true});holder.release();const lease=await waiter;expect(lease.mode).toBe("shared");lease.release();
 }finally{rmSync(directory,{recursive:true,force:true});}
}),30000);

/** 🍎️ Pins that `SQLITE_IOERR_VNODE` raised by a connection (at the first configuration or while acquiring) closes and reopens it on the declared pauses, and that a store replaced more often than those pauses fails with an explained error instead of looping. */
test("a vnode I/O error reopens the lease connection a bounded number of times and then explains itself",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"lease-vnode-")),control=createIssuedResourceControlV1(original,"lease:vnode",0);
 const vnode=()=>Object.assign(new Error("disk I/O error"),{code:"SQLITE_IOERR_VNODE",errno:6922,errcode:6922});
 const faulty=(budget:{left:number},opened:{count:number},match:string,target:string)=>(path:string,options:Readonly<{readonly:boolean;readwrite:boolean;create:boolean}>)=>{const real=new SqliteDatabaseV1(path,{...options});if(path!==target)return real;opened.count++;return new Proxy(real,{get:(self,key)=>key==="exec"?(sql:string)=>{if(sql.includes(match)&&budget.left-->0)throw vnode();return self.exec(sql);}:typeof (self as never as Record<PropertyKey,unknown>)[key]==="function"?(self as never as Record<PropertyKey,(...rest:unknown[])=>unknown>)[key]!.bind(self):(self as never as Record<PropertyKey,unknown>)[key]});};
 try{
  for(const [name,match] of [["configuration","busy_timeout"],["transaction","BEGIN"]] as const){
   const budget={left:2},opened={count:0},resource="fixture:vnode-"+name;
   const lease=await withResourceLeaseOpenPortV1(faulty(budget,opened,match,leases.resourceLeasePathV1(directory,resource)),()=>leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:control.signal,control,timeoutMs:20000}));
   expect(lease.mode).toBe("exclusive");expect(opened.count).toBe(3);lease.release();
  }
  const budget={left:1000},opened={count:0};
  await expect(withResourceLeaseOpenPortV1(faulty(budget,opened,"busy_timeout",leases.resourceLeasePathV1(directory,"fixture:vnode-persistent")),()=>leases.acquireResourceLease({directory,resource:"fixture:vnode-persistent",mode:"shared",signal:control.signal,control,timeoutMs:20000}))).rejects.toThrow(/replaced under its connection .* probably detached and attached again/u);
  expect(opened.count).toBe(leaseUnopenablePausesV1.length+1);
  const refusals={left:2,opened:0},target=leases.resourceLeasePathV1(directory,"fixture:vnode-open");
  const lease=await withResourceLeaseOpenPortV1((path,options)=>{if(path===target){refusals.opened++;if(refusals.left-->0)throw vnode();}return new SqliteDatabaseV1(path,{...options});},()=>leases.acquireResourceLease({directory,resource:"fixture:vnode-open",mode:"shared",signal:control.signal,control,timeoutMs:20000}));
  expect(refusals.opened).toBe(3);lease.release();
  const stuck={opened:0},stuckTarget=leases.resourceLeasePathV1(directory,"fixture:vnode-open-persistent");
  await expect(withResourceLeaseOpenPortV1((path,options)=>{if(path===stuckTarget){stuck.opened++;throw vnode();}return new SqliteDatabaseV1(path,{...options});},()=>leases.acquireResourceLease({directory,resource:"fixture:vnode-open-persistent",mode:"shared",signal:control.signal,control,timeoutMs:20000}))).rejects.toThrow(/replaced under its connection/u);
  expect(stuck.opened).toBe(leaseUnopenablePausesV1.length+1);
  expect(leaseDatabaseReplacedV1(vnode())).toBe(true);expect(leaseDatabaseReplacedV1({errcode:6922})).toBe(true);expect(leaseDatabaseReplacedV1({code:"SQLITE_BUSY",errcode:5})).toBe(false);expect(leaseDatabaseReplacedV1(null)).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
}),60000);

/** 🏁️ Pins that a lease file seen with two links (a concurrent publication or retirement window) is looked at again and acquired once it settles, and that one that stays doubly linked is refused with the explanation. */
test("a lease file with a transient second link is retried, a persistent one is refused with an explanation",()=>withBuildTestInvocationV1(async original=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR!;mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"lease-nlink-")),control=createIssuedResourceControlV1(original,"lease:nlink",0);
 try{
  for(const [resource,settles] of [["fixture:nlink-transient",true],["fixture:nlink-persistent",false]] as const){
   (await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:control.signal,control})).release();
   const path=leases.resourceLeasePathV1(directory,resource),extra=path+".extra";linkSync(path,extra);
   if(settles)setTimeout(()=>rmSync(extra,{force:true}),30);
   const attempt=leases.acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control,timeoutMs:20000});
   if(settles){const lease=await attempt;expect(lease.mode).toBe("shared");lease.release();}
   else await expect(attempt).rejects.toThrow(/Invalid resource lease database: .*nlink 2, still so after/u);
   rmSync(extra,{force:true});
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
}),60000);
