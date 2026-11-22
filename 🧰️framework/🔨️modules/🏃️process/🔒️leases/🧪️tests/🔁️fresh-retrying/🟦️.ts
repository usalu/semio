import {afterAll,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import policy from "../../🧭️order/🔣️policy.json";

/** 🔁️ Contract: a fresh claim whose directory admission gate was busy for the whole admission patience is tried again within the gate bounds `freshRetryAttempts` and `freshRetryPauseMilliseconds`; a real holder of the resource ends it at once (a fresh claim never waits for a real writer), a gate that stays busy ends in the typed timeout after every declared try, and cancellation ends it at once (the staged-input journal claim of the dependency sync relies on it).
 * @see ../../🧭️order/🧬️schema/🔣️.json the gate policy */
const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"fresh-retry-")),file=join(root,"policy.json");
writeFileSync(file,JSON.stringify({...policy,gate:{...policy.gate,admissionPatienceMilliseconds:300,freshRetryAttempts:4,freshRetryPauseMilliseconds:100}}));
process.env.SEMIO_LEASE_ORDER_POLICY=file;process.env.SEMIO_LEASE_ORDER="off";
afterAll(()=>rmSync(root,{recursive:true,force:true}));

test("the authored gate policy declares the retry bounds",()=>{
 expect(policy.gate.freshRetryAttempts).toBeGreaterThan(1);
 expect(policy.gate.freshRetryPauseMilliseconds).toBeGreaterThan(0);
});

test("a busy gate is waited out, a real holder is final at once, a gate that stays busy ends after every declared try, cancellation ends at once",async()=>{
 const {acquireFreshResourceLeaseV1,acquireFreshResourceLeaseRetryingV1,resourceLeasePathV1,ResourceLeaseTimeoutErrorV1}=await import("../../🟦️.ts");
 const signal=new AbortController().signal,options={directory:root,resource:"native-storage:identity",mode:"exclusive" as const,signal,control:null};
 (await acquireFreshResourceLeaseV1(options as never)).release();
 const gate=new Database(resourceLeasePathV1(root,"resource-directory-admission:v1"),{readwrite:true,create:false});
 let locked=false;const lock=()=>{gate.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");locked=true;},unlock=()=>{if(locked){gate.exec("ROLLBACK");locked=false;}};
 try{
  lock();setTimeout(unlock,500);
  const waited=await acquireFreshResourceLeaseRetryingV1(options as never);
  expect(waited.resource).toBe("native-storage:identity");

  const started=Date.now(),error=await acquireFreshResourceLeaseRetryingV1(options as never).catch(failure=>failure);
  expect(error).toBeInstanceOf(ResourceLeaseTimeoutErrorV1);
  expect(Date.now()-started).toBeLessThan(250);
  waited.release();

  lock();
  const stuck=Date.now(),final=await acquireFreshResourceLeaseRetryingV1(options as never).catch(failure=>failure);
  expect(final).toBeInstanceOf(ResourceLeaseTimeoutErrorV1);
  expect(Date.now()-stuck).toBeGreaterThanOrEqual(4*300+100+200+300-60);
  expect(Date.now()-stuck).toBeLessThan(6000);

  const controller=new AbortController(),cancelled=acquireFreshResourceLeaseRetryingV1({...options,signal:controller.signal} as never).catch(failure=>failure);
  setTimeout(()=>controller.abort(),100);
  const reason=await cancelled;
  expect(reason).not.toBeInstanceOf(ResourceLeaseTimeoutErrorV1);
 }finally{unlock();gate.close(true);}
});
