import {OFFLOAD_POLICY_V1,offloadTextsV1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";

/** 🚛️ How the scheduler's queue stands for a waiting unit: the actions waiting, the origins that run actions now and the typical wait in seconds (the median of the last placements), each `undefined` when the scheduler does not say. */
export type BeyondHostQueueV1=Readonly<{queued?:number;holders?:readonly string[];typicalSeconds?:number}>;

/** 🚛️ The answer of one offload attempt as the wait reads it: the compiler's result (`done`) or a reason the unit did not run elsewhere. */
export type BeyondHostAttemptV1=Readonly<{kind:"done"}>|Readonly<{kind:"local";reason:OffloadReasonV1;detail:string}>;

/** 🔌️ Everything the wait touches, so a test drives it with a fake clock and a fake scheduler. */
export type BeyondHostPortsV1<T extends BeyondHostAttemptV1>=Readonly<{crate:string;attempt:(remainingMilliseconds:number)=>Promise<T>;queue:()=>Promise<BeyondHostQueueV1|undefined>;report:(text:string)=>void;signal?:AbortSignal;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;timeoutMilliseconds?:number;progressMilliseconds?:number;retryMilliseconds?:number}>;

const RETRYABLE:ReadonlySet<OffloadReasonV1>=new Set(["infrastructure","no-idle-machine"]);

/** 🚛️ Waits for a machine that can carry a unit that is beyond the host of its requester and never lets it compile there: it runs `attempt` with the time that is left, repeats it after a short pause while the fleet is only busy or unreachable (`infrastructure`, `no-idle-machine`), reports in English and German every `progressMilliseconds` how many actions wait, who is running, the typical wait and the time that is left, honours the abort signal at once, and ends after `beyondHost.timeoutMilliseconds` (or at any other reason that no machine will take it) with `beyond-host-refused` and a bilingual line. Never throws.
 * @see ../🧬️schema/🔣️.json (`Policy.beyondHost`)
 * @see ./🧪️tests/🟦️.ts */
export async function awaitBeyondHostV1<T extends BeyondHostAttemptV1>(ports:BeyondHostPortsV1<T>):Promise<T|Readonly<{kind:"local";reason:OffloadReasonV1;detail:string}>>{
 const rules=OFFLOAD_POLICY_V1.beyondHost,now=ports.now??Date.now,timeout=ports.timeoutMilliseconds??rules.timeoutMilliseconds,progress=ports.progressMilliseconds??rules.progressMilliseconds,retry=ports.retryMilliseconds??rules.retryMilliseconds,sleep=ports.sleep??((milliseconds:number,signal?:AbortSignal)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}));
 const deadline=now()+timeout,remaining=():number=>Math.max(0,deadline-now());
 const aborted=():boolean=>ports.signal?.aborted===true;
 const refuse=(reason:string)=>{ports.report(offloadTextsV1("beyondHost.refused",{crate:ports.crate,reason}));return{kind:"local" as const,reason:"beyond-host-refused" as const,detail:reason};};
 const say=async():Promise<void>=>{
  const queue=await ports.queue().catch(()=>undefined);
  ports.report(offloadTextsV1("beyondHost.waiting",{crate:ports.crate,queued:queue?.queued??"?",holders:queue?.holders===undefined||queue.holders.length===0?"?":queue.holders.join(", "),typical:queue?.typicalSeconds??"?",remaining:Math.ceil(remaining()/60000)}));
 };
 for(;;){
  if(aborted())return{kind:"local",reason:"cancelled",detail:""};
  if(remaining()<=0)return refuse("queue-timeout");
  const timer=setInterval(()=>{void say();},progress);
  let outcome:T;
  try{outcome=await ports.attempt(remaining());}finally{clearInterval(timer);}
  if(outcome.kind==="done")return outcome;
  if(outcome.reason==="cancelled"||aborted())return{kind:"local",reason:"cancelled",detail:""};
  if(!RETRYABLE.has(outcome.reason))return refuse(`${outcome.reason}${outcome.detail===""?"":` ${outcome.detail}`}`);
  if(remaining()<=retry)return refuse(`${outcome.reason}${outcome.detail===""?"":` ${outcome.detail}`}`);
  await say();
  await sleep(retry,ports.signal);
 }
}
