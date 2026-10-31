/** 🎟️ One request that arrived earlier in a resource queue, as its ticket file states it. */
export type QueueTicketV1=Readonly<{pid:number;mode:"shared"|"exclusive";sharedCohort:boolean;yielding:boolean;arrivedAtMs:number}>;
/** 🙋️ The request whose turn is decided; `holdsShared` says its process already holds the resource shared. */
export type QueueRequesterV1=Readonly<{pid:number;mode:"shared"|"exclusive";sharedCohort:boolean;yielding:boolean;holdsShared:boolean}>;

/** 🙇️ Whether an earlier exclusive ticket stands aside for the requester: a yielding writer never delays a request that is not itself yielding, and a writer keeps its preference over later shared requests only for `writerPreferenceMilliseconds`, so a writer that waits for long holders can neither close the queue for hours nor starve the holders' own descendants queued behind it. Other writers keep arrival order. */
export function queueTicketDefersV1(requester:QueueRequesterV1,ahead:QueueTicketV1,nowMs:number,writerPreferenceMilliseconds:number):boolean{
 if(ahead.mode!=="exclusive")return false;
 if(requester.mode==="shared")return ahead.yielding||nowMs-ahead.arrivedAtMs>=writerPreferenceMilliseconds;
 return ahead.yielding&&!requester.yielding;
}

/** ⏳️ How long a waiting writer of this resource keeps its preference over later shared requests: the longest declared prefix that matches (unbounded when the row declares no duration: a draining maintenance writer keeps its place until it is granted), else the general value (unbounded when none is declared). */
export function writerPreferenceForResourceV1(fairness:Readonly<{writerPreferenceMilliseconds?:number;writerPreferenceByPrefix?:readonly Readonly<{prefix:string;milliseconds?:number}>[]}>,resource:string):number{
 const match=[...fairness.writerPreferenceByPrefix??[]].filter(row=>resource.startsWith(row.prefix)).sort((left,right)=>right.prefix.length-left.prefix.length)[0];
 return match?match.milliseconds??Infinity:fairness.writerPreferenceMilliseconds??Infinity;
}

/** 🚦️ Whether the requester may try its kernel lease now: every earlier ticket that does not stand aside is a shared cohort ticket of a shared cohort request, a shared ticket when the requester's process or its owning ancestor already holds the resource shared (shared tickets never conflict, the owner may wait for this very request, as an Nx daemon waits behind its loader's held ticket, and a waiting writer ahead cannot hold while the owner does, so it waits for this very tree: the tree is admitted past it, otherwise holder and writer wait for each other until the writer preference ends. The pass has no time limit: a writer ahead waits for the holders of the resource, so a holder that queued behind it would close a cycle in the wait-for graph (holder waits for the writer ticket, writer waits for the holder's lease) that nothing could ever break; the writer is not starved by it because the pass only covers processes that already hold the resource, and it is granted as soon as they release. */
export function queueMayProceedV1(requester:QueueRequesterV1,ahead:readonly QueueTicketV1[],nowMs:number,writerPreferenceMilliseconds:number):boolean{
 const blocking=ahead.filter(row=>!queueTicketDefersV1(requester,row,nowMs,writerPreferenceMilliseconds));
 return blocking.length===0
  ||requester.sharedCohort&&blocking.every(row=>row.mode==="shared"&&row.sharedCohort)
  ||requester.mode==="shared"&&requester.holdsShared;
}
