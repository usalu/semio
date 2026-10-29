/** 🎛️ Contract: the queued lease port of the claim registry hands the process lease module the signal of the issued control whenever it has one (the module refuses a control next to any other signal, `Original lease signal identity required`), while a caller's own abort still ends its wait at once, a lease granted after that is released exactly once, and the waiters of one resource share one queued ticket instead of queueing a new one per poll.  The refusal rule is that of `remaining` in the process module's leases, copied into the stub.
 * @see ../../🟨️.mjs — the port */
import{expect,test}from"bun:test";
import{createQueuedClaimLeasePortV1}from"../../🟨️.mjs";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import probes from"./probes.json"with{type:"json"};
import freshCases from"./fresh.json"with{type:"json"};

interface Request{readonly resource:string;readonly mode:string;readonly signal:AbortSignal;readonly control?:{readonly signal:AbortSignal}}
const issued=()=>({signal:new AbortController().signal});
const lender=()=>{
 const asked:Request[]=[],grants:(()=>void)[]=[],released:number[]=[];
 const acquire=(request:Request)=>{
  asked.push(request);
  if(request.control&&request.control.signal!==request.signal)return Promise.reject(Error("Original lease signal identity required"));
  return new Promise<{release():void}>(accept=>{grants.push(()=>accept({release:()=>released.push(asked.length)}));});
 };
 return{asked,grants,released,acquire:acquire as never};
};
const settle=async()=>{for(let turn=0;turn<5;turn++)await Promise.resolve();await Bun.sleep(1);};

test("fresh claim probes retain admission failures and use only their bounded kernel port",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.FreshProbeCase),db=new Database(":memory:");
 try{for(const row of freshCases.cases){expect(validate(row)).toBe(true);const control=issued(),failure=Error("fresh failure"),foreign=Error("Another operation timed out"),admission=Object.assign(Error("Resource lease timed out: token"),{resourceLeaseAdmission:Object.freeze({operation:"open"})});let calls=0,releases=0,result="unavailable",caught:unknown;
  const port=createQueuedClaimLeasePortV1({directory:"d",control,acquire:async()=>{throw Error("Queued fresh probe forbidden");},tryAcquire:async()=>{throw Error("Liveness fresh probe forbidden");},acquireFresh:async request=>{calls++;expect(request.timeoutMs).toBe(0);expect(request.signal).toBe(control.signal);expect(request.control).toBe(control);if(row.outcome==="busy")throw Error("Resource lease timed out: token");if(row.outcome==="admission")throw admission;if(row.outcome==="foreign-timeout")throw foreign;if(row.outcome==="failure")throw failure;return{release:()=>{releases++;}};}});
  try{const held=await port.tryHold("token",{fresh:true});if(held){result="held";held.release();}}catch(error){caught=error;result="error";}
  const oracle=db.query("SELECT CASE ? WHEN 'held' THEN 'held' WHEN 'busy' THEN 'unavailable' ELSE 'error' END result").get(row.outcome)as{result:string};expect(result).toBe(row.result);expect(result).toBe(oracle.result);expect(calls).toBe(1);expect(releases).toBe(row.outcome==="held"?1:0);expect(caught).toBe(row.outcome==="admission"?admission:row.outcome==="failure"?failure:row.outcome==="foreign-timeout"?foreign:undefined);
 }}finally{db.close();}
});

test("custody liveness probes use only the nonwaiting kernel port and retain original signals",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.ProbeCase),db=new Database(":memory:");
 try{for(const row of probes.cases){expect(validate(row)).toBe(true);const control=issued(),failure=Error("probe failure");let queued=0,tried=0,released=0,result="unavailable",caught:unknown;
  const port=createQueuedClaimLeasePortV1({directory:"d",control,acquireFresh:async()=>{throw Error("Fresh probe forbidden");},acquire:async()=>{queued++;throw Error("Queued probe forbidden");},tryAcquire:async request=>{tried++;expect(request.timeoutMs).toBe(0);expect(request.mode).toBe("exclusive");expect(request.signal).toBe(control.signal);expect(request.control).toBe(control);if(row.outcome==="busy")throw Error("Resource lease timed out: token");if(row.outcome==="failure")throw failure;return{release:()=>{released++;}};}});
  try{const held=await port.tryHold("token");if(held){result="held";held.release();}}catch(error){caught=error;result="error";}
  const oracle=db.query("SELECT CASE ? WHEN 'held' THEN 'held' WHEN 'busy' THEN 'unavailable' ELSE 'error' END result").get(row.outcome)as{result:string};expect(result).toBe(row.result);expect(result).toBe(oracle.result);expect(queued).toBe(0);expect(tried).toBe(1);expect(released).toBe(row.outcome==="held"?1:0);expect(caught).toBe(row.outcome==="failure"?failure:undefined);
 }}finally{db.close();}
});

test("a wait or a hold under an issued control asks the lease module with the control's own signal, whatever signal the caller derived",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,tryAcquire:bank.acquire,acquireFresh:bank.acquire,directory:"d",control});
 const derived=new AbortController(),waited=port.wait("r",{signal:derived.signal}),held=port.hold("h",{signal:derived.signal});
 await settle();
 expect(bank.asked.map(request=>request.signal)).toEqual([control.signal,control.signal]);
 for(const grant of bank.grants)grant();
 await waited;(await held).release();
});

test("a derived signal that aborts ends the wait at once and the lease granted late is released exactly once",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,tryAcquire:bank.acquire,acquireFresh:bank.acquire,directory:"d",control});
 const derived=new AbortController(),held=port.hold("h",{signal:derived.signal});
 await settle();derived.abort(Error("stop"));
 await expect(held).rejects.toThrow("stop");
 bank.grants[0]!();await settle();
 expect(bank.released).toHaveLength(1);
});

test("the waiters of one resource share one queued ticket however often they poll",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,tryAcquire:bank.acquire,acquireFresh:bank.acquire,directory:"d",control});
 const polls=[0,1,2,3,4].map(()=>{const derived=new AbortController(),wait=port.wait("r",{signal:derived.signal}).catch(()=>undefined);queueMicrotask(()=>derived.abort());return wait;});
 await Promise.all(polls);
 expect(bank.asked).toHaveLength(1);
 bank.grants[0]!();await settle();
 expect(bank.released).toHaveLength(1);
});

test("without a control the caller's own signal reaches the lease module, so an abandoned ticket leaves the queue",async()=>{
 const bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,tryAcquire:bank.acquire,acquireFresh:bank.acquire,directory:"d"}),own=new AbortController(),held=port.hold("h",{signal:own.signal});
 await settle();
 expect(bank.asked.map(request=>request.signal)).toEqual([own.signal]);
 bank.grants[0]!();(await held).release();
});
