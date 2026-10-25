import {AsyncLocalStorage} from "node:async_hooks";
import {setTimeout as pause} from "node:timers/promises";
import {nativeImageInventoryV1,type NativeImageInventoryPolicyV1} from "../📥️information/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.ImageProtocol.const;

/** 📡️ One whole kernel image inventory and the monotonic instant its capture began. */
export type NativeImageSnapshotV1=Readonly<{serial:number;startedAt:number;information:Record<string,unknown>;text:string}>;
/** 📬️ Asks for an accepted view of one snapshot that is at most `maximumAgeMilliseconds` old and newer than `after`. */
export type NativeImageRosterRequestV1<T>=Readonly<{capture:()=>Promise<string>;accept:(information:Record<string,unknown>,text:string)=>T;policy:NativeImageInventoryPolicyV1;maximumAgeMilliseconds?:number;after?:number}>;
/** 🖼️ Shares kernel inventory captures between every image in one process. */
export type NativeImageRosterV1=Readonly<{observe<T>(request:NativeImageRosterRequestV1<T>):Promise<Readonly<{value:T;serial:number}>>;invalidate():void;captures():number;shared():number}>;

class SnapshotFailure{readonly error:unknown;readonly own:boolean;constructor(error:unknown,own:boolean){this.error=error;this.own=own;}}
const retryable=(error:unknown)=>error instanceof Error&&(/ exited with status [0-9]+ signal null/u.test(error.message)||error.message.startsWith("Torn system image inventory")||error.message.startsWith("System image plist required")||error.message.startsWith("Invalid image plist")||error.message.startsWith("Incomplete image plist")||error.message.startsWith("Unsupported system image plist"));

/** 🧭️ A roster whose captures are single-flight: an arrival during a capture waits for the next one, never for an older one, and a snapshot younger than the caller's age bound is reused. */
export function createNativeImageRosterV1(options:Readonly<{now:()=>number}>):NativeImageRosterV1{
 let serial=0,captured=0,reused=0,epoch=0,latest:NativeImageSnapshotV1|undefined,running:Promise<NativeImageSnapshotV1>|undefined,queued:Promise<NativeImageSnapshotV1>|undefined;
 const start=(capture:()=>Promise<string>):Promise<NativeImageSnapshotV1>=>{
  const startedAt=options.now(),startedEpoch=epoch,work=(async()=>{captured++;const text=await capture(),information=nativeImageInventoryV1(text),snapshot=Object.freeze({serial:++serial,startedAt,information,text});if(startedEpoch===epoch)latest=snapshot;return snapshot;})();
  running=work;const clear=()=>{if(running===work)running=undefined;};work.then(clear,clear);return work;
 };
 const next=(capture:()=>Promise<string>):Promise<NativeImageSnapshotV1>=>{
  if(queued)return queued;
  if(!running)return start(capture);
  const prior=running;queued=(async()=>{await prior.catch(()=>undefined);queued=undefined;return start(capture);})();return queued;
 };
 const snapshot=(request:NativeImageRosterRequestV1<unknown>,fresh:boolean)=>{
  const after=request.after??0,age=request.maximumAgeMilliseconds??0;
  if(!fresh&&age>0&&latest&&latest.serial>after&&options.now()-latest.startedAt<=age){reused++;return Promise.resolve({snapshot:latest,own:false});}
  let own=false;const wrapped=()=>{own=true;return request.capture();};
  return next(wrapped).then(value=>({snapshot:value,own}),error=>{throw new SnapshotFailure(error,own);});
 };
 const observe=async<T>(request:NativeImageRosterRequestV1<T>):Promise<Readonly<{value:T;serial:number}>>=>{
  let failure:unknown,fresh=false;
  for(let attempt=0;attempt<request.policy.attempts;attempt++){
   if(attempt)await pause(request.policy.backoffMilliseconds*attempt+process.pid%7*10,undefined,request.policy.signal?{signal:request.policy.signal}:undefined);
   let got:Awaited<ReturnType<typeof snapshot>>;
   try{got=await snapshot(request,fresh);}catch(wrapper){const error=wrapper instanceof SnapshotFailure?wrapper.error:wrapper,own=wrapper instanceof SnapshotFailure?wrapper.own:true;if(request.policy.signal?.aborted)throw error;if(own&&!retryable(error))throw error;failure=error;fresh=true;continue;}
   try{return Object.freeze({value:request.accept(got.snapshot.information,got.snapshot.text),serial:got.snapshot.serial});}catch(error){failure=error;fresh=true;if(latest===got.snapshot)latest=undefined;}
  }
  throw failure instanceof Error?new Error(failure.message+" ("+request.policy.attempts+" consistent inventory attempts exhausted)",{cause:failure}):failure;
 };
 return Object.freeze({observe,invalidate:()=>{epoch++;latest=undefined;},captures:()=>captured,shared:()=>reused});
}

