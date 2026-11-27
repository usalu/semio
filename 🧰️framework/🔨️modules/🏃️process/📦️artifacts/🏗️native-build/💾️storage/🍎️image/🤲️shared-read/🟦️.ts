import{AsyncLocalStorage}from"node:async_hooks";
import{randomBytes}from"node:crypto";
import{existsSync}from"node:fs";
import{acquireQueuedResourceLease}from"../../../../../🔒️leases/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=schema.definitions.Policy.const,texts=schema.definitions.Texts.const;
/** 📐️ The schema-declared bounds of a shared image hold: the longest one hold, the idle release, the wait of a sampler and its refresh interval.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_SHARED_READ_POLICY_V1=policy;
/** 🏷️ The lease resource that guards every file of one cache image: shared for readers, exclusive for retirement, regrow and detach. */
export function nativeImageSharedReadResourceV1(imageDirectory:string):string{return policy.resourcePrefix+imageDirectory;}
/** 🔌️ The lease effect of a reader: takes the shared lease of one image and answers its release; a `timeoutMs` ends a wait behind a waiting writer. */
export type NativeImageSharedLeasePortV1=Readonly<{acquire(input:Readonly<{leaseDirectory:string;resource:string;signal:AbortSignal;timeoutMs?:number}>):Promise<Readonly<{release():void}>>}>;
/** 🔒️ The production port: the queued shared lease, so a waiting exclusive writer keeps its preference over later readers (`native-fixed-image:` declares an unbounded writer preference). A lease directory that is not mounted holds no lease, so nothing can retire the image and nothing is taken.
 * @see ../../../../../🔒️leases/🧭️order/🔣️policy.json the writer preference */
