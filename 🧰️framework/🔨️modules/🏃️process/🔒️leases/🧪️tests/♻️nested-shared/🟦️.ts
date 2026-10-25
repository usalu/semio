import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{join}from"node:path";
import{randomBytes}from"node:crypto";
import{testScratchRootV1}from"../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{acquireQueuedResourceLease,resourceLeasePathV1}from"../../🟦️.ts";
const owner=()=>randomBytes(8).toString("hex");
test("a holder that asks for the same shared lease again is granted at once instead of queueing behind its own ticket",async()=>{
 const root=testScratchRootV1("leases-nested-shared");mkdirSync(root,{recursive:true});const directory=mkdtempSync(join(root,"n-")),resource="build-graph:/fixture/snapshot.json",controller=new AbortController();
 try{
  const first=await acquireQueuedResourceLease({directory,resource,mode:"shared",owner:owner(),signal:controller.signal,timeoutMs:5000});
  const started=Date.now(),second=await acquireQueuedResourceLease({directory,resource,mode:"shared",owner:owner(),signal:controller.signal,timeoutMs:5000});
  expect(Date.now()-started).toBeLessThan(2500);
  const oracle=new Database(resourceLeasePathV1(directory,resource));try{oracle.exec("pragma busy_timeout=0");expect(()=>oracle.exec("begin exclusive")).toThrow();first.release();expect(()=>oracle.exec("begin exclusive")).toThrow();second.release();oracle.exec("begin exclusive;rollback");}finally{oracle.close();}
  const exclusive=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:owner(),signal:controller.signal,timeoutMs:5000});exclusive.release();
 }finally{controller.abort();rmSync(directory,{recursive:true,force:true});}
});