/** 🧲️ The process-wide roster every persistent image reads. */
export const nativeImageRosterV1=createNativeImageRosterV1({now:()=>performance.now()});

/** 🧹️ Forgets every snapshot that began before this process changed the kernel image set: a snapshot taken before a detach would still list the detached image, one taken before an attach would miss the new mount. */
export function invalidateNativeImageRosterV1():void{nativeImageRosterV1.invalidate();}

/** 🔁️ Accepted roster view with the shape of the unshared observer; arrivals during one capture share the next. */
export async function observeSharedNativeImageInventoryV1<T>(capture:()=>Promise<string>,accept:(information:Record<string,unknown>,text:string)=>T,policy:NativeImageInventoryPolicyV1):Promise<T>{return (await nativeImageRosterV1.observe({capture,accept,policy})).value;}

const windows=new AsyncLocalStorage<number>();
/** 🪟️ Declares that physical rechecks inside `body` may reuse a verification younger than `milliseconds`; image open, close and maintenance stay strict. */
export function withNativeRecheckWindowV1<T>(body:()=>Promise<T>,milliseconds:number=protocol.recheckWindowMilliseconds):Promise<T>{
 if(!Number.isSafeInteger(milliseconds)||milliseconds<0||milliseconds>protocol.recheckWindowMaximumMilliseconds)throw Error("Bounded native recheck window required");
 return windows.run(milliseconds,body);
}
/** 💻️ The recheck window of the running operation: zero outside a declared window. */
export function nativeRecheckWindowV1():number{return windows.getStore()??0;}

/** 🚧️ Reuses the last full physical verification inside the declared window while the cheap identity checks still run on every call. */
export function createNativePhysicalGateV1(now:()=>number=()=>performance.now()):Readonly<{run(window:number,cheap:()=>void,full:()=>Promise<void>):Promise<void>;fulls():number}>{
 let verifiedAt=Number.NEGATIVE_INFINITY,inflight:Readonly<{startedAt:number;work:Promise<void>}>|undefined,fulls=0;
 return Object.freeze({
  async run(window,cheap,full){
   cheap();
   if(window>0&&now()-verifiedAt<=window)return;
   if(window>0&&inflight&&now()-inflight.startedAt<=window){try{await inflight.work;return;}catch{}}
   const startedAt=now(),work=(async()=>{fulls++;await full();verifiedAt=startedAt;})(),entry=Object.freeze({startedAt,work}),clear=()=>{if(inflight===entry)inflight=undefined;};
   inflight=entry;work.then(clear,clear);return work;
  },
  fulls:()=>fulls,
 });
}

const settledRosters=new WeakMap<object,string>();
/** 🧷️ Remembers the last roster rows a receiver observed in two agreeing captures. */
export function rememberSettledNativeRosterV1(owner:object,rows:string):void{settledRosters.set(owner,rows);}
/** 📞️ Returns the last settled roster rows of a receiver. */
export function settledNativeRosterV1(owner:object):string|undefined{return settledRosters.get(owner);}
