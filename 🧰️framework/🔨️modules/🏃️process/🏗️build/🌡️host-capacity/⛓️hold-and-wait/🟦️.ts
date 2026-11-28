import {hostFloorMegabytesV1,hostGrowthOfV1,hostGrowthParentV1,type HostCapacityPolicyV1,type HostDecisionV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";

/** ⛓️ What one resolution of a hold-and-wait looks at: the live grants, the queue, what the decision of this very round said about every request, the memory that is spare right now (the headroom of the evaluation) and the clock. */
export type HostHoldAndWaitFactsV1=Readonly<{ledger:readonly HostGrantV1[];waiters:readonly HostWaiterV1[];decisions:readonly Pick<HostDecisionV1,"id"|"verdict">[];spareMegabytes:number;nowMs:number}>;

/** 🩺️ How a hold-and-wait ends: `none` (some grant can still make progress) or `preempt` (the `victims` release their grants, which frees `releasedMegabytes` for the waiting growths of the others; `holders` names every grant that waited). A growth that declared a reduced floor never waits this long: the reduced start of `decideHostCapacityV1` runs it below its estimate as soon as the spare memory carries the floor. */
export type HostHoldAndWaitVerdictV1=Readonly<{kind:"none"}>|Readonly<{kind:"preempt";victims:readonly string[];releasedMegabytes:number;holders:readonly string[];waitedMilliseconds:number}>;

const none:HostHoldAndWaitVerdictV1=Object.freeze({kind:"none"});
const needOf=(request:HostRequestV1):number=>request.minimumMegabytes??request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes;
const priorityOf=(grant:HostGrantV1):number=>(grant.priority??(grant.operation==="command"?"batch":"interactive"))==="batch"?0:1;
const heldOf=(policy:HostCapacityPolicyV1,grant:HostGrantV1,nowMs:number):number=>grant.observedMegabytes===null?grant.reservedMegabytes:nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds)||grant.retainReservation===true?Math.max(grant.reservedMegabytes,grant.observedMegabytes):grant.observedMegabytes;

/** 🪢️ Resolves a hold-and-wait: every running grant (one that no other grant owns) waits for the growth of its own work (requests labelled `unit-growth:<grant id>`) for at least `wait.deadlockMilliseconds`, and this round's decision admitted nothing, so nothing is left that could return memory and the queue would wait for ever. Grants are preempted, lowest priority first, then the youngest (it lost the least running time; its finished units stay cached and it is queued again), then by id, and only as many as it takes for a waiting growth of a grant that stays, or any waiting request, to fit what is spare and what they release. A pure function of the facts: the gate applies it under the ledger lock and tells the victims through the preempt marker. Without that fit (nothing could ever be admitted) it answers `none`: the unreachable rule refuses the request.
 * @see ../🧪️tests/⛓️hold-and-wait/🟦️.ts
 * @see ../🧪️tests/⛓️hold-and-wait/🐍️oracle/🐍️.py */
export function resolveHostHoldAndWaitV1(policy:HostCapacityPolicyV1,facts:HostHoldAndWaitFactsV1):HostHoldAndWaitVerdictV1{
 const limit=policy.wait.deadlockMilliseconds;
 if(limit===undefined||facts.decisions.some(row=>row.verdict==="admit"))return none;
 const roots=facts.ledger.filter(grant=>grant.lentSinceMs===undefined&&hostGrowthParentV1(grant.label)===undefined);
 const growths=facts.waiters.filter(waiter=>hostGrowthOfV1(facts.ledger,waiter)!==undefined).sort((left,right)=>left.arrivedAtMs-right.arrivedAtMs||(left.request.id<right.request.id?-1:1));
 const stalled=new Set(growths.filter(waiter=>facts.nowMs-waiter.arrivedAtMs>=limit).map(waiter=>hostGrowthOfV1(facts.ledger,waiter)!));
 if(roots.length===0||roots.some(grant=>!stalled.has(grant.id)))return none;
 const spare=Math.max(0,facts.spareMegabytes);
 const rank=[...roots].sort((left,right)=>priorityOf(left)-priorityOf(right)||right.admittedAtMs-left.admittedAtMs||(left.id<right.id?-1:1));
 const victims:string[]=[];let released=0;
 for(const grant of rank){
  victims.push(grant.id);released+=facts.ledger.reduce((sum,row)=>sum+(row.id===grant.id||hostGrowthParentV1(row.label)===grant.id?heldOf(policy,row,facts.nowMs):0),0);
  if(facts.waiters.some(waiter=>!victims.includes(hostGrowthParentV1(waiter.request.label)??"")&&needOf(waiter.request)<=spare+released))
   return Object.freeze({kind:"preempt",victims:Object.freeze(victims),releasedMegabytes:released,holders:Object.freeze(roots.map(row=>row.label??row.operation)),waitedMilliseconds:Math.max(...growths.map(waiter=>facts.nowMs-waiter.arrivedAtMs))});
 }
 return none;
}

/** 🧮️ What this host could give out even if every grant of ours ended right now: the memory above the floor that the host holds free or that our grants hold, within the authored budget share of the machine, and the part of the machine that other applications hold. */
export function hostPossibleMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes"|"availableMegabytes">,ledger:readonly Pick<HostGrantV1,"observedMegabytes"|"reservedMegabytes"|"lentSinceMs">[]):Readonly<{floorMegabytes:number;possibleMegabytes:number;foreignMegabytes:number}>{
 const floor=hostFloorMegabytesV1(policy,host),ours=ledger.reduce((sum,grant)=>sum+(grant.lentSinceMs===undefined?grant.observedMegabytes??grant.reservedMegabytes:0),0),drained=Math.min(host.availableMegabytes+ours,host.totalMegabytes);
 return Object.freeze({floorMegabytes:floor,possibleMegabytes:Math.max(0,Math.min(drained-floor,Math.floor(Math.max(0,host.totalMegabytes-floor)*policy.memory.budgetBasisPoints/10000))),foreignMegabytes:Math.max(0,host.totalMegabytes-drained)});
}

/** 🎯️ The least memory a request must get to run at all: its reduced floor, else its declared minimum, never below the policy minimum a reduced start needs. A request without a declared minimum can always start reduced, so only a host below the policy minimum is out of reach for it. */
export function hostLeastMegabytesV1(policy:HostCapacityPolicyV1,request:Pick<HostRequestV1,"minimumMegabytes"|"floorMegabytes">):number{
 return Math.max(policy.estimates.minimumMegabytes,request.floorMegabytes??request.minimumMegabytes??0);
}

/** 🚧️ Starts, keeps or ends the timer of an unreachable request: the moment since which its least need exceeded everything the host could give out, `undefined` once it fits again, so memory that other applications give back for a moment restarts the count. */
export function hostUnreachableSinceV1(previous:number|undefined,facts:Readonly<{leastMegabytes:number;possibleMegabytes:number;nowMs:number}>):number|undefined{
 return facts.leastMegabytes>facts.possibleMegabytes?previous??facts.nowMs:undefined;
}

/** 🚫️ Whether an unreachable request is refused now: its need exceeded everything the host could give out for `wait.unreachableMilliseconds`, so waiting cannot help and the stage that asked fails with an actionable message instead of queueing for ever. A policy without the field never refuses.
 * @see ../🧪️tests/⛓️hold-and-wait/🟦️.ts */
export function hostRequestUnreachableV1(policy:HostCapacityPolicyV1,since:number|undefined,nowMs:number):boolean{
 return since!==undefined&&policy.wait.unreachableMilliseconds!==undefined&&nowMs-since>=policy.wait.unreachableMilliseconds;
}