export const nativeImageSharedLeasePortV1:NativeImageSharedLeasePortV1=Object.freeze({
 async acquire(input){
  if(!existsSync(input.leaseDirectory))return Object.freeze({release(){}});
  const lease=await acquireQueuedResourceLease({directory:input.leaseDirectory,resource:input.resource,mode:"shared",owner:randomBytes(12).toString("hex"),sharedCohort:true,signal:input.signal,...input.timeoutMs===undefined?{}:{timeoutMs:input.timeoutMs}});
  return Object.freeze({release(){lease.release();}});
 }
});
/** 👓️ A reader of one image that holds its shared lease only while it reads. */
export type NativeImageSharedReaderV1=Readonly<{run<T>(body:()=>T|Promise<T>):Promise<T>;close():Promise<void>}>;
/** 🧾️ Everything a reader needs; `port`, `now` and `after` are the test seams. */
export type NativeImageSharedReaderInputV1=Readonly<{leaseDirectory:string;imageDirectory:string;signal:AbortSignal;timeoutMs?:number;maximumHoldMilliseconds?:number;idleReleaseMilliseconds?:number;port?:NativeImageSharedLeasePortV1;now?:()=>number;after?:(milliseconds:number,run:()=>void)=>()=>void}>;
const defaultAfter=(milliseconds:number,run:()=>void)=>{const timer=setTimeout(run,milliseconds);timer.unref?.();return()=>clearTimeout(timer);};
/** 👓️ Makes a reader whose operations (an operation started inside a running operation of the same reader runs inside it and never waits for itself) run under one shared lease of the image that is kept across a burst of operations (so a thousand file reads take one lease, not a thousand), released when the reader idles for `idleReleaseMilliseconds` and never held longer than `maximumHoldMilliseconds`: after that limit new operations wait until the running ones end, the lease is released, and the next operation queues for it again, behind a waiting writer. A waiting writer therefore waits for at most one hold plus the longest single operation. */
export function createNativeImageSharedReaderV1(given:NativeImageSharedReaderInputV1):NativeImageSharedReaderV1{
 const port=given.port??nativeImageSharedLeasePortV1,now=given.now??Date.now,after=given.after??defaultAfter,resource=nativeImageSharedReadResourceV1(given.imageDirectory),maximum=given.maximumHoldMilliseconds??policy.maximumHoldMilliseconds,idle=given.idleReleaseMilliseconds??policy.idleReleaseMilliseconds;
 const inside=new AsyncLocalStorage<true>();
 let held:{release():void;since:number}|undefined,taking:Promise<void>|undefined,running=0,closed=false,cancel:(()=>void)|undefined,drained:Promise<void>|undefined,settle:(()=>void)|undefined;
 const release=()=>{const lease=held;held=undefined;cancel?.();cancel=undefined;try{lease?.release();}catch{}};
 const take=()=>taking??=(async()=>{try{const lease=await port.acquire({leaseDirectory:given.leaseDirectory,resource,signal:given.signal,...given.timeoutMs===undefined?{}:{timeoutMs:given.timeoutMs}});if(closed){lease.release();return;}held={release:lease.release,since:now()};}finally{taking=undefined;}})();
 const drain=()=>drained??=new Promise<void>(resolve=>{settle=resolve;if(running===0)finish();}).then(()=>{release();drained=undefined;});
 const finish=()=>{const done=settle;settle=undefined;done?.();};
 const arm=()=>{cancel?.();cancel=after(idle,()=>{if(running===0&&held!==undefined)release();});};
 return Object.freeze({
  async run<T>(body:()=>T|Promise<T>):Promise<T>{
   if(inside.getStore()===true)return await body();
   for(;;){
    if(closed)throw new Error(texts.closed.en+" / "+texts.closed.de);
    if(held!==undefined&&now()-held.since>=maximum){await drain();continue;}
    if(held===undefined){await take();continue;}
    break;
   }
   running++;cancel?.();cancel=undefined;
   try{return await inside.run(true,async()=>body());}
   finally{running--;if(running===0){if(settle!==undefined)finish();else if(held!==undefined)arm();}}
  },
  async close(){closed=true;await taking?.catch(()=>{});if(held!==undefined)await drain();release();}
 });
}
/** 🔒️ Runs `body` once under the shared lease of the image, released when it ends: the choice for a single read. */
export async function withNativeImageSharedReadV1<T>(given:Readonly<{leaseDirectory:string;imageDirectory:string;signal:AbortSignal;timeoutMs?:number;port?:NativeImageSharedLeasePortV1}>,body:()=>T|Promise<T>):Promise<T>{
 const lease=await(given.port??nativeImageSharedLeasePortV1).acquire({leaseDirectory:given.leaseDirectory,resource:nativeImageSharedReadResourceV1(given.imageDirectory),signal:given.signal,...given.timeoutMs===undefined?{}:{timeoutMs:given.timeoutMs}});
 try{return await body();}finally{lease.release();}
}
/** 📸️ A value a synchronous caller polls (a progress stamp the restart watch reads) and that is refreshed in the background under the shared lease, with a bounded wait: while a writer holds or awaits the image the last value stays, nothing is read and nothing blocks. */
export type NativeImageSharedSampleV1<T>=Readonly<{value():T;settled():Promise<void>}>;
/** 📸️ Makes a sample: `value()` answers the last value at once and starts one refresh when `refreshMilliseconds` have passed since the last start; a failed refresh (a writer that did not release in `sampleWaitMilliseconds`) keeps the last value. */
export function createNativeImageSharedSampleV1<T>(given:Readonly<{reader:NativeImageSharedReaderV1;read:()=>T;initial:T;refreshMilliseconds?:number;now?:()=>number}>):NativeImageSharedSampleV1<T>{
 const now=given.now??Date.now,every=given.refreshMilliseconds??policy.sampleRefreshMilliseconds;
 let last=given.initial,startedAt=-Infinity,flight:Promise<void>|undefined;
 return Object.freeze({
  value(){if(flight===undefined&&now()-startedAt>=every){startedAt=now();flight=given.reader.run(given.read).then(value=>{last=value;},()=>{}).finally(()=>{flight=undefined;});}return last;},
  settled:async()=>{await flight;}
 });
}
